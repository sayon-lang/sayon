//! Functions called by compiled Sayon programs.
//!
//! Conventions:
//! - Strings are `SayonStr` values; `cap == 0` marks static or empty text
//!   that is never freed.
//! - Arrays are `SayonArray` values whose elements the generated code reads
//!   and writes directly; this module only manages their storage.
//! - Dictionaries and errors are heap objects referred to by pointer. An
//!   error slot is a `*mut *mut SayonError`; a null pointer means empty.
//! - Fallible functions return 0 on success and 1 after storing an error.
//! - Integer type codes are the indexes of `text::INTEGER_TYPE_NAMES`.
//!
//! Every function here is called only by generated code that upholds these
//! conventions, so the individual functions do not repeat them.

#![allow(clippy::missing_safety_doc)]

use crate::scopes::{self, Room};
use crate::text;
use std::alloc::{Layout, alloc, dealloc, realloc};
use std::ffi::{CStr, c_char};
use std::io::Write;
use std::sync::atomic::{AtomicI64, AtomicU8, Ordering};

/// Heap objects the runtime has created and not yet released: owned string
/// data, array storage, dictionaries, errors, text builders, and closure
/// environments. With `SAYON_DEBUG_ALLOC` set, a program reports the count
/// when it exits; a correct program reports zero.
static LIVE: AtomicI64 = AtomicI64::new(0);
/// Whether `LIVE` counts: only with `SAYON_DEBUG_ALLOC`, since counting
/// from several threads costs every allocation a locked instruction. It is
/// 0 until the first count decides, then 1 for no and 2 for yes. Deciding
/// then, not when `main` starts, counts what comes before: a web page's
/// `main` allocates its own frame before its first line.
static COUNTING_LIVE: AtomicU8 = AtomicU8::new(0);

#[inline]
fn counting_live() -> bool {
    match COUNTING_LIVE.load(Ordering::Relaxed) {
        1 => false,
        2 => true,
        _ => decide_counting_live(),
    }
}

#[cold]
fn decide_counting_live() -> bool {
    let counting = crate::console::option("SAYON_DEBUG_ALLOC");
    COUNTING_LIVE.store(if counting { 2 } else { 1 }, Ordering::Relaxed);
    counting
}

#[inline]
pub(crate) fn allocated() {
    if counting_live() {
        LIVE.fetch_add(1, Ordering::Relaxed);
    }
}

#[inline]
pub(crate) fn released() {
    if counting_live() {
        LIVE.fetch_sub(1, Ordering::Relaxed);
    }
}

/// A string. Text up to `INLINE` bytes long lies inside the structure
/// itself, in its bytes but the last, which holds the text's length with
/// its top bit set. Longer text lies at `ptr`, in a buffer of `cap` bytes the
/// string owns, or with `cap` 0 in the program's constants. A capacity never
/// has its top bit set, so the last byte tells the two apart.
#[repr(C)]
pub struct SayonStr {
    pub ptr: *const u8,
    pub len: i64,
    pub cap: i64,
}

/// The most text a string keeps inside itself. Every target is
/// little-endian, so a capacity's top bit is the structure's last bit. On
/// WebAssembly an address takes 4 of the first 8 bytes and leaves a gap that
/// copying the structure skips, so strings there keep their text outside.
#[cfg(target_pointer_width = "64")]
pub const INLINE: usize = 23;
#[cfg(not(target_pointer_width = "64"))]
pub const INLINE: usize = 0;

const _: () = assert!(std::mem::size_of::<SayonStr>() == 24);

impl SayonStr {
    const EMPTY: SayonStr = SayonStr {
        ptr: std::ptr::null(),
        len: 0,
        cap: 0,
    };

    /// Whether the text lies inside the structure.
    fn is_inline(&self) -> bool {
        INLINE > 0 && self.cap < 0
    }

    /// The text's bytes, wherever they lie.
    fn bytes(&self) -> &[u8] {
        // SAFETY: the structure is a valid string: inside it, its length's
        // worth of bytes; otherwise `len` bytes at `ptr`.
        unsafe {
            if self.is_inline() {
                let length = ((self.cap as u64) >> 56) as usize & 0x7f;
                std::slice::from_raw_parts(self as *const SayonStr as *const u8, length)
            } else if self.len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.ptr, self.len as usize)
            }
        }
    }

    /// A string holding `text` inside itself. `text` is at most `INLINE`
    /// bytes, and none is the empty string.
    fn inline(text: &[u8]) -> SayonStr {
        if text.is_empty() || text.len() > INLINE {
            return SayonStr::EMPTY;
        }
        let mut bytes = [0u8; 24];
        bytes[..text.len()].copy_from_slice(text);
        bytes[23] = 0x80 | text.len() as u8;
        // SAFETY: a string is 24 bytes of plain data, which any bytes are.
        unsafe { std::mem::transmute::<[u8; 24], SayonStr>(bytes) }
    }
}

/// A string holding a copy of `text`: inside itself when it fits.
fn copied(text: &str) -> SayonStr {
    if text.len() <= INLINE {
        SayonStr::inline(text.as_bytes())
    } else {
        owned(text.to_string())
    }
}

#[repr(C)]
pub struct SayonArray {
    pub ptr: *mut u8,
    pub len: i64,
    pub cap: i64,
}

#[repr(C)]
pub struct SayonRange {
    pub start: i64,
    pub end: i64,
    pub step: i64,
}

pub struct SayonError {
    pub code: String,
    pub message: String,
    pub location: Option<String>,
}

/// An insertion-ordered dictionary: keys and values in insertion order, and
/// an open-addressing table of positions found by the keys' hashes.
///
/// Removing an entry leaves its place empty, with the hash `REMOVED`, so
/// that the entries after it keep their positions and removing costs no
/// more than adding. The empty places are closed up once they outnumber
/// the entries, or when the table grows: both change the dictionary, so no
/// position found before is still in use. A loop over the entries steps
/// over the empty places (`sayon_dict_next`).
pub struct SayonDict {
    ops: *const KeyOps,
    /// Key storage in 8-byte words, `key_stride` words per key.
    keys: Vec<u64>,
    key_stride: usize,
    hashes: Vec<u64>,
    /// Positions plus one, or 0 for a free slot; the length is a power of two.
    /// A removed entry's slot keeps its position, so lookups go on past it.
    table: Vec<u32>,
    /// Value storage in 8-byte words, `stride` words per value.
    values: Vec<u64>,
    stride: usize,
    value_size: usize,
    /// Where the last key found is, or `usize::MAX`. A lookup tries it
    /// first, so that `haskey(d, k)` and then `d[k]` hash `k` once.
    last: std::cell::Cell<usize>,
    /// How many places hold a removed entry.
    removed: usize,
}

/// The hash kept in the place of a removed entry, which no key's hash is.
const REMOVED: u64 = u64::MAX;

/// Text being put together, as `sayon_builder_*` append to it. Compiled code
/// keeps one on its own stack, the size of a string, from
/// `sayon_builder_start` to `sayon_builder_end`.
/// The builder is a string itself, so short text stays inside it, and the
/// string it ends with is the builder's own.
#[repr(C)]
pub struct Builder(SayonStr);

const _: () = assert!(std::mem::size_of::<Builder>() == std::mem::size_of::<SayonStr>());

impl Builder {
    fn push_str(&mut self, text: &str) {
        // SAFETY: the builder holds a valid string, and `text` is not in it.
        unsafe { add_bytes(&mut self.0, text.as_bytes()) }
    }

    fn push(&mut self, ch: char) {
        self.push_str(ch.encode_utf8(&mut [0; 4]));
    }
}

impl std::fmt::Write for Builder {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        self.push_str(text);
        Ok(())
    }
}

pub(crate) type DropFn = Option<unsafe extern "C" fn(*mut u8)>;
type CloneFn = Option<unsafe extern "C" fn(*const u8, *mut u8)>;
type EqFn = unsafe extern "C" fn(*const u8, *const u8) -> bool;

const ELEMENT_ALIGN: usize = 8;

// ----- helpers ---------------------------------------------------------------

unsafe fn as_str<'a>(s: *const SayonStr) -> &'a str {
    // SAFETY: callers pass a valid SayonStr holding UTF-8 text, which stays
    // where it is while they use the text.
    unsafe { std::str::from_utf8_unchecked((*s).bytes()) }
}

unsafe fn c_str<'a>(text: *const c_char) -> &'a str {
    if text.is_null() {
        return "";
    }
    // SAFETY: the generated code passes NUL-terminated UTF-8 constants.
    unsafe { CStr::from_ptr(text).to_str().unwrap_or("") }
}

/// The string of `text`: inside the string when it fits, and otherwise the
/// string takes over `text`'s buffer. A little room to spare stays, as
/// moving the text to a buffer of its exact size would cost more than the
/// bytes saved; more than that is given back.
fn owned(mut text: String) -> SayonStr {
    if text.len() <= INLINE {
        return SayonStr::inline(text.as_bytes());
    }
    let spare = text.capacity() - text.len();
    if spare > 16 && spare > text.len() / 4 {
        text.shrink_to_fit();
    }
    let mut text = std::mem::ManuallyDrop::new(text);
    allocated();
    SayonStr {
        ptr: text.as_mut_ptr(),
        len: text.len() as i64,
        cap: text.capacity() as i64,
    }
}

/// One-byte strings for every ASCII character, so that most characters
/// taken from a string need no allocation.
static ASCII: [u8; 128] = {
    let mut table = [0u8; 128];
    let mut index = 0;
    while index < 128 {
        table[index] = index as u8;
        index += 1;
    }
    table
};

/// A string holding one character.
fn character(ch: char) -> SayonStr {
    if ch.is_ascii() {
        SayonStr {
            ptr: &ASCII[ch as usize],
            len: 1,
            cap: 0,
        }
    } else {
        copied(ch.encode_utf8(&mut [0; 4]))
    }
}

pub(crate) unsafe fn set_error(
    slot: *mut *mut SayonError,
    code: &str,
    message: String,
    location: *const c_char,
) -> i32 {
    // SAFETY: `slot` points to the caller's error slot.
    unsafe {
        if !(*slot).is_null() {
            drop(Box::from_raw(*slot));
            released();
        }
        let location = if location.is_null() {
            None
        } else {
            Some(c_str(location).to_string())
        };
        allocated();
        *slot = Box::into_raw(Box::new(SayonError {
            code: code.to_string(),
            message,
            location,
        }));
    }
    1
}

fn type_name(code: u32) -> &'static str {
    text::INTEGER_TYPE_NAMES
        .get(code as usize)
        .copied()
        .unwrap_or("integer")
}

fn widen(value: i64, code: u32) -> i128 {
    if code >= 4 {
        value as u64 as i128
    } else {
        value as i128
    }
}

// ----- strings ---------------------------------------------------------------------

/// Releases an owned string and leaves it empty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_drop(s: *mut SayonStr) {
    // SAFETY: `s` is a valid string; owned data came from `owned` or a
    // clone, a buffer of `cap` bytes, of which only the first `len` need
    // hold text.
    unsafe {
        let value = &mut *s;
        if value.cap > 0 {
            std::alloc::dealloc(
                value.ptr as *mut u8,
                std::alloc::Layout::from_size_align_unchecked(value.cap as usize, 1),
            );
            released();
        }
        *value = SayonStr {
            ptr: std::ptr::null(),
            len: 0,
            cap: 0,
        };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_clone(src: *const SayonStr, dst: *mut SayonStr) {
    // SAFETY: both pointers are valid. Text in the program's constants can
    // be shared, and text inside a string is copied with it.
    unsafe {
        let source = &*src;
        *dst = if source.cap <= 0 {
            SayonStr {
                ptr: source.ptr,
                len: source.len,
                cap: source.cap,
            }
        } else {
            copied(as_str(src))
        };
    }
}

/// Two texts one after the other, in a string of its own: inside it when the
/// whole fits, or in a buffer of `room` bytes, at least as many as the text.
fn joined(first: &[u8], second: &[u8], room: usize) -> SayonStr {
    let total = first.len() + second.len();
    if total <= INLINE {
        let mut bytes = [0u8; 24];
        bytes[..first.len()].copy_from_slice(first);
        bytes[first.len()..total].copy_from_slice(second);
        return SayonStr::inline(&bytes[..total]);
    }
    let mut buffer: Vec<u8> = Vec::with_capacity(room.max(total));
    buffer.extend_from_slice(first);
    buffer.extend_from_slice(second);
    let mut buffer = std::mem::ManuallyDrop::new(buffer);
    allocated();
    SayonStr {
        ptr: buffer.as_mut_ptr(),
        len: buffer.len() as i64,
        cap: buffer.capacity() as i64,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_concat(
    a: *const SayonStr,
    b: *const SayonStr,
    out: *mut SayonStr,
) {
    // SAFETY: valid strings in, `out` is writable.
    unsafe {
        let (a, b) = ((*a).bytes(), (*b).bytes());
        *out = joined(a, b, 0);
    }
}

/// Adds the text of `more` to the end of `text`, inside the string while the
/// whole fits, then in a buffer of its own, which grows twice as big when it
/// runs out of room, so that adding piece after piece takes time in
/// proportion to the text's length. `more` may be `text` itself.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_append(text: *mut SayonStr, more: *const SayonStr) {
    // SAFETY: valid strings; an owned one's buffer holds `cap` bytes.
    unsafe {
        if std::ptr::eq(text, more) {
            // The same text twice: take a copy of it first.
            let twice = (*more).bytes().to_vec();
            let doubled = SayonStr {
                ptr: twice.as_ptr(),
                len: twice.len() as i64,
                cap: 0,
            };
            return sayon_str_append(text, &doubled);
        }
        add_bytes(&mut *text, (*more).bytes());
    }
}

/// Adds `addition` to the end of `target`'s text, as `sayon_str_append`
/// does. `addition` is not in `target`.
unsafe fn add_bytes(target: &mut SayonStr, addition: &[u8]) {
    // SAFETY: a valid string; an owned one's buffer holds `cap` bytes.
    unsafe {
        let count = addition.len();
        if count == 0 {
            return;
        }
        if target.cap <= 0 {
            // Inside the string, among the program's constants, or none: the
            // whole goes inside the string when it fits, and otherwise in a
            // buffer with room to grow by half again.
            let room = (target.bytes().len() + count) * 3 / 2;
            let whole = joined(target.bytes(), addition, room);
            *target = whole;
            return;
        }
        let length = target.len as usize;
        let source = addition.as_ptr();
        let mut buffer = target.ptr as *mut u8;
        let capacity = target.cap as usize;
        let needed = length + count;
        if needed > capacity {
            let grown = needed.max(capacity * 2);
            buffer = std::alloc::realloc(
                buffer,
                std::alloc::Layout::from_size_align_unchecked(capacity, 1),
                grown,
            );
            if buffer.is_null() {
                std::alloc::handle_alloc_error(std::alloc::Layout::from_size_align_unchecked(
                    grown, 1,
                ));
            }
            target.ptr = buffer;
            target.cap = grown as i64;
        }
        // `more` is another string, so its text is not in this buffer.
        std::ptr::copy_nonoverlapping(source, buffer.add(length), count);
        target.len = needed as i64;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_eq(a: *const SayonStr, b: *const SayonStr) -> bool {
    // SAFETY: valid strings.
    unsafe { as_str(a) == as_str(b) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_cmp(a: *const SayonStr, b: *const SayonStr) -> i32 {
    // SAFETY: valid strings.
    unsafe { as_str(a).cmp(as_str(b)) as i32 }
}

/// The character at a position, counted in characters, into `out`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_char_at(
    err: *mut *mut SayonError,
    s: *const SayonStr,
    position: i64,
    out: *mut SayonStr,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid string, output place, and error slot.
    unsafe {
        let text = as_str(s);
        let found = usize::try_from(position)
            .ok()
            .and_then(|position| text.chars().nth(position));
        match found {
            Some(ch) => {
                *out = character(ch);
                0
            }
            None => {
                let message = text::position_error(position as i128, text.chars().count());
                set_error(err, "BoundsError", message, location)
            }
        }
    }
}

/// The character starting at byte `*offset` into `out`, advancing the offset.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_next_char(
    s: *const SayonStr,
    offset: *mut i64,
    out: *mut SayonStr,
) {
    // SAFETY: valid string and places; the offset is a character boundary
    // before the end of the string.
    unsafe {
        let text = as_str(s);
        let ch = text[*offset as usize..]
            .chars()
            .next()
            .expect("a character");
        *offset += ch.len_utf8() as i64;
        *out = character(ch);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_chars(s: *const SayonStr) -> i64 {
    // SAFETY: valid string.
    unsafe { as_str(s).chars().count() as i64 }
}

// ----- building text -------------------------------------------------------------------

/// Starts a builder in the memory at `b`. `capacity` is the caller's guess
/// of the text's length: text too long to lie inside a string gets a buffer
/// of that size at once, which saves growing it a step at a time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_start(b: *mut Builder, capacity: i64) {
    // SAFETY: `b` is room for a builder, which nothing holds yet.
    unsafe {
        let capacity = capacity.max(0) as usize;
        let text = if capacity > INLINE {
            let mut buffer = std::mem::ManuallyDrop::new(Vec::<u8>::with_capacity(capacity));
            allocated();
            SayonStr {
                ptr: buffer.as_mut_ptr(),
                len: 0,
                cap: buffer.capacity() as i64,
            }
        } else {
            SayonStr::EMPTY
        };
        b.write(Builder(text));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_bytes(b: *mut Builder, ptr: *const u8, len: i64) {
    // SAFETY: `ptr` holds `len` bytes of UTF-8 text from a constant.
    unsafe {
        if len > 0 {
            let bytes = std::slice::from_raw_parts(ptr, len as usize);
            (*b).push_str(std::str::from_utf8_unchecked(bytes));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_str(b: *mut Builder, s: *const SayonStr, quoted: bool) {
    // SAFETY: valid builder and string.
    unsafe {
        if quoted {
            text::quote(as_str(s), &mut *b);
        } else {
            (*b).push_str(as_str(s));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_int(b: *mut Builder, value: i64, code: u32) {
    // SAFETY: valid builder.
    unsafe {
        use std::fmt::Write as _;
        let _ = write!(*b, "{}", widen(value, code));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_f64(b: *mut Builder, value: f64) {
    // SAFETY: valid builder.
    unsafe { (*b).push_str(&text::format_f64(value)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_f32(b: *mut Builder, value: f32) {
    // SAFETY: valid builder.
    unsafe { (*b).push_str(&text::format_f32(value)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_bool(b: *mut Builder, value: bool) {
    // SAFETY: valid builder.
    unsafe { (*b).push_str(if value { "true" } else { "false" }) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_error(
    b: *mut Builder,
    e: *const *mut SayonError,
    nested: bool,
) {
    // SAFETY: valid builder and error slot.
    unsafe {
        let error = &**e;
        if nested {
            (*b).push_str("Error(");
            text::quote(&error.message, &mut *b);
            (*b).push(')');
        } else {
            (*b).push_str(&error.message);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_range(b: *mut Builder, r: *const SayonRange) {
    // SAFETY: valid builder and range.
    unsafe {
        use std::fmt::Write as _;
        let range = &*r;
        if range.step == 1 {
            let _ = write!(*b, "range({}, {})", range.start, range.end);
        } else {
            let _ = write!(
                *b,
                "range({}, {}, {})",
                range.start,
                range.end,
                range.step
            );
        }
    }
}

/// Ends a builder, giving its text as a string. Short text moves inside the
/// string, and a buffer with much room to spare shrinks to its text.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_end(b: *mut Builder, out: *mut SayonStr) {
    // SAFETY: `b` was started by `sayon_builder_start` and is not used again.
    unsafe {
        let mut text = b.read().0;
        if text.cap > 0 {
            let (length, capacity) = (text.len as usize, text.cap as usize);
            if length <= INLINE {
                let short = SayonStr::inline(text.bytes());
                sayon_str_drop(&mut text);
                text = short;
            } else if capacity - length > 16 && capacity - length > length / 4 {
                let buffer = std::alloc::realloc(
                    text.ptr as *mut u8,
                    std::alloc::Layout::from_size_align_unchecked(capacity, 1),
                    length,
                );
                if buffer.is_null() {
                    std::alloc::handle_alloc_error(std::alloc::Layout::from_size_align_unchecked(
                        length, 1,
                    ));
                }
                text.ptr = buffer;
                text.cap = length as i64;
            }
        }
        *out = text;
    }
}

// ----- output ----------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_print(
    err: *mut *mut SayonError,
    s: *const SayonStr,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid string and error slot.
    unsafe {
        let mut stdout = crate::console::stdout().lock();
        let result = stdout
            .write_all(as_str(s).as_bytes())
            .and_then(|_| stdout.write_all(b"\n"));
        match result {
            Ok(()) => 0,
            Err(error) => set_error(
                err,
                "IOError",
                format!("could not write output: {error}"),
                location,
            ),
        }
    }
}

// ----- closure environments ---------------------------------------------------------------

fn block_layout(size: i64) -> Layout {
    Layout::from_size_align(size.max(1) as usize, ELEMENT_ALIGN).expect("block layout")
}

/// Allocates `size` bytes for a closure environment.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_alloc(size: i64) -> *mut u8 {
    let layout = block_layout(size);
    // SAFETY: the layout has a nonzero size.
    let pointer = unsafe { alloc(layout) };
    if pointer.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    allocated();
    pointer
}

/// Frees a block from `sayon_alloc` of the same size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_free(pointer: *mut u8, size: i64) {
    released();
    // SAFETY: `pointer` came from `sayon_alloc(size)`.
    unsafe { dealloc(pointer, block_layout(size)) }
}

// ----- arrays ------------------------------------------------------------------------------

fn array_layout(capacity: usize, element_size: usize) -> Layout {
    Layout::from_size_align((capacity * element_size).max(1), ELEMENT_ALIGN).expect("array layout")
}

/// Gives an array room for exactly `count` elements, as an array written out
/// in full or a clone holds from the start. Growing with `push` instead
/// leaves room to spare (see `sayon_array_reserve`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_array_exact(a: *mut SayonArray, count: i64, element_size: i64) {
    // SAFETY: `a` is a valid array whose storage came from this module.
    unsafe { exact(&mut *a, count, element_size, None) }
}

/**
As `sayon_array_exact`, for an array the compiler keeps on data stack `stack`
(see `scopes`): storage it starts goes there when there is room.
*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_array_exact_on(
    a: *mut SayonArray,
    count: i64,
    element_size: i64,
    stack: i32,
) {
    /* SAFETY: `a` is a valid array whose storage came from this module or a data stack. */
    unsafe { exact(&mut *a, count, element_size, Some(stack as usize)) }
}

/** Gives `array` room for exactly `count` elements, starting its storage on `stack` if one is named. */
unsafe fn exact(array: &mut SayonArray, count: i64, element_size: i64, stack: Option<usize>) {
    if count <= array.cap {
        return;
    }
    if array.cap > 0 {
        /* SAFETY: the caller's valid array. */
        return unsafe { reserve(array, count - array.len, element_size, stack) };
    }
    let size = element_size.max(1) as usize;
    if let Some(stack) = stack
        && let Some(elements) = scopes::place(count as usize, size, stack)
    {
        array.ptr = elements;
        array.cap = count;
        return;
    }
    let layout = array_layout(count as usize, size);
    allocated();
    /* SAFETY: a layout of nonzero size. */
    let pointer = unsafe { alloc(layout) };
    if pointer.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    array.ptr = pointer;
    array.cap = count;
}

/// Ensures room for `additional` more elements.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_array_reserve(
    a: *mut SayonArray,
    additional: i64,
    element_size: i64,
) {
    // SAFETY: `a` is a valid array whose storage came from this module.
    unsafe { reserve(&mut *a, additional, element_size, None) }
}

/**
As `sayon_array_reserve`, for an array the compiler keeps on data stack
`stack` (see `scopes`): storage it starts goes there when there is room.
*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_array_reserve_on(
    a: *mut SayonArray,
    additional: i64,
    element_size: i64,
    stack: i32,
) {
    /* SAFETY: `a` is a valid array whose storage came from this module or a data stack. */
    unsafe { reserve(&mut *a, additional, element_size, Some(stack as usize)) }
}

/**
Gives `array` room for `additional` more elements. Storage it starts goes on
`stack` if one is named; storage on a stack grows there when `scopes` allows
it and otherwise moves to the heap; storage on the heap grows on the heap.
*/
unsafe fn reserve(
    array: &mut SayonArray,
    additional: i64,
    element_size: i64,
    stack: Option<usize>,
) {
    let needed = array.len as usize + additional as usize;
    if needed <= array.cap as usize {
        return;
    }
    let size = element_size.max(1) as usize;
    let capacity = needed.max(array.cap as usize * 2).max(4);
    let new_layout = array_layout(capacity, size);
    let pointer = if array.ptr.is_null() || array.cap == 0 {
        if let Some(stack) = stack
            && let Some(elements) = scopes::place(capacity, size, stack)
        {
            array.ptr = elements;
            array.cap = capacity as i64;
            return;
        }
        allocated();
        /* SAFETY: a layout of nonzero size. */
        unsafe { alloc(new_layout) }
    } else {
        match scopes::grow(array.ptr, array.cap as usize, capacity, size) {
            Room::Grown => {
                array.cap = capacity as i64;
                return;
            }
            Room::Moving => {
                allocated();
                /* SAFETY: a layout of nonzero size. */
                let moved = unsafe { alloc(new_layout) };
                if !moved.is_null() {
                    /*
                    SAFETY: the array's elements, into new storage with room for them. Their
                    old place stays on its stack until its scope ends, and is never read again.
                    */
                    unsafe {
                        std::ptr::copy_nonoverlapping(array.ptr, moved, array.len as usize * size)
                    };
                }
                moved
            }
            /* SAFETY: storage from this module's heap, with its layout. */
            Room::Heap => unsafe {
                realloc(
                    array.ptr,
                    array_layout(array.cap as usize, size),
                    new_layout.size(),
                )
            },
        }
    };
    if pointer.is_null() {
        std::alloc::handle_alloc_error(new_layout);
    }
    array.ptr = pointer;
    array.cap = capacity as i64;
}

/// Releases array storage (elements must already be destroyed) and leaves it empty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_array_free(a: *mut SayonArray, element_size: i64) {
    // SAFETY: valid array.
    unsafe {
        let array = &mut *a;
        if !array.ptr.is_null() && array.cap > 0 && !scopes::holds(array.ptr) {
            released();
            dealloc(
                array.ptr,
                array_layout(array.cap as usize, element_size.max(1) as usize),
            );
        }
        *array = SayonArray {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        };
    }
}

/// Checks the bounds of a view of an array or view of `length` elements.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_check_view(
    err: *mut *mut SayonError,
    start: i64,
    end: i64,
    length: i64,
    location: *const c_char,
) -> i32 {
    if 0 <= start && start <= end && end <= length {
        return 0;
    }
    let message = text::view_bounds_message(start as i128, end as i128, length as usize);
    // SAFETY: valid error slot.
    unsafe { set_error(err, "BoundsError", message, location) }
}

/// Fails unless a view of `length` elements has `count` to split off.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_check_split(
    err: *mut *mut SayonError,
    count: i64,
    length: i64,
    location: *const c_char,
) -> i32 {
    if 0 <= count && count <= length {
        return 0;
    }
    let message = text::split_bounds_message(count as i128, length as usize);
    // SAFETY: valid error slot.
    unsafe { set_error(err, "BoundsError", message, location) }
}

/// Exchanges two elements, or fails when either position is outside the
/// array.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_array_swap(
    err: *mut *mut SayonError,
    a: *mut SayonArray,
    first: i64,
    second: i64,
    element_size: i64,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid array and error slot.
    unsafe {
        let array = &mut *a;
        for index in [first, second] {
            if index < 0 || index >= array.len {
                return sayon_check_index(err, index, array.len, location);
            }
        }
        if first != second {
            let size = element_size.max(0) as usize;
            std::ptr::swap_nonoverlapping(
                array.ptr.add(first as usize * size),
                array.ptr.add(second as usize * size),
                size,
            );
        }
        0
    }
}

/// Moves the element at `index` into `out` and the later elements down by
/// one, or fails outside the array.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_array_remove(
    err: *mut *mut SayonError,
    a: *mut SayonArray,
    index: i64,
    element_size: i64,
    out: *mut u8,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid array, output of `element_size` bytes, and error slot.
    unsafe {
        let array = &mut *a;
        if index < 0 || index >= array.len {
            return sayon_check_index(err, index, array.len, location);
        }
        let size = element_size.max(0) as usize;
        let start = array.ptr.add(index as usize * size);
        std::ptr::copy_nonoverlapping(start, out, size);
        let later = (array.len - index - 1) as usize * size;
        std::ptr::copy(start.add(size), start, later);
        array.len -= 1;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_check_index(
    err: *mut *mut SayonError,
    index: i64,
    length: i64,
    location: *const c_char,
) -> i32 {
    if index >= 0 && index < length {
        return 0;
    }
    let message = if length == 0 {
        format!("index {index} is out of bounds: the array is empty")
    } else {
        format!("index {index} is out of bounds for an array of length {length}")
    };
    // SAFETY: valid error slot.
    unsafe { set_error(err, "BoundsError", message, location) }
}

// ----- dictionaries ------------------------------------------------------------------------

/// The generated helpers a dictionary uses for its key type.
#[repr(C)]
pub struct KeyOps {
    pub size: i64,
    pub hash: unsafe extern "C" fn(*const u8) -> i64,
    pub eq: EqFn,
    /// `None` for keys that are copied bit for bit.
    pub clone: CloneFn,
    /// `None` for keys that own nothing.
    pub drop: DropFn,
    pub display: unsafe extern "C" fn(*mut Builder, *const u8, bool),
}

/// Spreads the bits of a key hash over the table.
fn mix(hash: i64) -> u64 {
    let mut x = hash as u64;
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

impl SayonDict {
    /// The entries the dictionary holds.
    fn len(&self) -> usize {
        self.hashes.len() - self.removed
    }

    /// The places for entries, those of removed entries among them.
    fn slots(&self) -> usize {
        self.hashes.len()
    }

    fn holds(&self, position: usize) -> bool {
        self.hashes[position] != REMOVED
    }

    /// Moves the entries down over the places of removed ones, keeping
    /// their order, and finds each a slot in the table again.
    fn close_up(&mut self) {
        let (key_stride, stride) = (self.key_stride, self.stride);
        let mut kept = 0;
        for position in 0..self.slots() {
            if !self.holds(position) {
                continue;
            }
            if kept != position {
                // The entry's bytes move; it is not copied, so nothing is
                // destroyed or kept twice.
                self.keys
                    .copy_within(position * key_stride..(position + 1) * key_stride, kept * key_stride);
                self.values
                    .copy_within(position * stride..(position + 1) * stride, kept * stride);
                self.hashes[kept] = self.hashes[position];
            }
            kept += 1;
        }
        self.keys.truncate(kept * key_stride);
        self.values.truncate(kept * stride);
        self.hashes.truncate(kept);
        self.removed = 0;
        self.last.set(usize::MAX);
        self.table.iter_mut().for_each(|slot| *slot = 0);
        for position in 0..kept {
            self.place(position);
        }
    }

    fn key(&self, position: usize) -> *const u8 {
        // SAFETY: the position is below the length.
        unsafe { self.keys.as_ptr().add(position * self.key_stride) as *const u8 }
    }

    fn value(&mut self, position: usize) -> *mut u8 {
        // SAFETY: the position is below the length.
        unsafe { self.values.as_mut_ptr().add(position * self.stride) as *mut u8 }
    }

    unsafe fn hash_of(&self, key: *const u8) -> u64 {
        // SAFETY: the key is a valid value of the key type.
        let hash = unsafe { mix(((*self.ops).hash)(key)) };
        // `REMOVED` marks the place of a removed entry.
        hash.min(REMOVED - 1)
    }

    unsafe fn find(&self, key: *const u8) -> Option<usize> {
        if self.table.is_empty() {
            return None;
        }
        // SAFETY: valid key; the table holds positions of stored keys.
        unsafe {
            // The key found last, tried first. Keys are unique, so an equal
            // one is the entry asked for, wherever entries have moved since.
            let last = self.last.get();
            if last < self.slots() && self.holds(last) && ((*self.ops).eq)(self.key(last), key)
            {
                return Some(last);
            }
            let hash = self.hash_of(key);
            let mask = self.table.len() - 1;
            let mut slot = hash as usize & mask;
            loop {
                let entry = self.table[slot];
                if entry == 0 {
                    return None;
                }
                let position = entry as usize - 1;
                if self.hashes[position] == hash && ((*self.ops).eq)(self.key(position), key) {
                    self.last.set(position);
                    return Some(position);
                }
                slot = (slot + 1) & mask;
            }
        }
    }

    fn place(&mut self, position: usize) {
        let mask = self.table.len() - 1;
        let mut slot = self.hashes[position] as usize & mask;
        while self.table[slot] != 0 {
            slot = (slot + 1) & mask;
        }
        self.table[slot] = position as u32 + 1;
    }

    /// Appends a copy of the key with an empty value, returning its position.
    unsafe fn push(&mut self, key: *const u8, hash: u64) -> usize {
        if (self.slots() + 1) * 4 > self.table.len() * 3 {
            // Closing up the places of removed entries may leave room
            // enough; otherwise the table doubles.
            if self.removed > 0 {
                self.close_up();
            }
            if (self.slots() + 1) * 4 > self.table.len() * 3 {
                self.table = vec![0; (self.table.len() * 2).max(8)];
                for position in 0..self.slots() {
                    self.place(position);
                }
            }
        }
        let position = self.slots();
        self.keys.resize(self.keys.len() + self.key_stride, 0);
        // SAFETY: the new key storage is `key_stride` words.
        unsafe {
            let target = self.keys.as_mut_ptr().add(position * self.key_stride) as *mut u8;
            match (*self.ops).clone {
                Some(clone) => clone(key, target),
                None => std::ptr::copy_nonoverlapping(key, target, (*self.ops).size as usize),
            }
        }
        self.hashes.push(hash);
        self.values.resize(self.values.len() + self.stride, 0);
        self.place(position);
        position
    }

    unsafe fn describe(&self, key: *const u8) -> String {
        let mut builder = Builder(SayonStr::EMPTY);
        // SAFETY: valid key and builder.
        unsafe {
            ((*self.ops).display)(&mut builder, key, true);
            let text = as_str(&builder.0).to_string();
            sayon_str_drop(&mut builder.0);
            text
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_new(
    out: *mut *mut SayonDict,
    ops: *const KeyOps,
    value_size: i64,
) {
    let value_size = value_size.max(0) as usize;
    // SAFETY: `ops` is a static table from the generated code.
    let key_size = unsafe { (*ops).size.max(0) as usize };
    let dict = SayonDict {
        ops,
        keys: Vec::new(),
        key_stride: key_size.div_ceil(8).max(1),
        hashes: Vec::new(),
        table: Vec::new(),
        values: Vec::new(),
        stride: value_size.div_ceil(8).max(1),
        value_size,
        last: std::cell::Cell::new(usize::MAX),
        removed: 0,
    };
    // SAFETY: `out` is writable.
    allocated();
    unsafe { *out = Box::into_raw(Box::new(dict)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_len(d: *const SayonDict) -> i64 {
    // SAFETY: valid dictionary.
    unsafe { (*d).len() as i64 }
}

/// The places for entries, those of removed entries among them: where a
/// loop over the entries ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_slots(d: *const SayonDict) -> i64 {
    // SAFETY: valid dictionary.
    unsafe { (*d).slots() as i64 }
}

/// The first place at or after `position` that holds an entry, or the
/// number of places when none does: a loop over the entries steps over the
/// places of removed ones.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_next(d: *const SayonDict, position: i64) -> i64 {
    // SAFETY: valid dictionary.
    let dict = unsafe { &*d };
    let mut position = position.max(0) as usize;
    while position < dict.slots() && !dict.holds(position) {
        position += 1;
    }
    position as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_find(d: *const SayonDict, key: *const u8) -> i64 {
    // SAFETY: valid dictionary and key.
    unsafe { (*d).find(key).map(|i| i as i64).unwrap_or(-1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_find_checked(
    err: *mut *mut SayonError,
    d: *const SayonDict,
    key: *const u8,
    location: *const c_char,
    out: *mut i64,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe {
        match (*d).find(key) {
            Some(position) => {
                *out = position as i64;
                0
            }
            None => {
                let shown = (*d).describe(key);
                set_error(
                    err,
                    "KeyError",
                    format!("the key {shown} is not in the dictionary"),
                    location,
                )
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_key_ptr(d: *const SayonDict, position: i64) -> *const u8 {
    // SAFETY: valid dictionary and position.
    unsafe { (*d).key(position as usize) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_value_ptr(d: *mut SayonDict, position: i64) -> *mut u8 {
    // SAFETY: valid dictionary and position.
    unsafe { (*d).value(position as usize) }
}

/// Inserts or replaces an entry. The key is copied; the value's bytes are
/// moved in, and a replaced value is destroyed with `drop_value`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_set(
    d: *mut SayonDict,
    key: *const u8,
    value: *const u8,
    drop_value: DropFn,
) {
    // SAFETY: valid dictionary, key, and value of `value_size` bytes.
    unsafe {
        let dict = &mut *d;
        let position = match dict.find(key) {
            Some(position) => {
                if let Some(drop_value) = drop_value {
                    drop_value(dict.value(position));
                }
                position
            }
            None => {
                let hash = dict.hash_of(key);
                dict.push(key, hash)
            }
        };
        let target = dict.value(position);
        std::ptr::copy_nonoverlapping(value, target, dict.value_size);
    }
}

/// Removes a key's entry, moving its value into `out`; the other entries
/// keep their order. Returns whether the key was there.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_remove(
    d: *mut SayonDict,
    key: *const u8,
    out: *mut u8,
) -> bool {
    // SAFETY: valid dictionary, key, and output of `value_size` bytes.
    unsafe {
        let dict = &mut *d;
        let Some(position) = dict.find(key) else {
            return false;
        };
        std::ptr::copy_nonoverlapping(dict.value(position), out, dict.value_size);
        if let Some(drop_key) = (*dict.ops).drop {
            drop_key(dict.key(position) as *mut u8);
        }
        // The place stays, empty, so that no other entry moves.
        dict.hashes[position] = REMOVED;
        dict.removed += 1;
        if dict.last.get() == position {
            dict.last.set(usize::MAX);
        }
        if dict.removed > dict.len() {
            dict.close_up();
        }
        true
    }
}

/// Destroys every key and value, frees the dictionary, and leaves the slot
/// empty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_drop(slot: *mut *mut SayonDict, drop_value: DropFn) {
    // SAFETY: `slot` holds null or a dictionary from `sayon_dict_new`.
    unsafe {
        let d = *slot;
        if d.is_null() {
            return;
        }
        let mut dict = Box::from_raw(d);
        released();
        for position in 0..dict.slots() {
            if !dict.holds(position) {
                continue;
            }
            if let Some(drop_value) = drop_value {
                drop_value(dict.value(position));
            }
            if let Some(drop_key) = (*dict.ops).drop {
                drop_key(dict.key(position) as *mut u8);
            }
        }
        *slot = std::ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_clone(
    src: *const SayonDict,
    dst: *mut *mut SayonDict,
    clone_value: CloneFn,
) {
    // SAFETY: valid source; `dst` is writable.
    unsafe {
        let source = &*src;
        sayon_dict_new(dst, source.ops, source.value_size as i64);
        let target = &mut **dst;
        for position in 0..source.slots() {
            if !source.holds(position) {
                continue;
            }
            let copied = target.push(source.key(position), source.hashes[position]);
            let from = source.values.as_ptr().add(position * source.stride) as *const u8;
            let to = target.value(copied);
            match clone_value {
                Some(clone_value) => clone_value(from, to),
                None => std::ptr::copy_nonoverlapping(from, to, source.value_size),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_dict_eq(
    a: *const SayonDict,
    b: *const SayonDict,
    eq_value: EqFn,
) -> bool {
    // SAFETY: valid dictionaries of the same type.
    unsafe {
        let (x, y) = (&*a, &*b);
        if x.len() != y.len() {
            return false;
        }
        for position in 0..x.slots() {
            if !x.holds(position) {
                continue;
            }
            let Some(other) = y.find(x.key(position)) else {
                return false;
            };
            let left = x.values.as_ptr().add(position * x.stride) as *const u8;
            let right = y.values.as_ptr().add(other * y.stride) as *const u8;
            if !eq_value(left, right) {
                return false;
            }
        }
        true
    }
}

/// A hash of a string's bytes, for dictionary keys.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_str_hash(s: *const SayonStr) -> i64 {
    // SAFETY: valid string.
    let text = unsafe { as_str(s) };
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash as i64
}

// ----- errors ----------------------------------------------------------------------------------

/// Stores a new error with a message string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_error_raise(
    err: *mut *mut SayonError,
    code: *const c_char,
    message: *const SayonStr,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe { set_error(err, c_str(code), as_str(message).to_string(), location) }
}

/// Stores a new error with a constant message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_error_raise_text(
    err: *mut *mut SayonError,
    code: *const c_char,
    message: *const c_char,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe { set_error(err, c_str(code), c_str(message).to_string(), location) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_error_overflow(
    err: *mut *mut SayonError,
    symbol: *const c_char,
    a: i64,
    b: i64,
    code: u32,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe {
        let message = format!(
            "{} {} {} does not fit in {}",
            widen(a, code),
            c_str(symbol),
            widen(b, code),
            type_name(code)
        );
        set_error(err, "OverflowError", message, location)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_error_negate(
    err: *mut *mut SayonError,
    a: i64,
    code: u32,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe {
        let message = format!("-({}) does not fit in {}", widen(a, code), type_name(code));
        set_error(err, "OverflowError", message, location)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_error_drop(slot: *mut *mut SayonError) {
    // SAFETY: the slot holds null or an error from this module.
    unsafe {
        if !(*slot).is_null() {
            drop(Box::from_raw(*slot));
            released();
            *slot = std::ptr::null_mut();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_error_clone(src: *const *mut SayonError, dst: *mut *mut SayonError) {
    // SAFETY: valid slots.
    unsafe {
        let error = &**src;
        allocated();
        *dst = Box::into_raw(Box::new(SayonError {
            code: error.code.clone(),
            message: error.message.clone(),
            location: error.location.clone(),
        }));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_error_message(e: *const *mut SayonError, out: *mut SayonStr) {
    // SAFETY: valid error slot.
    unsafe { *out = owned((**e).message.clone()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_error_code(e: *const *mut SayonError, out: *mut SayonStr) {
    // SAFETY: valid error slot.
    unsafe { *out = owned((**e).code.clone()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_error_eq(
    a: *const *mut SayonError,
    b: *const *mut SayonError,
) -> bool {
    // SAFETY: valid error slots.
    unsafe {
        let (x, y) = (&**a, &**b);
        x.code == y.code && x.message == y.message
    }
}

/// Reports an error nothing caught and releases it. Returns the exit status.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_report_uncaught(e: *mut SayonError) -> i32 {
    let _ = crate::console::stdout().flush();
    // SAFETY: `e` came from this module and is not used again.
    unsafe {
        if e.is_null() {
            report!("error: uncaught error");
            return 1;
        }
        let error = Box::from_raw(e);
        released();
        if error.code == crate::host::EXIT_CODE {
            return crate::host::exit_status(&error.message);
        }
        report!("error: uncaught {}: {}", error.code, error.message);
        if let Some(location) = &error.location {
            report!("  --> {location}");
        }
    }
    1
}

// ----- checks and conversions ---------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_int_convert(
    err: *mut *mut SayonError,
    value: i64,
    from: u32,
    to: u32,
    location: *const c_char,
    out: *mut i64,
) -> i32 {
    let wide = widen(value, from);
    let (min, max) = text::integer_type_range(to);
    // SAFETY: valid pointers.
    unsafe {
        if wide < min || wide > max {
            return set_error(
                err,
                "ConversionError",
                format!("{wide} does not fit in {}", type_name(to)),
                location,
            );
        }
        *out = wide as i64;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_float_to_int(
    err: *mut *mut SayonError,
    value: f64,
    to: u32,
    location: *const c_char,
    out: *mut i64,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe {
        if !value.is_finite() {
            let message = format!(
                "{} is not a finite number, so it has no {} value",
                text::format_f64(value),
                type_name(to)
            );
            return set_error(err, "ConversionError", message, location);
        }
        let truncated = value.trunc();
        let (min, max) = text::integer_type_range(to);
        if truncated < min as f64 || truncated >= (max + 1) as f64 {
            let message = format!(
                "{} does not fit in {}",
                text::format_f64(value),
                type_name(to)
            );
            return set_error(err, "ConversionError", message, location);
        }
        *out = truncated as i128 as i64;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_f64_to_f32(
    err: *mut *mut SayonError,
    value: f64,
    location: *const c_char,
    out: *mut f32,
) -> i32 {
    let narrowed = value as f32;
    // SAFETY: valid pointers.
    unsafe {
        if value.is_finite() && narrowed.is_infinite() {
            let message = format!("{} is too large for float32", text::format_f64(value));
            return set_error(err, "ConversionError", message, location);
        }
        *out = narrowed;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_tonumber(
    err: *mut *mut SayonError,
    s: *const SayonStr,
    location: *const c_char,
    is_float: *mut bool,
    integer: *mut i64,
    float: *mut f64,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe {
        match text::parse_number(as_str(s)) {
            Ok(text::Number::Integer(value)) => {
                *is_float = false;
                *integer = value;
                0
            }
            Ok(text::Number::Float(value)) => {
                *is_float = true;
                *float = value;
                0
            }
            Err((code, message)) => set_error(err, code, message, location),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_range_new(
    err: *mut *mut SayonError,
    start: i64,
    end: i64,
    step: i64,
    location: *const c_char,
    out: *mut SayonRange,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe {
        if step == 0 {
            return set_error(
                err,
                "RangeError",
                "a range step cannot be zero".to_string(),
                location,
            );
        }
        *out = SayonRange { start, end, step };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_range_length(r: *const SayonRange) -> i64 {
    // SAFETY: valid range.
    unsafe {
        let range = &*r;
        text::range_length(range.start, range.end, range.step)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_assert_failed(
    err: *mut *mut SayonError,
    message: *const SayonStr,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid pointers; `message` may be null.
    unsafe {
        let text = if message.is_null() {
            "assertion failed".to_string()
        } else {
            as_str(message).to_string()
        };
        set_error(err, "AssertionError", text, location)
    }
}

// ----- files -------------------------------------------------------------------------------------

fn io_message(error: &std::io::Error) -> String {
    match error.kind() {
        std::io::ErrorKind::NotFound => "the file does not exist".to_string(),
        std::io::ErrorKind::PermissionDenied => "permission denied".to_string(),
        _ => error.to_string(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_read_file(
    err: *mut *mut SayonError,
    path: *const SayonStr,
    location: *const c_char,
    out: *mut SayonStr,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe {
        let path = as_str(path);
        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) => {
                return set_error(
                    err,
                    "IOError",
                    format!("could not read \"{path}\": {}", io_message(&error)),
                    location,
                );
            }
        };
        match String::from_utf8(bytes) {
            Ok(text) => {
                *out = owned(text);
                0
            }
            Err(error) => set_error(
                err,
                "EncodingError",
                format!(
                    "\"{path}\" is not valid UTF-8 (byte {})",
                    error.utf8_error().valid_up_to()
                ),
                location,
            ),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_write_file(
    err: *mut *mut SayonError,
    path: *const SayonStr,
    content: *const SayonStr,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid pointers.
    unsafe {
        let path = as_str(path);
        match std::fs::write(path, as_str(content).as_bytes()) {
            Ok(()) => 0,
            Err(error) => set_error(
                err,
                "IOError",
                format!("could not write \"{path}\": {}", io_message(&error)),
                location,
            ),
        }
    }
}

// ----- program lifecycle ----------------------------------------------------------------------------

// ----- shared values --------------------------------------------------------------------------------
//
// A shared value is one block: the count of `Shared` handles, the count of
// `Weak` handles plus one for all the `Shared` handles together, the
// block's size, and the value at `SHARED_HEADER` bytes. The value is
// destroyed when the last `Shared` handle goes, and the block is freed when
// the last handle of either kind goes, so it is freed exactly once.

/// The bytes before a shared value; the code generator uses the same.
pub const SHARED_HEADER: i64 = 24;

#[repr(C)]
struct SharedBlock {
    strong: i64,
    weak: i64,
    size: i64,
}

/// A block for a shared value of `size` bytes, with one `Shared` handle;
/// the caller stores the value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_shared_new(size: i64) -> *mut u8 {
    let total = SHARED_HEADER + size;
    // SAFETY: a fresh block of at least the header's size.
    unsafe {
        let block = sayon_alloc(total);
        std::ptr::write(
            block as *mut SharedBlock,
            SharedBlock {
                strong: 1,
                weak: 1,
                size: total,
            },
        );
        block
    }
}

/// Removes a weak count; the last frees the block.
unsafe fn release_weak(block: *mut u8) {
    // SAFETY: a live block from `sayon_shared_new`.
    unsafe {
        let header = &mut *(block as *mut SharedBlock);
        header.weak -= 1;
        if header.weak == 0 {
            let size = header.size;
            sayon_free(block, size);
        }
    }
}

/// Drops a `Shared` handle; null is a moved-from handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_shared_release(slot: *mut *mut u8, drop: DropFn) {
    // SAFETY: the slot holds null or a live block.
    unsafe {
        let block = *slot;
        if block.is_null() {
            return;
        }
        *slot = std::ptr::null_mut();
        let header = &mut *(block as *mut SharedBlock);
        header.strong -= 1;
        if header.strong == 0 {
            if let Some(drop) = drop {
                drop(block.add(SHARED_HEADER as usize));
            }
            release_weak(block);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_shared_clone(src: *const *mut u8, dst: *mut *mut u8) {
    // SAFETY: the source holds a live block.
    unsafe {
        let block = *src;
        (*(block as *mut SharedBlock)).strong += 1;
        *dst = block;
    }
}

/// A new `Shared` handle from a `Weak` one, when a `Shared` handle is left.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_shared_upgrade(block: *mut u8) -> bool {
    // SAFETY: a weak handle keeps its block allocated.
    unsafe {
        let header = &mut *(block as *mut SharedBlock);
        if header.strong == 0 {
            return false;
        }
        header.strong += 1;
        true
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_weak_retain(block: *mut u8) {
    // SAFETY: a live block.
    unsafe { (*(block as *mut SharedBlock)).weak += 1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_weak_clone(src: *const *mut u8, dst: *mut *mut u8) {
    // SAFETY: the source holds a live block.
    unsafe {
        sayon_weak_retain(*src);
        *dst = *src;
    }
}

/// Drops a `Weak` handle; null is a moved-from handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_weak_release(slot: *mut *mut u8) {
    // SAFETY: the slot holds null or a live block.
    unsafe {
        let block = *slot;
        if block.is_null() {
            return;
        }
        *slot = std::ptr::null_mut();
        release_weak(block);
    }
}

// ----- streams --------------------------------------------------------------------------------------

/// Drops a stream reference; 0 is a moved-from stream.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_stream_drop(slot: *mut i64) {
    // SAFETY: the slot holds a stream number or 0.
    unsafe {
        if *slot != 0 {
            crate::streams::release(*slot);
            released();
            *slot = 0;
        }
    }
}

/// A clone of a stream refers to the same open file.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_stream_clone(src: *const i64, dst: *mut i64) {
    // SAFETY: valid slots.
    unsafe {
        crate::streams::share(*src);
        allocated();
        *dst = *src;
    }
}

// ----- program lifecycle ----------------------------------------------------------------------------

/// The bytes of a word before a C string's text: the size of its block.
const C_STRING_HEADER: usize = 8;

/// A new C string holding `bytes` and a NUL: the address of its text,
/// which follows a word holding the size of the whole block.
fn c_string_buffer(bytes: &[u8]) -> *mut u8 {
    let size = C_STRING_HEADER + bytes.len() + 1;
    // SAFETY: the block is `size` bytes, enough for the header, the bytes,
    // and the NUL.
    unsafe {
        let block = sayon_alloc(size as i64);
        *(block as *mut u64) = size as u64;
        let text = block.add(C_STRING_HEADER);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), text, bytes.len());
        *text.add(bytes.len()) = 0;
        text
    }
}

/// Releases the C string in a slot, unless the slot is empty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_c_string_drop(slot: *mut *mut u8) {
    // SAFETY: a non-null text pointer came from `c_string_buffer`.
    unsafe {
        let text = *slot;
        if text.is_null() {
            return;
        }
        let block = text.sub(C_STRING_HEADER);
        let size = *(block as *const u64);
        sayon_free(block, size as i64);
        *slot = std::ptr::null_mut();
    }
}

/// Shows a C string's text, quoted inside a collection like a string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_c_string(
    b: *mut Builder,
    slot: *const *const u8,
    quoted: bool,
) {
    // SAFETY: the slot holds the text of a live C string.
    unsafe {
        let text = String::from_utf8_lossy(CStr::from_ptr(*slot as *const c_char).to_bytes());
        if quoted {
            text::quote(&text, &mut *b);
        } else {
            (*b).push_str(&text);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_builder_pointer(b: *mut Builder, address: u64) {
    // The address comes widened to 64 bits on every target.
    let address = address as usize;
    // SAFETY: `b` is a live builder.
    unsafe { (*b).push_str(&text::pointer_text(address)) }
}

#[unsafe(no_mangle)]
pub extern "C" fn sayon_runtime_finish() {
    crate::streams::flush_all();
    let _ = crate::console::stdout().flush();
    // In a web page the program's entry can be a coroutine, whose frame is
    // freed only after this; `sayon_run` reports then.
    if !cfg!(all(target_family = "wasm", target_os = "unknown")) {
        sayon_runtime_report();
    }
}

/// With `SAYON_DEBUG_ALLOC`, reports the allocations still live as the
/// program ends, which a program that released everything has none of.
#[unsafe(no_mangle)]
pub extern "C" fn sayon_runtime_report() {
    if counting_live() {
        report!(
            "sayon: allocations live at exit: {}",
            LIVE.load(Ordering::Relaxed)
        );
    }
}

/// Maps the integer `main` returned to the process exit status. Portable
/// statuses are 0 to 255; anything else exits with 255 after a warning.
#[unsafe(no_mangle)]
pub extern "C" fn sayon_exit_status(code: i64) -> i32 {
    if (0..=255).contains(&code) {
        code as i32
    } else {
        let _ = crate::console::stdout().flush();
        report!(
            "warning: `main` returned {code}; portable exit statuses are 0 to 255, so 255 is used"
        );
        255
    }
}

// ----- math --------------------------------------------------------------------

/// The functions of `standard/math` that compiled code calls directly rather
/// than through their entries as primitives, which gather their arguments
/// first.
/// They are the functions the interpreter's primitives use, so both give the
/// same results.
macro_rules! math {
    ($($name:ident = |$($x:ident),+| $body:expr;)*) => {
        $(
            #[unsafe(no_mangle)]
            pub extern "C" fn $name($($x: f64),+) -> f64 {
                $body
            }
        )*
    };
}

math! {
    sayon_math_round = |x| x.round();
    sayon_math_sin = |x| x.sin();
    sayon_math_cos = |x| x.cos();
    sayon_math_tan = |x| x.tan();
    sayon_math_asin = |x| x.asin();
    sayon_math_acos = |x| x.acos();
    sayon_math_atan = |x| x.atan();
    sayon_math_atan2 = |y, x| y.atan2(x);
    sayon_math_exp = |x| x.exp();
    sayon_math_log = |x| x.ln();
    sayon_math_log10 = |x| x.log10();
    sayon_math_log2 = |x| x.log2();
    sayon_math_pow = |base, exponent| base.powf(exponent);
}

// ----- text --------------------------------------------------------------------

// The functions of `standard/strings` that compiled code calls directly. The
// primitives they stand for gather their arguments, and build their results,
// as values of their own first; these read and write Sayon's strings and
// arrays where they are. They give the same results as the primitives.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_text_contains(text: *const SayonStr, part: *const SayonStr) -> bool {
    // SAFETY: valid strings.
    unsafe { as_str(text).contains(as_str(part)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_text_starts_with(
    text: *const SayonStr,
    prefix: *const SayonStr,
) -> bool {
    // SAFETY: valid strings.
    unsafe { as_str(text).starts_with(as_str(prefix)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_text_ends_with(
    text: *const SayonStr,
    suffix: *const SayonStr,
) -> bool {
    // SAFETY: valid strings.
    unsafe { as_str(text).ends_with(as_str(suffix)) }
}

/// Where `part` first starts in `text`, counted in characters, or -1.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_text_find(text: *const SayonStr, part: *const SayonStr) -> i64 {
    // SAFETY: valid strings.
    unsafe {
        let text = as_str(text);
        match text.find(as_str(part)) {
            Some(offset) => text[..offset].chars().count() as i64,
            None => -1,
        }
    }
}

/// Splits `text` at each `separator` into a new array at `out`, made at its
/// size at once, with each piece copied straight into it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_text_split(
    err: *mut *mut SayonError,
    text: *const SayonStr,
    separator: *const SayonStr,
    out: *mut SayonArray,
    location: *const c_char,
) -> i32 {
    // SAFETY: valid strings, an array to write, and the error slot.
    unsafe {
        let (text, separator) = (as_str(text), as_str(separator));
        if separator.is_empty() {
            let message = "the separator cannot be empty".to_string();
            return set_error(err, "ValueError", message, location);
        }
        let count = text.split(separator).count();
        *out = SayonArray {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        };
        sayon_array_exact(out, count as i64, std::mem::size_of::<SayonStr>() as i64);
        let pieces = (*out).ptr as *mut SayonStr;
        for (index, piece) in text.split(separator).enumerate() {
            pieces.add(index).write(copied(piece));
        }
        (*out).len = count as i64;
        0
    }
}

/// The strings of `parts` joined by `separator`, into `out`, in a buffer
/// made at its size at once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sayon_text_join(
    parts: *const SayonArray,
    separator: *const SayonStr,
    out: *mut SayonStr,
) {
    // SAFETY: a valid array of strings, a valid string, and a string to write.
    unsafe {
        let separator = as_str(separator);
        let count = (*parts).len as usize;
        let items = (*parts).ptr as *const SayonStr;
        let mut total = separator.len() * count.saturating_sub(1);
        for index in 0..count {
            total += (*items.add(index)).bytes().len();
        }
        let mut joined = String::with_capacity(total);
        for index in 0..count {
            if index > 0 {
                joined.push_str(separator);
            }
            joined.push_str(as_str(items.add(index)));
        }
        *out = owned(joined);
    }
}

// ----- host primitives -------------------------------------------------------

/// Records the program's arguments (without the program name).
#[unsafe(no_mangle)]
pub extern "C" fn sayon_runtime_init() {
    crate::host::set_arguments(std::env::args().skip(1).collect());
    // A page has no standard errors of Rust's: a fault inside the runtime is
    // reported through the host instead.
    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    std::panic::set_hook(Box::new(|info| report!("fatal error: {info}")));
}

/// Runs a primitive, letting other tasks run while it waits for a socket:
/// it runs again whenever the socket may be ready, until it finishes or
/// waits longer than the socket's timeout.
#[cfg_attr(not(debug_assertions), inline(always))]
fn call_waiting(
    host: crate::host::Host,
    values: &[crate::host::HostArg],
) -> Result<crate::host::HostValue, crate::host::HostError> {
    use crate::network::{BLOCKED_CODE, take_blocked, timeout_message};
    let mut started = None;
    loop {
        match crate::host::run(host, values) {
            Err((code, _)) if code == BLOCKED_CODE => {
                let blocked = take_blocked().expect("what the primitive waits for");
                let started = *started.get_or_insert_with(crate::clock::Instant::now);
                let deadline = blocked.timeout.map(|timeout| started + timeout);
                match crate::tasks::wait_io(blocked.wait, deadline) {
                    Ok(()) => {}
                    Err(crate::tasks::IoStop::TimedOut) => {
                        let timeout = blocked.timeout.unwrap_or_default();
                        return Err(("TimeoutError", timeout_message(timeout)));
                    }
                    Err(crate::tasks::IoStop::Refused(refusal)) => {
                        return Err((refusal.code(), refusal.message().to_string()));
                    }
                    Err(crate::tasks::IoStop::Cancelled) => {
                        let code = crate::host::CANCELLED_CODE;
                        return Err((code, "the task was cancelled".to_string()));
                    }
                }
            }
            other => return other,
        }
    }
}

/// In a web page: has the browser fetch the request `host_fetch`'s
/// arguments describe, and lets the task wait for it while the others run.
/// The stream holding the response goes through `out`.
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
unsafe fn fetch(
    err: *mut *mut SayonError,
    values: &[crate::host::HostArg],
    out: *mut u8,
    location: *const c_char,
) -> i32 {
    // SAFETY: the caller's error slot, and its result of a stream.
    unsafe {
        let request = match crate::fetch::start(values) {
            Ok(request) => request,
            Err((code, message)) => return set_error(err, code, message, location),
        };
        let finish = move || match crate::fetch::finish(request) {
            Ok(stream) => {
                allocated();
                *(out as *mut i64) = stream;
                0
            }
            Err((code, message)) => set_error(err, code, message, location),
        };
        crate::tasks::await_response(err, request, location, finish)
    }
}

/// What each primitive's entry does (see `host::entries`), inlined there in a
/// release build so that only the primitive's own code stays. `args` points to one pointer per
/// argument value, in the primitive's parameter order. The result is written
/// through `out`; for an optional string, whether it is present is written
/// through `present` and the text through `out` when it is.
#[cfg_attr(not(debug_assertions), inline(always))]
pub(crate) unsafe fn host_entry(
    host: crate::host::Host,
    err: *mut *mut SayonError,
    args: *const *const u8,
    out: *mut u8,
    present: *mut bool,
    location: *const c_char,
) -> i32 {
    use crate::host::{HostArg, HostType, HostValue};
    // SAFETY: the generated code passes valid pointers matching the
    // primitive's parameter and result types.
    unsafe {
        let mut values = Vec::new();
        for (index, ty) in host.params().iter().enumerate() {
            let pointer = *args.add(index);
            values.push(match ty {
                HostType::String => HostArg::String(as_str(pointer as *const SayonStr)),
                HostType::Integer => HostArg::Integer(*(pointer as *const i64)),
                HostType::Float => HostArg::Float(*(pointer as *const f64)),
                HostType::Boolean => HostArg::Boolean(*pointer != 0),
                HostType::StringArray => {
                    let array = &*(pointer as *const SayonArray);
                    let elements: Vec<&str> = (0..array.len as usize)
                        .map(|i| as_str((array.ptr as *const SayonStr).add(i)))
                        .collect();
                    HostArg::Strings(elements)
                }
                HostType::Integers => {
                    let array = &*(pointer as *const SayonArray);
                    let integers = if array.len == 0 {
                        Vec::new()
                    } else {
                        std::slice::from_raw_parts(array.ptr as *const i64, array.len as usize)
                            .to_vec()
                    };
                    HostArg::Integers(integers)
                }
                HostType::Stream => HostArg::Stream(*(pointer as *const i64)),
                // A C string is its buffer's address.
                HostType::Pointer | HostType::CString => {
                    HostArg::Pointer(*(pointer as *const usize))
                }
                HostType::Bytes => {
                    let array = &*(pointer as *const SayonArray);
                    let bytes = if array.len == 0 {
                        Vec::new()
                    } else {
                        std::slice::from_raw_parts(array.ptr, array.len as usize).to_vec()
                    };
                    HostArg::Bytes(bytes)
                }
                HostType::Nothing | HostType::OptionalString | HostType::Unsigned => {
                    panic!("unsupported host parameter")
                }
            });
        }
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        if host == crate::host::Host::Fetch {
            return fetch(err, &values, out, location);
        }
        // Only a primitive that can wait for a socket brings in the
        // scheduler's waiting.
        let result = if host.may_wait() {
            call_waiting(host, &values)
        } else {
            crate::host::run(host, &values)
        };
        match result {
            Ok(HostValue::String(text)) => *(out as *mut SayonStr) = owned(text),
            Ok(HostValue::Integer(value)) => *(out as *mut i64) = value,
            Ok(HostValue::Float(value)) => *(out as *mut f64) = value,
            Ok(HostValue::Boolean(value)) => *out = value as u8,
            Ok(HostValue::Nothing) => *out = 0,
            Ok(HostValue::Strings(items)) => {
                let array = out as *mut SayonArray;
                *array = SayonArray {
                    ptr: std::ptr::null_mut(),
                    len: 0,
                    cap: 0,
                };
                let element_size = std::mem::size_of::<SayonStr>() as i64;
                let count = items.len() as i64;
                if count > 0 {
                    sayon_array_reserve(array, count, element_size);
                }
                for (index, item) in items.into_iter().enumerate() {
                    let target = ((*array).ptr as *mut SayonStr).add(index);
                    std::ptr::write(target, owned(item));
                }
                (*array).len = count;
            }
            Ok(HostValue::OptionalString(text)) => {
                *present = text.is_some();
                if let Some(text) = text {
                    *(out as *mut SayonStr) = owned(text);
                }
            }
            Ok(HostValue::Stream(id)) => {
                allocated();
                *(out as *mut i64) = id;
            }
            Ok(HostValue::Unsigned(value)) => *(out as *mut u64) = value,
            Ok(HostValue::Pointer(address)) => *(out as *mut usize) = address,
            Ok(HostValue::CString(bytes)) => *(out as *mut *mut u8) = c_string_buffer(&bytes),
            Ok(HostValue::Bytes(bytes)) => {
                let array = out as *mut SayonArray;
                *array = SayonArray {
                    ptr: std::ptr::null_mut(),
                    len: 0,
                    cap: 0,
                };
                let count = bytes.len() as i64;
                if count > 0 {
                    sayon_array_reserve(array, count, 1);
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), (*array).ptr, bytes.len());
                }
                (*array).len = count;
            }
            Err((code, message)) => return set_error(err, code, message, location),
        }
    }
    0
}
