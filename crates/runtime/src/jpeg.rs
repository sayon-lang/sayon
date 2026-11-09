/*!
JPEG images for `standard/graphics`: decoding baseline and progressive
JPEG (ITU-T T.81) with Huffman coding, as cameras and the web make them,
into red, green, blue, and alpha bytes, turned as a photo's EXIF
orientation says. Samples are gray, YCbCr, RGB, or Adobe's CMYK and
YCCK, with any sampling factors and restart intervals. The inverse DCT
is libjpeg's integer one, color is upsampled by repeating samples, and
YCbCr becomes RGB as libjpeg rounds it, so that the pixels are those of
`djpeg -nosmooth -dct int`. Coded data that ends early leaves the rest
of the image gray, as browsers show such files. Arithmetic coding,
12-bit samples, and the lossless and hierarchical modes fail with
ImageError.
*/

use crate::host::HostError;
use crate::png::Image;

fn fail(message: impl Into<String>) -> HostError {
    ("ImageError", message.into())
}

fn damaged() -> HostError {
    fail("the JPEG image is damaged")
}

/**
Where each coefficient of a block goes, row by row, by its place in the
zigzag order the image lists them in.
*/
const ZIGZAG: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];

/** Whether bytes start as a JPEG image does. */
pub fn is_jpeg(bytes: &[u8]) -> bool {
    bytes.len() >= 3 && bytes[0] == 0xFF && bytes[1] == 0xD8 && bytes[2] == 0xFF
}

/** A big-endian number of two bytes. */
fn number(bytes: &[u8], at: usize) -> Result<usize, HostError> {
    bytes
        .get(at..at + 2)
        .map(|pair| (pair[0] as usize) << 8 | pair[1] as usize)
        .ok_or_else(|| fail("the JPEG image ends early"))
}

/** The next marker from `at`: its code, and where what follows it starts. */
fn next_marker(bytes: &[u8], mut at: usize) -> Option<(u8, usize)> {
    while at + 1 < bytes.len() {
        if bytes[at] == 0xFF && !matches!(bytes[at + 1], 0x00 | 0xFF) {
            return Some((bytes[at + 1], at + 2));
        }
        at += 1;
    }
    None
}

/** Whether a marker starts a frame, of any coding. */
fn starts_frame(marker: u8) -> bool {
    matches!(marker, 0xC0..=0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF)
}

/** The orientation an EXIF segment gives a photo, 1 to 8, or nothing. */
fn orientation(segment: &[u8]) -> Option<u8> {
    let tiff = segment.strip_prefix(b"Exif\0\0")?;
    let big = match tiff.get(0..2)? {
        b"MM" => true,
        b"II" => false,
        _ => return None,
    };
    let read = |at: usize, size: usize| -> Option<usize> {
        let bytes = tiff.get(at..at.checked_add(size)?)?;
        let mut value = 0;
        for index in 0..size {
            let byte = if big {
                bytes[index]
            } else {
                bytes[size - 1 - index]
            };
            value = value << 8 | byte as usize;
        }
        Some(value)
    };
    if read(2, 2)? != 42 {
        return None;
    }
    let directory = read(4, 4)?;
    for entry in 0..read(directory, 2)? {
        let at = directory.checked_add(2 + 12 * entry)?;
        if read(at, 2)? == 0x0112 {
            let value = read(at.checked_add(8)?, 2)?;
            return (1..=8).contains(&value).then_some(value as u8);
        }
    }
    None
}

/**
The width and height of a JPEG image as decoding gives them, turned as
its orientation says, read from its header alone.
*/
pub fn size(bytes: &[u8]) -> Result<(usize, usize), HostError> {
    if !is_jpeg(bytes) {
        return Err(fail("the data is not a JPEG image"));
    }
    let mut turned = false;
    let mut at = 2;
    while let Some((marker, start)) = next_marker(bytes, at) {
        if matches!(marker, 0x01 | 0xD0..=0xD8) {
            at = start;
            continue;
        }
        if matches!(marker, 0xD9 | 0xDA) {
            break;
        }
        let length = number(bytes, start)?;
        let end = start + length;
        if length < 2 || end > bytes.len() {
            return Err(fail("the JPEG image ends early"));
        }
        let segment = &bytes[start + 2..end];
        if marker == 0xE1 {
            if let Some(orientation) = orientation(segment) {
                turned = orientation >= 5;
            }
        } else if starts_frame(marker) {
            let (height, width) = (number(segment, 1)?, number(segment, 3)?);
            return Ok(if turned {
                (height, width)
            } else {
                (width, height)
            });
        }
        at = end;
    }
    Err(fail("the JPEG image has no frame"))
}

/**
The bits a code is looked up by at once; longer codes are found bit by
bit.
*/
const FAST_BITS: u32 = 9;

/** A Huffman table of a scan's symbols. */
struct Huffman {
    /**
    By the next FAST_BITS bits: a code's length times 256 and its symbol,
    or 0 for a longer code.
    */
    fast: Vec<u16>,
    /**
    By length: the largest code of that length, or -1 for none; the
    smallest; and where its symbols start.
    */
    largest: [i32; 17],
    smallest: [i32; 17],
    start: [usize; 17],
    symbols: Vec<u8>,
}

impl Huffman {
    /**
    The table of `counts` codes of each length from 1 to 16, for
    `symbols` in order.
    */
    fn new(counts: &[u8], symbols: &[u8]) -> Result<Huffman, HostError> {
        let mut table = Huffman {
            fast: vec![0; 1 << FAST_BITS],
            largest: [-1; 17],
            smallest: [0; 17],
            start: [0; 17],
            symbols: symbols.to_vec(),
        };
        let mut code = 0u32;
        let mut next = 0;
        for length in 1..=16u32 {
            let count = counts[length as usize - 1] as usize;
            table.start[length as usize] = next;
            table.smallest[length as usize] = code as i32;
            for _ in 0..count {
                if code >= 1 << length {
                    return Err(damaged());
                }
                if length <= FAST_BITS {
                    let shift = FAST_BITS - length;
                    let first = (code << shift) as usize;
                    let entry = (length as u16) << 8 | symbols[next] as u16;
                    table.fast[first..first + (1 << shift)].fill(entry);
                }
                code += 1;
                next += 1;
            }
            if count > 0 {
                table.largest[length as usize] = code as i32 - 1;
            }
            code <<= 1;
        }
        Ok(table)
    }
}

/**
The bits of a scan's coded data, first to last, without the zero bytes
stuffed after each 0xFF. From a marker, or the end of the data, on, it
gives zeros.
*/
struct Reader<'a> {
    data: &'a [u8],
    position: usize,
    bits: u64,
    count: u32,
    ended: bool,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8], position: usize) -> Reader<'a> {
        Reader {
            data,
            position,
            bits: 0,
            count: 0,
            ended: false,
        }
    }

    /** Takes whole bytes until more than 56 bits wait. */
    fn fill(&mut self) {
        while self.count <= 56 {
            let mut byte = 0;
            if !self.ended {
                match self.data.get(self.position) {
                    Some(0xFF) => {
                        if self.data.get(self.position + 1) == Some(&0x00) {
                            byte = 0xFF;
                            self.position += 2;
                        } else {
                            self.ended = true;
                        }
                    }
                    Some(&value) => {
                        byte = value;
                        self.position += 1;
                    }
                    None => self.ended = true,
                }
            }
            self.bits |= (byte as u64) << (56 - self.count);
            self.count += 8;
        }
    }

    /** The next `count` bits, 1 to 16 of them, without taking them. */
    fn peek(&mut self, count: u32) -> u32 {
        if self.count < count {
            self.fill();
        }
        (self.bits >> (64 - count)) as u32
    }

    fn skip(&mut self, count: u32) {
        self.bits <<= count;
        self.count -= count;
    }

    fn bits(&mut self, count: u32) -> u32 {
        if count == 0 {
            return 0;
        }
        let value = self.peek(count);
        self.skip(count);
        value
    }

    fn bit(&mut self) -> bool {
        self.bits(1) == 1
    }

    /** A number of `size` bits, made signed as T.81's EXTEND does. */
    fn signed(&mut self, size: u32) -> i32 {
        if size == 0 {
            return 0;
        }
        let value = self.bits(size) as i32;
        if value < 1 << (size - 1) {
            value - (1 << size) + 1
        } else {
            value
        }
    }

    /** The next symbol of a Huffman table. */
    fn symbol(&mut self, table: &Huffman) -> Result<u8, HostError> {
        let entry = table.fast[self.peek(FAST_BITS) as usize];
        if entry != 0 {
            self.skip((entry >> 8) as u32);
            return Ok(entry as u8);
        }
        let window = self.peek(16) as i32;
        for length in FAST_BITS + 1..=16 {
            let code = window >> (16 - length);
            if code <= table.largest[length as usize] {
                let offset = code - table.smallest[length as usize];
                if offset < 0 {
                    return Err(damaged());
                }
                self.skip(length);
                let index = table.start[length as usize] + offset as usize;
                return table.symbols.get(index).copied().ok_or_else(damaged);
            }
        }
        Err(damaged())
    }

    /** Goes past a restart marker, dropping the bits left before it. */
    fn restart(&mut self) {
        self.bits = 0;
        self.count = 0;
        self.ended = false;
        while self.position + 1 < self.data.len() {
            if self.data[self.position] == 0xFF {
                match self.data[self.position + 1] {
                    0xD0..=0xD7 => {
                        self.position += 2;
                        return;
                    }
                    0x00 | 0xFF => {}
                    _ => {
                        self.ended = true;
                        return;
                    }
                }
            }
            self.position += 1;
        }
        self.ended = true;
    }
}

/**
A component of a frame: its id and sampling factors, its quantization
table, and the coefficients of its blocks, in whole MCUs.
*/
struct Component {
    id: u8,
    h: usize,
    v: usize,
    table: usize,
    /**
    The quantization table, row by row, as it was when the component's
    first scan began.
    */
    quantization: Option<[i32; 64]>,
    blocks_wide: usize,
    blocks_high: usize,
    coefficients: Vec<i16>,
    prediction: i32,
}

struct Frame {
    width: usize,
    height: usize,
    progressive: bool,
    components: Vec<Component>,
    h_most: usize,
    v_most: usize,
    mcus_wide: usize,
    mcus_high: usize,
}

impl Frame {
    /**
    How many of a component's blocks hold the image, across and down, as
    a scan of that component alone goes through them.
    */
    fn blocks(&self, component: &Component) -> (usize, usize) {
        (
            (self.width * component.h).div_ceil(self.h_most).div_ceil(8),
            (self.height * component.v)
                .div_ceil(self.v_most)
                .div_ceil(8),
        )
    }
}

/**
A scan being decoded: its band of coefficients, from `first` to `last`
in zigzag order, and its successive approximation, bits `high` to
`low`; and how many more blocks end their band at once, by an EOB run.
*/
struct Scan<'a> {
    reader: Reader<'a>,
    progressive: bool,
    first: usize,
    last: usize,
    high: u32,
    low: u32,
    run: u32,
}

impl Scan<'_> {
    fn block(
        &mut self,
        block: &mut [i16],
        prediction: &mut i32,
        dc: Option<&Huffman>,
        ac: Option<&Huffman>,
    ) -> Result<(), HostError> {
        if !self.progressive {
            return self.whole(
                block,
                prediction,
                dc.ok_or_else(damaged)?,
                ac.ok_or_else(damaged)?,
            );
        }
        if self.first == 0 {
            if self.high == 0 {
                let size = self.reader.symbol(dc.ok_or_else(damaged)?)? as u32;
                if size > 15 {
                    return Err(damaged());
                }
                *prediction = prediction.wrapping_add(self.reader.signed(size));
                block[0] = (*prediction << self.low) as i16;
            } else if self.reader.bit() {
                block[0] |= 1 << self.low;
            }
            return Ok(());
        }
        let ac = ac.ok_or_else(damaged)?;
        if self.high == 0 {
            self.first_ac(block, ac)
        } else {
            self.refine_ac(block, ac)
        }
    }

    /**
    A block of a sequential scan: its DC difference and every AC
    coefficient.
    */
    fn whole(
        &mut self,
        block: &mut [i16],
        prediction: &mut i32,
        dc: &Huffman,
        ac: &Huffman,
    ) -> Result<(), HostError> {
        let size = self.reader.symbol(dc)? as u32;
        if size > 15 {
            return Err(damaged());
        }
        *prediction = prediction.wrapping_add(self.reader.signed(size));
        block[0] = *prediction as i16;
        let mut k = 1;
        while k < 64 {
            let symbol = self.reader.symbol(ac)?;
            let (zeros, size) = ((symbol >> 4) as usize, (symbol & 15) as u32);
            if size == 0 {
                if zeros != 15 {
                    break;
                }
                k += 16;
                continue;
            }
            k += zeros;
            if k > 63 {
                return Err(damaged());
            }
            block[ZIGZAG[k]] = self.reader.signed(size) as i16;
            k += 1;
        }
        Ok(())
    }

    /** The first scan of a band of AC coefficients: their high bits. */
    fn first_ac(&mut self, block: &mut [i16], ac: &Huffman) -> Result<(), HostError> {
        if self.run > 0 {
            self.run -= 1;
            return Ok(());
        }
        let mut k = self.first;
        while k <= self.last {
            let symbol = self.reader.symbol(ac)?;
            let (zeros, size) = ((symbol >> 4) as u32, (symbol & 15) as u32);
            if size == 0 {
                if zeros < 15 {
                    self.run = (1 << zeros) - 1 + self.reader.bits(zeros);
                    break;
                }
                k += 16;
                continue;
            }
            k += zeros as usize;
            if k > 63 {
                return Err(damaged());
            }
            block[ZIGZAG[k]] = (self.reader.signed(size) << self.low) as i16;
            k += 1;
        }
        Ok(())
    }

    /**
    A later scan of a band of AC coefficients: a bit more of each that
    is not zero, and the coefficients that become so by this bit.
    */
    fn refine_ac(&mut self, block: &mut [i16], ac: &Huffman) -> Result<(), HostError> {
        let bit: i16 = 1 << self.low;
        let mut k = self.first;
        if self.run == 0 {
            while k <= self.last {
                let symbol = self.reader.symbol(ac)?;
                let (mut zeros, size) = ((symbol >> 4) as i32, symbol & 15);
                let mut value = 0;
                if size == 0 {
                    if zeros < 15 {
                        self.run = (1 << zeros) + self.reader.bits(zeros as u32);
                        /* The rest of the band is refined, then it ends. */
                        zeros = 64;
                    }
                } else {
                    if size != 1 {
                        return Err(damaged());
                    }
                    value = if self.reader.bit() { bit } else { -bit };
                }
                while k <= self.last {
                    let coefficient = &mut block[ZIGZAG[k]];
                    k += 1;
                    if *coefficient != 0 {
                        if self.reader.bit() && *coefficient & bit == 0 {
                            *coefficient = refined(*coefficient, bit);
                        }
                    } else if zeros == 0 {
                        *coefficient = value;
                        break;
                    } else {
                        zeros -= 1;
                    }
                }
            }
        }
        if self.run > 0 {
            while k <= self.last {
                let coefficient = &mut block[ZIGZAG[k]];
                k += 1;
                if *coefficient != 0 && self.reader.bit() && *coefficient & bit == 0 {
                    *coefficient = refined(*coefficient, bit);
                }
            }
            self.run -= 1;
        }
        Ok(())
    }

    fn restart(&mut self) {
        self.reader.restart();
        self.run = 0;
    }
}

/** A coefficient with one more bit of its size. */
fn refined(coefficient: i16, bit: i16) -> i16 {
    if coefficient >= 0 {
        coefficient.wrapping_add(bit)
    } else {
        coefficient.wrapping_sub(bit)
    }
}

#[derive(Default)]
struct Decoder {
    quantization: [Option<[i32; 64]>; 4],
    dc: [Option<Huffman>; 4],
    ac: [Option<Huffman>; 4],
    interval: usize,
    frame: Option<Frame>,
    orientation: u8,
    transform: Option<u8>,
    scans: usize,
}

impl Decoder {
    fn frame(&mut self, segment: &[u8], progressive: bool, most: usize) -> Result<(), HostError> {
        if self.frame.is_some() {
            return Err(damaged());
        }
        let precision = *segment.first().ok_or_else(damaged)?;
        if precision != 8 {
            return Err(fail(format!(
                "JPEG images of {precision}-bit samples are not supported, only of 8-bit ones"
            )));
        }
        let height = number(segment, 1)?;
        let width = number(segment, 3)?;
        let count = *segment.get(5).ok_or_else(damaged)? as usize;
        if width == 0 || height == 0 {
            return Err(fail("the JPEG image gives no size"));
        }
        if width * height > most || width > 16384 || height > 16384 {
            return Err(fail(format!(
                "an image of {width} by {height} pixels is larger than Sayon draws"
            )));
        }
        if !matches!(count, 1 | 3 | 4) || segment.len() < 6 + 3 * count {
            return Err(damaged());
        }
        let mut components = Vec::with_capacity(count);
        for index in 0..count {
            let at = 6 + 3 * index;
            let (h, v) = (
                (segment[at + 1] >> 4) as usize,
                (segment[at + 1] & 15) as usize,
            );
            let table = segment[at + 2] as usize;
            if !(1..=4).contains(&h) || !(1..=4).contains(&v) || table > 3 {
                return Err(damaged());
            }
            components.push(Component {
                id: segment[at],
                h,
                v,
                table,
                quantization: None,
                blocks_wide: 0,
                blocks_high: 0,
                coefficients: Vec::new(),
                prediction: 0,
            });
        }
        let h_most = components.iter().map(|c| c.h).max().unwrap_or(1);
        let v_most = components.iter().map(|c| c.v).max().unwrap_or(1);
        let mcus_wide = width.div_ceil(8 * h_most);
        let mcus_high = height.div_ceil(8 * v_most);
        for component in &mut components {
            component.blocks_wide = mcus_wide * component.h;
            component.blocks_high = mcus_high * component.v;
            component.coefficients = vec![0; component.blocks_wide * component.blocks_high * 64];
        }
        self.frame = Some(Frame {
            width,
            height,
            progressive,
            components,
            h_most,
            v_most,
            mcus_wide,
            mcus_high,
        });
        Ok(())
    }

    fn huffman(&mut self, mut segment: &[u8]) -> Result<(), HostError> {
        while !segment.is_empty() {
            if segment.len() < 17 {
                return Err(damaged());
            }
            let (class, index) = (segment[0] >> 4, (segment[0] & 15) as usize);
            if class > 1 || index > 3 {
                return Err(damaged());
            }
            let counts = &segment[1..17];
            let total: usize = counts.iter().map(|&count| count as usize).sum();
            let symbols = segment.get(17..17 + total).ok_or_else(damaged)?;
            let table = Huffman::new(counts, symbols)?;
            if class == 0 {
                self.dc[index] = Some(table);
            } else {
                self.ac[index] = Some(table);
            }
            segment = &segment[17 + total..];
        }
        Ok(())
    }

    fn quantization(&mut self, mut segment: &[u8]) -> Result<(), HostError> {
        while !segment.is_empty() {
            let (precision, index) = (segment[0] >> 4, (segment[0] & 15) as usize);
            if precision > 1 || index > 3 {
                return Err(damaged());
            }
            let size = if precision == 0 { 64 } else { 128 };
            let values = segment.get(1..1 + size).ok_or_else(damaged)?;
            let mut table = [0; 64];
            for k in 0..64 {
                table[ZIGZAG[k]] = if precision == 0 {
                    values[k] as i32
                } else {
                    (values[2 * k] as i32) << 8 | values[2 * k + 1] as i32
                };
            }
            self.quantization[index] = Some(table);
            segment = &segment[1 + size..];
        }
        Ok(())
    }

    /**
    Decodes a scan whose coded data starts at `start`, and gives where
    that data ends.
    */
    fn scan(&mut self, segment: &[u8], bytes: &[u8], start: usize) -> Result<usize, HostError> {
        let Some(frame) = self.frame.as_mut() else {
            return Err(fail("the JPEG image has a scan before its frame"));
        };
        let count = *segment.first().ok_or_else(damaged)? as usize;
        if !(1..=4).contains(&count) || segment.len() < 4 + 2 * count {
            return Err(damaged());
        }
        let at = 1 + 2 * count;
        let mut scan = Scan {
            reader: Reader::new(bytes, start),
            progressive: frame.progressive,
            first: segment[at] as usize,
            last: segment[at + 1] as usize,
            high: (segment[at + 2] >> 4) as u32,
            low: (segment[at + 2] & 15) as u32,
            run: 0,
        };
        if !frame.progressive {
            (scan.first, scan.last, scan.high, scan.low) = (0, 63, 0, 0);
        } else if scan.first > scan.last
            || scan.last > 63
            || scan.low > 13
            || (scan.first == 0) != (scan.last == 0)
            || scan.first > 0 && count > 1
        {
            return Err(damaged());
        }
        let needs_dc = scan.first == 0 && scan.high == 0;
        let needs_ac = scan.last > 0;
        let missing = || fail("the JPEG image lacks a Huffman table");
        let mut members = Vec::with_capacity(count);
        for index in 0..count {
            let id = segment[1 + 2 * index];
            let tables = segment[2 + 2 * index];
            let position = frame
                .components
                .iter()
                .position(|c| c.id == id)
                .ok_or_else(damaged)?;
            let component = &mut frame.components[position];
            if component.quantization.is_none() {
                component.quantization = self.quantization[component.table];
                if component.quantization.is_none() {
                    return Err(fail("the JPEG image lacks a quantization table"));
                }
            }
            component.prediction = 0;
            let dc = match needs_dc {
                true => Some(
                    self.dc
                        .get((tables >> 4) as usize)
                        .and_then(Option::as_ref)
                        .ok_or_else(missing)?,
                ),
                false => None,
            };
            let ac = match needs_ac {
                true => Some(
                    self.ac
                        .get((tables & 15) as usize)
                        .and_then(Option::as_ref)
                        .ok_or_else(missing)?,
                ),
                false => None,
            };
            members.push((position, dc, ac));
        }
        let interval = self.interval;
        let mut done = 0;
        if count == 1 {
            /* A scan of one component goes through its blocks row by row. */
            let (position, dc, ac) = members[0];
            let (wide, high) = frame.blocks(&frame.components[position]);
            let component = &mut frame.components[position];
            for row in 0..high {
                for column in 0..wide {
                    let at = (row * component.blocks_wide + column) * 64;
                    scan.block(
                        &mut component.coefficients[at..at + 64],
                        &mut component.prediction,
                        dc,
                        ac,
                    )?;
                    done += 1;
                    if interval > 0 && done % interval == 0 && done < wide * high {
                        scan.restart();
                        component.prediction = 0;
                    }
                }
            }
        } else {
            let total = frame.mcus_wide * frame.mcus_high;
            for mcu_row in 0..frame.mcus_high {
                for mcu_column in 0..frame.mcus_wide {
                    for &(position, dc, ac) in &members {
                        let component = &mut frame.components[position];
                        for v in 0..component.v {
                            for h in 0..component.h {
                                let row = mcu_row * component.v + v;
                                let column = mcu_column * component.h + h;
                                let at = (row * component.blocks_wide + column) * 64;
                                scan.block(
                                    &mut component.coefficients[at..at + 64],
                                    &mut component.prediction,
                                    dc,
                                    ac,
                                )?;
                            }
                        }
                    }
                    done += 1;
                    if interval > 0 && done % interval == 0 && done < total {
                        scan.restart();
                        for &(position, _, _) in &members {
                            frame.components[position].prediction = 0;
                        }
                    }
                }
            }
        }
        self.scans += 1;
        Ok(scan.reader.position)
    }

    fn finish(self) -> Result<Image, HostError> {
        let Some(frame) = self.frame else {
            return Err(fail("the JPEG image has no frame"));
        };
        if self.scans == 0 {
            return Err(fail("the JPEG image has no scan"));
        }
        let planes: Vec<Vec<u8>> = frame
            .components
            .iter()
            .map(|component| plane(&frame, component))
            .collect();
        let (width, height) = (frame.width, frame.height);
        let components = &frame.components;
        let rgb = components.len() == 3
            && (self.transform == Some(0) || components.iter().map(|c| c.id).eq(*b"RGB"));
        /* Each component's sample for each column and row of the image. */
        let columns: Vec<Vec<usize>> = components
            .iter()
            .map(|c| (0..width).map(|x| x * c.h / frame.h_most).collect())
            .collect();
        let rows: Vec<Vec<usize>> = components
            .iter()
            .map(|c| {
                (0..height)
                    .map(|y| y * c.v / frame.v_most * c.blocks_wide * 8)
                    .collect()
            })
            .collect();
        let sample = |index: usize, x: usize, y: usize| {
            planes[index][rows[index][y] + columns[index][x]] as i32
        };
        let mut pixels = vec![255; width * height * 4];
        for y in 0..height {
            for x in 0..width {
                let color = match components.len() {
                    1 => [sample(0, x, y) as u8; 3],
                    3 if rgb => [0, 1, 2].map(|index| sample(index, x, y) as u8),
                    3 => to_rgb(sample(0, x, y), sample(1, x, y), sample(2, x, y)),
                    _ => {
                        let black = sample(3, x, y);
                        let ink = if self.transform == Some(2) {
                            to_rgb(sample(0, x, y), sample(1, x, y), sample(2, x, y))
                                .map(|value| 255 - value as i32)
                        } else {
                            [0, 1, 2].map(|index| sample(index, x, y))
                        };
                        ink.map(|value| times(value, black))
                    }
                };
                pixels[(y * width + x) * 4..][..3].copy_from_slice(&color);
            }
        }
        Ok(orient(
            Image {
                width,
                height,
                pixels,
            },
            self.orientation,
        ))
    }
}

/** `a` times `b` over 255, rounded, for inks of Adobe's inverted CMYK. */
fn times(a: i32, b: i32) -> u8 {
    let product = a * b + 128;
    ((product + (product >> 8)) >> 8) as u8
}

/** YCbCr to RGB, as JFIF defines it and libjpeg rounds it. */
fn to_rgb(y: i32, cb: i32, cr: i32) -> [u8; 3] {
    let (cb, cr) = (cb - 128, cr - 128);
    let red = y + ((91881 * cr + 32768) >> 16);
    let green = y + ((-22554 * cb - 46802 * cr + 32768) >> 16);
    let blue = y + ((116130 * cb + 32768) >> 16);
    [red, green, blue].map(|value| value.clamp(0, 255) as u8)
}

/**
A component's samples, row by row, its blocks through the inverse DCT;
blocks no scan reached, and padding, stay gray.
*/
fn plane(frame: &Frame, component: &Component) -> Vec<u8> {
    let stride = component.blocks_wide * 8;
    let mut samples = vec![128; stride * component.blocks_high * 8];
    let Some(table) = &component.quantization else {
        return samples;
    };
    let (wide, high) = frame.blocks(component);
    for row in 0..high {
        for column in 0..wide {
            let at = (row * component.blocks_wide + column) * 64;
            let out = &mut samples[row * 8 * stride + column * 8..];
            idct(&component.coefficients[at..at + 64], table, out, stride);
        }
    }
    samples
}

/** `value` over 2^bits, rounded. */
fn descale(value: i64, bits: u32) -> i64 {
    (value + (1 << (bits - 1))) >> bits
}

fn clamp_sample(value: i64) -> u8 {
    (value + 128).clamp(0, 255) as u8
}

/**
One dimension of libjpeg's accurate integer inverse DCT (jidctint.c):
eight inputs to eight outputs, scaled up by 2^13.
*/
fn butterfly(x: [i64; 8]) -> [i64; 8] {
    let z1 = (x[2] + x[6]) * 4433;
    let even2 = z1 + x[6] * -15137;
    let even3 = z1 + x[2] * 6270;
    let even0 = (x[0] + x[4]) << 13;
    let even1 = (x[0] - x[4]) << 13;
    let (sum0, sum3) = (even0 + even3, even0 - even3);
    let (sum1, sum2) = (even1 + even2, even1 - even2);
    let (t0, t1, t2, t3) = (x[7], x[5], x[3], x[1]);
    let (z1, z2, z3, z4) = (t0 + t3, t1 + t2, t0 + t2, t1 + t3);
    let z5 = (z3 + z4) * 9633;
    let (z1, z2) = (z1 * -7373, z2 * -20995);
    let (z3, z4) = (z3 * -16069 + z5, z4 * -3196 + z5);
    let odd0 = t0 * 2446 + z1 + z3;
    let odd1 = t1 * 16819 + z2 + z4;
    let odd2 = t2 * 25172 + z2 + z3;
    let odd3 = t3 * 12299 + z1 + z4;
    [
        sum0 + odd3,
        sum1 + odd2,
        sum2 + odd1,
        sum3 + odd0,
        sum3 - odd0,
        sum2 - odd1,
        sum1 - odd2,
        sum0 - odd3,
    ]
}

/** A block's quantized coefficients to samples, written `stride` apart. */
fn idct(block: &[i16], table: &[i32; 64], out: &mut [u8], stride: usize) {
    const BITS: u32 = 13;
    const PASS: u32 = 2;
    let mut work = [0i64; 64];
    for column in 0..8 {
        let input = |row: usize| block[row * 8 + column] as i64 * table[row * 8 + column] as i64;
        if (1..8).all(|row| block[row * 8 + column] == 0) {
            let dc = input(0) << PASS;
            for row in 0..8 {
                work[row * 8 + column] = dc;
            }
            continue;
        }
        let values = butterfly([0, 1, 2, 3, 4, 5, 6, 7].map(input));
        for (row, value) in values.iter().enumerate() {
            work[row * 8 + column] = descale(*value, BITS - PASS);
        }
    }
    for row in 0..8 {
        let line: [i64; 8] = work[row * 8..row * 8 + 8].try_into().expect("eight values");
        let target = &mut out[row * stride..row * stride + 8];
        if line[1..].iter().all(|&value| value == 0) {
            target.fill(clamp_sample(descale(line[0], PASS + 3)));
            continue;
        }
        for (column, value) in butterfly(line).iter().enumerate() {
            target[column] = clamp_sample(descale(*value, BITS + PASS + 3));
        }
    }
}

/** An image turned and mirrored as an EXIF orientation says, 1 to 8. */
fn orient(image: Image, orientation: u8) -> Image {
    if !(2..=8).contains(&orientation) {
        return image;
    }
    let (w, h) = (image.width, image.height);
    let (width, height) = if orientation >= 5 { (h, w) } else { (w, h) };
    let mut pixels = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let (from_x, from_y) = match orientation {
                2 => (w - 1 - x, y),
                3 => (w - 1 - x, h - 1 - y),
                4 => (x, h - 1 - y),
                5 => (y, x),
                6 => (y, h - 1 - x),
                7 => (w - 1 - y, h - 1 - x),
                _ => (w - 1 - y, x),
            };
            let from = (from_y * w + from_x) * 4;
            pixels[(y * width + x) * 4..][..4].copy_from_slice(&image.pixels[from..from + 4]);
        }
    }
    Image {
        width,
        height,
        pixels,
    }
}

/** Decodes a JPEG image of at most `most` pixels. */
pub fn decode(bytes: &[u8], most: usize) -> Result<Image, HostError> {
    if !is_jpeg(bytes) {
        return Err(fail("the data is not a JPEG image"));
    }
    let mut decoder = Decoder {
        orientation: 1,
        ..Decoder::default()
    };
    let mut at = 2;
    while let Some((marker, start)) = next_marker(bytes, at) {
        if marker == 0xD9 {
            break;
        }
        if matches!(marker, 0x01 | 0xD0..=0xD8) {
            at = start;
            continue;
        }
        let length = number(bytes, start)?;
        let end = start + length;
        if length < 2 || end > bytes.len() {
            return Err(fail("the JPEG image ends early"));
        }
        let segment = &bytes[start + 2..end];
        match marker {
            0xC0 | 0xC1 => decoder.frame(segment, false, most)?,
            0xC2 => decoder.frame(segment, true, most)?,
            marker if starts_frame(marker) => {
                return Err(fail(
                    "JPEG images coded arithmetically, losslessly, or hierarchically are not supported",
                ));
            }
            0xC4 => decoder.huffman(segment)?,
            0xDB => decoder.quantization(segment)?,
            0xDD => decoder.interval = number(segment, 0)?,
            0xDA => {
                at = decoder.scan(segment, bytes, end)?;
                continue;
            }
            0xE1 => {
                if let Some(orientation) = orientation(segment) {
                    decoder.orientation = orientation;
                }
            }
            0xEE if segment.len() >= 12 && segment.starts_with(b"Adobe") => {
                decoder.transform = Some(segment[11]);
            }
            _ => {}
        }
        at = end;
    }
    decoder.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    Made by scripts/jpeg_fixtures.sayon with libjpeg-turbo: a picture of
    45 by 29 pixels in each sampling and kind of color, and the pixels
    `djpeg -nosmooth -dct int` decodes each to.
    */
    const J420: &[u8] = include_bytes!("../tests/jpeg/420.jpg");
    const J444: &[u8] = include_bytes!("../tests/jpeg/444.jpg");
    const GRAY: &[u8] = include_bytes!("../tests/jpeg/gray.jpg");
    const PICTURES: [(&str, &[u8], &[u8]); 6] = [
        ("420", J420, include_bytes!("../tests/jpeg/420.ppm")),
        (
            "422",
            include_bytes!("../tests/jpeg/422.jpg"),
            include_bytes!("../tests/jpeg/422.ppm"),
        ),
        ("444", J444, include_bytes!("../tests/jpeg/444.ppm")),
        ("gray", GRAY, include_bytes!("../tests/jpeg/gray.pgm")),
        (
            "rgb",
            include_bytes!("../tests/jpeg/rgb.jpg"),
            include_bytes!("../tests/jpeg/rgb.ppm"),
        ),
        (
            "restart",
            include_bytes!("../tests/jpeg/restart.jpg"),
            include_bytes!("../tests/jpeg/restart.ppm"),
        ),
    ];

    /** The size and the red, green, and blue of a binary PPM or PGM file. */
    fn netpbm(bytes: &[u8]) -> (usize, usize, Vec<u8>) {
        let mut fields = Vec::new();
        let mut at = 0;
        while fields.len() < 4 {
            while bytes[at].is_ascii_whitespace() {
                at += 1;
            }
            let start = at;
            while !bytes[at].is_ascii_whitespace() {
                at += 1;
            }
            fields.push(String::from_utf8_lossy(&bytes[start..at]).to_string());
        }
        let data = &bytes[at + 1..];
        let colors = if fields[0] == "P5" {
            data.iter().flat_map(|&gray| [gray; 3]).collect()
        } else {
            data.to_vec()
        };
        (
            fields[1].parse().unwrap(),
            fields[2].parse().unwrap(),
            colors,
        )
    }

    fn colors(image: &Image) -> Vec<u8> {
        image
            .pixels
            .chunks(4)
            .flat_map(|p| [p[0], p[1], p[2]])
            .collect()
    }

    fn pixel(image: &Image, x: usize, y: usize) -> [u8; 4] {
        image.pixels[(y * image.width + x) * 4..][..4]
            .try_into()
            .unwrap()
    }

    #[test]
    fn baseline_images_decode_to_the_pixels_libjpeg_gives() {
        for (name, jpeg, expected) in PICTURES {
            let image = decode(jpeg, usize::MAX).unwrap_or_else(|e| panic!("{name}: {e:?}"));
            let (width, height, pixels) = netpbm(expected);
            assert_eq!((image.width, image.height), (width, height), "{name}");
            let got = colors(&image);
            let differences = got.iter().zip(&pixels).filter(|(a, b)| a != b).count();
            assert_eq!(
                differences,
                0,
                "{name}: {differences} of {} values differ",
                pixels.len()
            );
            assert!(image.pixels.chunks(4).all(|p| p[3] == 255), "{name}");
        }
    }

    #[test]
    fn progressive_scans_and_restarts_give_the_same_pixels() {
        let same: [(&[u8], &[u8]); 4] = [
            (include_bytes!("../tests/jpeg/420-progressive.jpg"), J420),
            (
                include_bytes!("../tests/jpeg/420-progressive-restart.jpg"),
                J420,
            ),
            (include_bytes!("../tests/jpeg/444-progressive.jpg"), J444),
            (include_bytes!("../tests/jpeg/gray-progressive.jpg"), GRAY),
        ];
        for (index, (progressive, baseline)) in same.into_iter().enumerate() {
            let a = decode(progressive, usize::MAX).unwrap_or_else(|e| panic!("{index}: {e:?}"));
            let b = decode(baseline, usize::MAX).unwrap();
            assert_eq!((a.width, a.height), (b.width, b.height), "{index}");
            assert!(
                a.pixels == b.pixels,
                "progressive image {index} differs from its baseline"
            );
        }
    }

    /**
    A JPEG image with an EXIF segment right after its start, giving an
    orientation, in either byte order, as cameras write it.
    */
    fn oriented(jpeg: &[u8], orientation: u8, big: bool) -> Vec<u8> {
        let mut exif = b"Exif\0\0".to_vec();
        if big {
            exif.extend_from_slice(b"MM\0\x2a\0\0\0\x08");
            exif.extend_from_slice(&[0, 1, 0x01, 0x12, 0, 3, 0, 0, 0, 1, 0, orientation, 0, 0]);
        } else {
            exif.extend_from_slice(b"II\x2a\0\x08\0\0\0");
            exif.extend_from_slice(&[1, 0, 0x12, 0x01, 3, 0, 1, 0, 0, 0, orientation, 0, 0, 0]);
        }
        exif.extend_from_slice(&[0, 0, 0, 0]);
        let mut out = vec![0xFF, 0xD8, 0xFF, 0xE1, 0, (exif.len() + 2) as u8];
        out.extend_from_slice(&exif);
        out.extend_from_slice(&jpeg[2..]);
        out
    }

    #[test]
    fn photos_are_turned_as_their_orientation_says() {
        let plain = decode(J444, usize::MAX).unwrap();
        let (w, h) = (plain.width, plain.height);
        /*
        Where the top left pixel shown, and the one right of it, are kept,
        by EXIF's description of each orientation: which side the first
        row kept is shown at, and which the first column.
        */
        let corners = [
            ((0, 0), (1, 0)),
            ((w - 1, 0), (w - 2, 0)),
            ((w - 1, h - 1), (w - 2, h - 1)),
            ((0, h - 1), (1, h - 1)),
            ((0, 0), (0, 1)),
            ((0, h - 1), (0, h - 2)),
            ((w - 1, h - 1), (w - 1, h - 2)),
            ((w - 1, 0), (w - 1, 1)),
        ];
        for (index, (first, second)) in corners.into_iter().enumerate() {
            let orientation = index as u8 + 1;
            for big in [true, false] {
                let bytes = oriented(J444, orientation, big);
                let image = decode(&bytes, usize::MAX).unwrap();
                let shown = if orientation >= 5 { (h, w) } else { (w, h) };
                assert_eq!(
                    (image.width, image.height),
                    shown,
                    "orientation {orientation}"
                );
                assert_eq!(size(&bytes).unwrap(), shown, "orientation {orientation}");
                assert_eq!(
                    pixel(&image, 0, 0),
                    pixel(&plain, first.0, first.1),
                    "orientation {orientation}"
                );
                assert_eq!(
                    pixel(&image, 1, 0),
                    pixel(&plain, second.0, second.1),
                    "orientation {orientation}"
                );
            }
        }
    }

    #[test]
    fn sizes_come_from_the_header() {
        assert_eq!(size(J420).unwrap(), (45, 29));
        assert_eq!(
            size(include_bytes!("../tests/jpeg/gray-progressive.jpg")).unwrap(),
            (45, 29)
        );
        assert!(size(b"GIF89a, not a JPEG image").is_err());
        assert!(size(&J420[..12]).is_err());
    }

    #[test]
    fn what_is_not_a_jpeg_image_sayon_reads_fails() {
        assert!(decode(b"GIF89a, not a JPEG image", usize::MAX).is_err());
        assert!(decode(&J420[..40], usize::MAX).is_err());
        assert!(decode(J420, 45 * 29 - 1).is_err());
        let frame = J420
            .windows(2)
            .position(|pair| pair == [0xFF, 0xC0])
            .unwrap();
        let mut arithmetic = J420.to_vec();
        arithmetic[frame + 1] = 0xC9;
        let refused = decode(&arithmetic, usize::MAX).err().unwrap();
        assert!(refused.1.contains("not supported"), "{}", refused.1);
        let mut deep = J420.to_vec();
        deep[frame + 4] = 12;
        assert!(
            decode(&deep, usize::MAX)
                .err()
                .unwrap()
                .1
                .contains("12-bit")
        );
    }

    #[test]
    fn coded_data_cut_short_leaves_the_rest_gray() {
        let image = decode(&J420[..J420.len() - 300], usize::MAX).unwrap();
        assert_eq!((image.width, image.height), (45, 29));
        let whole = decode(J420, usize::MAX).unwrap();
        assert_eq!(pixel(&image, 0, 0), pixel(&whole, 0, 0));
    }
}
