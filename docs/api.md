# The standard library's API

Every export of the standard library, read from its declarations in `stdlib/`, with the first paragraph of the comment above each. `sayon run scripts/api_inventory.sayon` writes this page, and `cargo test` checks that it is current, so it always says what the library has. 13 modules hold 382 exports, 382 of them with a description.

A method is a function whose name starts with its type's, called on a value of the type: `set_add(set, value)` is written `set.add(value)`.

| Module | Exports | For |
|---|---|---|
| [`standard/collections.sayon`](#standardcollectionssayon) | 57 | Sorting arrays, and the Set, Queue, Stack, List, and Map collections. |
| [`standard/crypto.sayon`](#standardcryptosayon) | 8 | Hashing, signing, passwords, and secure random bytes: SHA-256 digests, HMAC-SHA256 codes, passwords kept as PBKDF2 hashes, and bytes from the system's secure source, fit for keys and tokens, unlike `math.random`. |
| [`standard/filesystem.sayon`](#standardfilesystemsayon) | 18 | Files, directories, and paths. |
| [`standard/graphics.sayon`](#standardgraphicssayon) | 54 | Drawing, and windows to show it in. |
| [`standard/io.sayon`](#standardiosayon) | 30 | Console and file input and output. |
| [`standard/json.sayon`](#standardjsonsayon) | 25 | JSON: reading text into values, and writing values as text. |
| [`standard/math.sayon`](#standardmathsayon) | 30 | Mathematical functions and constants. |
| [`standard/memory.sayon`](#standardmemorysayon) | 8 | Reading and writing the memory that C code shares with a program. |
| [`standard/network.sayon`](#standardnetworksayon) | 42 | TCP connections and listeners, UDP datagrams, TLS, and an HTTP client. |
| [`standard/os.sayon`](#standardossayon) | 15 | The program's environment. |
| [`standard/strings.sayon`](#standardstringssayon) | 31 | Operations on text. Positions and lengths count Unicode characters (code points); `graphemes` works with what readers see as characters, and `bytes` with the UTF-8 encoding. |
| [`standard/threads.sayon`](#standardthreadssayon) | 16 | Tasks that run together, and channels between them. |
| [`standard/time.sayon`](#standardtimesayon) | 48 | Clocks, durations, and dates. |

## standard/collections.sayon

Sorting arrays, and the Set, Queue, Stack, List, and Map collections.

| Export | Declaration | Description |
|---|---|---|
| `sort` | `function sort(update items)` | Sorts an array in place, smallest first. Elements are numbers or strings. Sorting is stable: equal elements keep their order. It takes time proportional to n log n for n elements, and room for half of them: the sort works with copies of the first half, so sorting strings copies half their text for a while. |
| `sort_by` | `function sort_by(update items, update before)` | Sorts an array in place by a comparison: `before(a, b)` is true when `a` must come before `b`. Sorting is stable, so elements for which neither comes before the other keep their order. It needs the same room as `sort`. An error in `before` stops the sort; the elements are then all still in the array, in some order. |
| `reverse` | `function reverse(update items)` | Reverses an array in place. |
| `Set` | `type Set<T> = [members: Dict<T, boolean>]` | Values without duplicates, in the order they were first added. Values are compared like dictionary keys, so they must be valid keys. Adding a value and looking one up take constant time on average; removing one takes time proportional to the set's size, since the values after it keep their order. Sets are equal (`==`) when they have the same values, in any order. |
| `new_set` | `function new_set<T>() -> Set<T>` | A new, empty set. |
| `set_add` | `function set_add<T>(update set: Set<T>, own value: T) -> boolean` | Method `add` of `Set`. Adds a value, returning false when the set already had it. |
| `set_contains` | `function set_contains<T>(set: Set<T>, value: T) -> boolean` | Method `contains` of `Set`. Whether the set has the value. |
| `set_remove` | `function set_remove<T>(update set: Set<T>, value: T) -> boolean` | Method `remove` of `Set`. Removes a value, returning whether the set had it. |
| `set_size` | `function set_size<T>(set: Set<T>) -> integer` | Method `size` of `Set`. How many values the set has. |
| `set_values` | `function set_values<T>(set: Set<T>) -> Array<T>` | Method `values` of `Set`. The values, in the order they were first added. |
| `Queue` | `type Queue<T> = [front: Array<T>, back: Array<T>]` | Values taken out in the order they were put in. Adding and taking take constant time on average: a value is moved once, from the back to the front. How a queue is kept depends on the order of its changes, so `==` can find two queues of the same values different: compare their `values()`. |
| `new_queue` | `function new_queue<T>() -> Queue<T>` | A new, empty queue. |
| `queue_add` | `function queue_add<T>(update queue: Queue<T>, own value: T)` | Method `add` of `Queue`. Puts a value at the end of the queue. |
| `queue_take` | `function queue_take<T>(update queue: Queue<T>) -> T or nothing` | Method `take` of `Queue`. Takes the value at the front of the queue, or nothing when it is empty. |
| `queue_peek` | `function queue_peek<T>(queue: Queue<T>) -> T or nothing` | Method `peek` of `Queue`. A copy of the value at the front of the queue, the one `take` would take next, or nothing when the queue is empty. |
| `queue_size` | `function queue_size<T>(queue: Queue<T>) -> integer` | Method `size` of `Queue`. How many values wait in the queue. |
| `queue_values` | `function queue_values<T>(queue: Queue<T>) -> Array<T>` | Method `values` of `Queue`. Copies of the values, from the front: the order `take` would take them. |
| `Stack` | `type Stack<T> = [items: Array<T>]` | Values taken out in the opposite order they were put in: `pop` takes the value pushed last. Pushing and popping take constant time on average. Stacks are equal (`==`) when they have the same values in the same order. |
| `new_stack` | `function new_stack<T>() -> Stack<T>` | A new, empty stack. |
| `stack_push` | `function stack_push<T>(update stack: Stack<T>, own value: T)` | Method `push` of `Stack`. Puts a value on top of the stack. |
| `stack_pop` | `function stack_pop<T>(update stack: Stack<T>) -> T or nothing` | Method `pop` of `Stack`. Takes the value on top of the stack, or nothing when it is empty. |
| `stack_peek` | `function stack_peek<T>(stack: Stack<T>) -> T or nothing` | Method `peek` of `Stack`. A copy of the value on top of the stack, the one `pop` would take next, or nothing when the stack is empty. |
| `stack_size` | `function stack_size<T>(stack: Stack<T>) -> integer` | Method `size` of `Stack`. How many values the stack holds. |
| `stack_values` | `function stack_values<T>(stack: Stack<T>) -> Array<T>` | Method `values` of `Stack`. Copies of the values, from the top down: the order `pop` would take them. |
| `List` | `type List<T> = [items: Array<T>, successors: Array<integer>, predecessors: Array<integer>, handles: Array<integer>, slots: Array<integer>, generations: Array<integer>, spare: Array<integer>, head: integer, tail: integer]` | Values in a sequence, like an array, where adding a value or taking one out anywhere takes constant time, as do reaching a value through its position and moving to its neighbors. Each value added gets a `Position`, which names it until it is taken out, however the list changes around it. How a list is kept depends on the order of its changes, so `==` can find two lists of the same values different: compare their `values()`. |
| `Position` | `type Position = [handle: integer, generation: integer]` | Where a value is in a list. A position names its value until the value is taken out; after that, using it is a `ValueError`. A position belongs to the list that gave it out: with another list it names another value, or none. |
| `new_list` | `function new_list<T>() -> List<T>` | A new, empty list. |
| `list_add_first` | `function list_add_first<T>(update list: List<T>, own value: T) -> Position` | Method `add_first` of `List`. Adds a value before the first one, returning its position. |
| `list_add_last` | `function list_add_last<T>(update list: List<T>, own value: T) -> Position` | Method `add_last` of `List`. Adds a value after the last one, returning its position. |
| `list_add_before` | `function list_add_before<T>(update list: List<T>, position: Position, own value: T) -> Position` | Method `add_before` of `List`. Adds a value just before the one at a position, returning its position. |
| `list_add_after` | `function list_add_after<T>(update list: List<T>, position: Position, own value: T) -> Position` | Method `add_after` of `List`. Adds a value just after the one at a position, returning its position. |
| `list_take` | `function list_take<T>(update list: List<T>, position: Position) -> T` | Method `take` of `List`. Takes out the value at a position. |
| `list_take_first` | `function list_take_first<T>(update list: List<T>) -> T or nothing` | Method `take_first` of `List`. Takes out the first value, or gives nothing when the list is empty. |
| `list_take_last` | `function list_take_last<T>(update list: List<T>) -> T or nothing` | Method `take_last` of `List`. Takes out the last value, or gives nothing when the list is empty. |
| `list_get` | `function list_get<T>(list: List<T>, position: Position) -> T` | Method `get` of `List`. A copy of the value at a position. |
| `list_set` | `function list_set<T>(update list: List<T>, position: Position, own value: T)` | Method `set` of `List`. Puts a value in place of the one at a position, which then names the new value. |
| `list_first` | `function list_first<T>(list: List<T>) -> Position or nothing` | Method `first` of `List`. The position of the first value, or nothing when the list is empty. |
| `list_last` | `function list_last<T>(list: List<T>) -> Position or nothing` | Method `last` of `List`. The position of the last value, or nothing when the list is empty. |
| `list_after` | `function list_after<T>(list: List<T>, position: Position) -> Position or nothing` | Method `after` of `List`. The position of the value after the one at a position, or nothing after the last. |
| `list_before` | `function list_before<T>(list: List<T>, position: Position) -> Position or nothing` | Method `before` of `List`. The position of the value before the one at a position, or nothing before the first. |
| `list_find` | `function list_find<T>(list: List<T>, value: T) -> Position or nothing` | Method `find` of `List`. The position of the first value equal to `value`, or nothing when no value is. It takes time proportional to the list's size. |
| `list_size` | `function list_size<T>(list: List<T>) -> integer` | Method `size` of `List`. How many values the list holds. |
| `list_values` | `function list_values<T>(list: List<T>) -> Array<T>` | Method `values` of `List`. Copies of the values, from the first to the last. |
| `Map` | `type Map<K, V> = [node_keys: Array<K>, node_values: Array<V>, lower: Array<integer>, higher: Array<integer>, parents: Array<integer>, heights: Array<integer>, root: integer]` | Keys with values, kept in the order of their keys, smallest first, where a dictionary keeps the order they were added in. Keys are numbers or strings, and cannot be NaN. Looking a key up, adding one, and removing one take time proportional to log n for n keys, as do finding the first and last keys and the keys around a key; listing them takes time proportional to n. How a map is kept depends on the order of its changes, so `==` can find two maps of the same keys and values different: compare their `keys()` and `values()`. |
| `new_map` | `function new_map<K, V>() -> Map<K, V>` | A new, empty map. |
| `map_set` | `function map_set<K, V>(update map: Map<K, V>, own key: K, own value: V)` | Method `set` of `Map`. Sets the value of a key, adding the key when the map lacks it. |
| `map_get` | `function map_get<K, V>(map: Map<K, V>, key: K) -> V or nothing` | Method `get` of `Map`. A copy of a key's value, or nothing when the map lacks the key. |
| `map_contains` | `function map_contains<K, V>(map: Map<K, V>, key: K) -> boolean` | Method `contains` of `Map`. Whether the map has a key. |
| `map_remove` | `function map_remove<K, V>(update map: Map<K, V>, key: K) -> V or nothing` | Method `remove` of `Map`. Removes a key, returning its value, or nothing when the map lacked it. |
| `map_size` | `function map_size<K, V>(map: Map<K, V>) -> integer` | Method `size` of `Map`. How many keys the map has. |
| `map_keys` | `function map_keys<K, V>(map: Map<K, V>) -> Array<K>` | Method `keys` of `Map`. Copies of the keys, smallest first. |
| `map_values` | `function map_values<K, V>(map: Map<K, V>) -> Array<V>` | Method `values` of `Map`. Copies of the values, in the order of their keys. |
| `map_first` | `function map_first<K, V>(map: Map<K, V>) -> K or nothing` | Method `first` of `Map`. The smallest key, or nothing when the map is empty. |
| `map_last` | `function map_last<K, V>(map: Map<K, V>) -> K or nothing` | Method `last` of `Map`. The largest key, or nothing when the map is empty. |
| `map_before` | `function map_before<K, V>(map: Map<K, V>, key: K) -> K or nothing` | Method `before` of `Map`. The largest key smaller than `key`, or nothing when no key is. |
| `map_after` | `function map_after<K, V>(map: Map<K, V>, key: K) -> K or nothing` | Method `after` of `Map`. The smallest key larger than `key`, or nothing when no key is. |
| `map_between` | `function map_between<K, V>(map: Map<K, V>, low: K, high: K) -> Array<K>` | Method `between` of `Map`. Copies of the keys from `low` up to, but not including, `high`, smallest first. |

## standard/crypto.sayon

Hashing, signing, passwords, and secure random bytes: SHA-256 digests, HMAC-SHA256 codes, passwords kept as PBKDF2 hashes, and bytes from the system's secure source, fit for keys and tokens, unlike `math.random`.

| Export | Declaration | Description |
|---|---|---|
| `sha256` | `function sha256(data: Array<byte>) -> Array<byte>` | The SHA-256 digest of bytes (FIPS 180-4): 32 bytes. |
| `hmac_sha256` | `function hmac_sha256(key: Array<byte>, message: Array<byte>) -> Array<byte>` | The HMAC-SHA256 code of a message under a key (RFC 2104): 32 bytes that only someone with the key could make, for telling that a message, such as a cookie, is one you made. Compare codes with `same`. |
| `pbkdf2_sha256` | `function pbkdf2_sha256(password: Array<byte>, salt: Array<byte>, rounds: integer, length: integer) -> Array<byte>` | PBKDF2 with HMAC-SHA256 (RFC 8018): `length` bytes derived from a password and a salt by `rounds` rounds of HMAC-SHA256, slow on purpose, so that guessing passwords is slow too. Rounds are 1 to 10,000,000 and the length 1 to 1,024 bytes; others raise `ValueError`. `hash_password` chooses them for keeping passwords. |
| `hash_password` | `function hash_password(password: string) -> string` | A password made safe to keep: `pbkdf2-sha256$<rounds>$<salt>$<hash>`, from 600,000 rounds of PBKDF2 and a new random salt, so that the same password never gives the same text twice. It takes a moment, on purpose. `check_password` tells whether a password is the one it was made from. |
| `check_password` | `function check_password(password: string, kept: string) -> boolean` | Whether `password` is the one `hash_password` made `kept` from, comparing the hashes in constant time; false for text `hash_password` does not make. |
| `random_bytes` | `function random_bytes(count: integer) -> Array<byte>` | `count` bytes from the system's secure random source, 1 MiB at most at a time; a negative or larger count raises `ValueError`. |
| `hex` | `function hex(data: Array<byte>) -> string` | Bytes as lowercase hexadecimal, two digits for each. |
| `same` | `function same(first: Array<byte>, second: Array<byte>) -> boolean` | Whether two byte arrays are equal, looking at every byte rather than stopping at the first that differs, for comparing secrets such as tokens. |

## standard/filesystem.sayon

Files, directories, and paths.

| Export | Declaration | Description |
|---|---|---|
| `exists` | `function exists(path: string) -> boolean` | Whether something is at `path`, following symbolic links. A path that cannot be looked at, as for lack of permission, counts as there. |
| `is_file` | `function is_file(path: string) -> boolean` | Whether `path` is a file, following symbolic links. |
| `is_directory` | `function is_directory(path: string) -> boolean` | Whether `path` is a directory, following symbolic links. |
| `list` | `function list(path: string) -> Array<string>` | The names in a directory, sorted. |
| `walk` | `function walk(path: string) -> Array<string>` | Every file and directory below a directory, as paths relative to it joined with "/", such as "docs/notes.txt". Each directory comes just before what it holds, and the entries of a directory are sorted by name. A symbolic link is listed but not followed, so a walk always ends. |
| `is_link` | `function is_link(path: string) -> boolean` | Whether the path itself is a symbolic link, whatever it points to. |
| `size` | `function size(path: string) -> integer` | A file's size in bytes. |
| `create_directory` | `function create_directory(path: string)` | Creates a directory and any missing parent directories. |
| `remove` | `function remove(path: string)` | Removes a file or an empty directory. |
| `rename` | `function rename(from: string, to: string)` | Moves a file or directory to `to`, replacing a file already there. |
| `copy` | `function copy(from: string, to: string)` | Copies a file's contents to `to`, replacing a file already there. |
| `join` | `function join(base: string, name: string) -> string` | Joins a directory and a name with one separator. |
| `name` | `function name(path: string) -> string` | The last component of a path: `name("docs/notes.txt")` is "notes.txt". |
| `parent` | `function parent(path: string) -> string` | The path without its last component: `parent("docs/notes.txt")` is "docs". A path from a root keeps it: `parent("/home/ada")` is "/home", and `parent("/home")` is "/"; on Windows `parent("C:\Users")` is "C:/". |
| `normalize` | `function normalize(path: string) -> string` | The path written plainly, without looking at the filesystem: one `/` between components, no `.` components, and each `..` taking away the component before it. `normalize("docs/./drafts/../notes.txt")` is "docs/notes.txt". A `..` at the start of a relative path stays, and one at a root goes: `normalize("../notes")` is "../notes", and `normalize("/..")` is "/". A relative path that comes to nothing is ".". When the component before a `..` is a symbolic link, the path can lead elsewhere than the normalized one does; `canonical` follows links. |
| `absolute` | `function absolute(path: string) -> string` | The path from a root, without looking at the filesystem: a relative path joined to the current directory, then normalized as `normalize` does. |
| `canonical` | `function canonical(path: string) -> string` | The name the system knows a file or directory by, as it writes it: the path from the root with every symbolic link followed, and on Windows with `\` and each name's letter case as it is on disk. Under WASI it is the path the program sees, from the root of the directories it was given. Fails with IOError when nothing is at `path`. |
| `extension` | `function extension(path: string) -> string` | The extension of the last component, without the dot, or "" when there is none: `extension("notes.txt")` is "txt", and `extension(".profile")` is "". |

## standard/graphics.sayon

Drawing, and windows to show it in.

| Export | Declaration | Description |
|---|---|---|
| `Color` | `type Color = [red: integer, green: integer, blue: integer, alpha: integer]` | A color: red, green, and blue, and how opaque it is, each 0 to 255. |
| `rgb` | `function rgb(red: integer, green: integer, blue: integer) -> Color` | An opaque color. |
| `rgba` | `function rgba(red: integer, green: integer, blue: integer, alpha: integer) -> Color` | A color that lets what is below show through, as `alpha` (0 to 255) says. |
| `BLACK` | `constant BLACK = Color(red = 0, green = 0, blue = 0, alpha = 255)` | Colors to draw with, opaque but for `TRANSPARENT`. |
| `WHITE` | `constant WHITE = Color(red = 255, green = 255, blue = 255, alpha = 255)` | Colors to draw with, opaque but for `TRANSPARENT`. |
| `RED` | `constant RED = Color(red = 255, green = 0, blue = 0, alpha = 255)` | Colors to draw with, opaque but for `TRANSPARENT`. |
| `GREEN` | `constant GREEN = Color(red = 0, green = 255, blue = 0, alpha = 255)` | Colors to draw with, opaque but for `TRANSPARENT`. |
| `BLUE` | `constant BLUE = Color(red = 0, green = 0, blue = 255, alpha = 255)` | Colors to draw with, opaque but for `TRANSPARENT`. |
| `TRANSPARENT` | `constant TRANSPARENT = Color(red = 0, green = 0, blue = 0, alpha = 0)` | Colors to draw with, opaque but for `TRANSPARENT`. |
| `Canvas` | `type Canvas = [pixels: HostStream, width: integer, height: integer, smooth: boolean]` | Pixels to draw on, `width` by `height`; circles, polygons, and lines on it are smooth where `smooth` is true. |
| `Point` | `type Point = [x: integer, y: integer]` | A point, in pixels from the top left corner. |
| `canvas` | `function canvas(width: integer, height: integer) -> Canvas` | A canvas of transparent pixels. Each side is 1 to 16384 pixels, and 67108864 pixels at most in all; other sizes fail with ValueError. |
| `image` | `function image(width: integer, height: integer, bytes: Array<byte>) -> Canvas` | A canvas of given pixels: red, green, blue, and alpha bytes for each, row by row from the top, as from `canvas.bytes()`. |
| `decode_image` | `function decode_image(bytes: Array<byte>) -> Canvas` | A canvas of the pixels of an image: a PNG image, of any of PNG's kinds of color, interlaced or not, its colors of 16 bits cut to 8; or a JPEG image, baseline or progressive, turned as a photo's EXIF orientation says. What is neither, or is damaged, fails with ImageError. |
| `load_image` | `function load_image(path: string) -> Canvas` | A canvas of the pixels of a PNG or JPEG image in a file. |
| `ImageSize` | `type ImageSize = [width: integer, height: integer]` | How wide and high an image is, in pixels. |
| `image_size` | `function image_size(bytes: Array<byte>) -> ImageSize` | The size of a PNG or JPEG image as `decode_image` gives it, turned as a photo's orientation says, read from the image's header alone, so that a program can refuse an image too large before decoding it. What is neither fails with ImageError. |
| `canvas_clear` | `function canvas_clear(update canvas: Canvas, color: Color)` | Method `clear` of `Canvas`. Makes every pixel `color`, replacing what was there. |
| `canvas_fill_rectangle` | `function canvas_fill_rectangle(update canvas: Canvas, x: integer, y: integer, width: integer, height: integer, color: Color)` | Method `fill_rectangle` of `Canvas`. Fills a rectangle with `color`, its top left corner at (x, y). |
| `canvas_stroke_rectangle` | `function canvas_stroke_rectangle(update canvas: Canvas, x: integer, y: integer, width: integer, height: integer, color: Color, thickness = 1)` | Method `stroke_rectangle` of `Canvas`. A rectangle's outline, `thickness` pixels wide, inside the rectangle. |
| `canvas_fill_circle` | `function canvas_fill_circle(update canvas: Canvas, x: integer, y: integer, radius: integer, color: Color)` | Method `fill_circle` of `Canvas`. A disc around the center of the pixel `x`, `y`. |
| `canvas_fill_polygon` | `function canvas_fill_polygon(update canvas: Canvas, points: Array<Point>, color: Color)` | Method `fill_polygon` of `Canvas`. The inside of a polygon, by the even-odd rule. |
| `canvas_line` | `function canvas_line(update canvas: Canvas, x1: integer, y1: integer, x2: integer, y2: integer, color: Color, thickness = 1)` | Method `line` of `Canvas`. A line from the center of one pixel to the center of another, `thickness` pixels wide, its ends cut square at the centers when smooth. |
| `canvas_draw` | `function canvas_draw(update canvas: Canvas, image: Canvas, x: integer, y: integer)` | Method `draw` of `Canvas`. Draws another canvas, such as an image, with its top left corner at `x`, `y`, over what is there. |
| `canvas_draw_scaled` | `function canvas_draw_scaled(update canvas: Canvas, image: Canvas, x: integer, y: integer, width: integer, height: integer)` | Method `draw_scaled` of `Canvas`. Draws another canvas scaled to fill the rectangle `width` by `height` pixels with its top left corner at `x`, `y`, over what is there. Made smaller, each pixel is the average of the pixels it covers, by how much of each; made larger, a blend of the nearest ones, by distance. Colors count as much as they are opaque. |
| `canvas_draw_part` | `function canvas_draw_part(update canvas: Canvas, image: Canvas, part_x: integer, part_y: integer, part_width: integer, part_height: integer, x: integer, y: integer, width: integer, height: integer)` | Method `draw_part` of `Canvas`. Draws a part of another canvas, `part_width` by `part_height` pixels from `part_x`, `part_y`, scaled to fill a rectangle as `draw_scaled` does, such as the middle square of a photo made small. A part reaching outside the image is cut to it. |
| `canvas_text` | `function canvas_text(update canvas: Canvas, text: string, x: integer, y: integer, size: integer, color: Color)` | Method `text` of `Canvas`. Draws text in the built-in font, its top left corner at `x`, `y`, each character in a cell of `size` pixels; a line break starts a line below. |
| `text_width` | `function text_width(text: string, size: integer) -> integer` | How wide text is in the built-in font: its longest line's characters times `size`. |
| `text_height` | `function text_height(text: string, size: integer) -> integer` | How high text is: its lines times `size`. |
| `canvas_pixel` | `function canvas_pixel(canvas: Canvas, x: integer, y: integer) -> Color` | Method `pixel` of `Canvas`. The color of a pixel; transparent outside the canvas. |
| `canvas_bytes` | `function canvas_bytes(canvas: Canvas) -> Array<byte>` | Method `bytes` of `Canvas`. The pixels, as `image` takes them. |
| `canvas_png` | `function canvas_png(canvas: Canvas) -> Array<byte>` | Method `png` of `Canvas`. The canvas as a PNG image, of 8-bit red, green, blue, and alpha. |
| `canvas_save_png` | `function canvas_save_png(canvas: Canvas, path: string)` | Method `save_png` of `Canvas`. Writes the canvas to a file as a PNG image. |
| `Font` | `type Font = [handle: HostStream, name: string]` | A font to write text in, and its name, such as "Arial Bold". Sizes are in pixels per em, as a web page's are: a font of size 16 is drawn as text of 16 pixels is in a browser. The runtime draws a TrueType font itself, its outlines smooth, without hinting or kerning, so that text in it is the same everywhere. |
| `decode_font` | `function decode_font(bytes: Array<byte>) -> Font` | The TrueType font in bytes, such as a `.ttf` file's, or the first of a collection (`.ttc`). Fonts of PostScript outlines (most `.otf` files), and what is not a font, fail with FontError. |
| `load_font` | `function load_font(path: string) -> Font` | The TrueType font in a file. |
| `system_font` | `function system_font(name = "sans-serif") -> Font` | One of the system's fonts, by its family's name, such as "Arial", or its full name, such as "Arial Bold"; "sans-serif", "serif", and "monospace" name the system's usual font of each kind. In a web page it is the browser's, which the browser draws. A font the system lacks fails with FontError. |
| `font_ascent` | `function font_ascent(font: Font, size: integer) -> integer` | Method `ascent` of `Font`. How far the font reaches above its baseline at `size`, in pixels. |
| `font_descent` | `function font_descent(font: Font, size: integer) -> integer` | Method `descent` of `Font`. How far it reaches below its baseline. |
| `font_line_height` | `function font_line_height(font: Font, size: integer) -> integer` | Method `line_height` of `Font`. How far one line's top is below the last's. |
| `font_width` | `function font_width(font: Font, text: string, size: integer) -> integer` | Method `width` of `Font`. How wide text is in the font at `size`: its longest line, in whole pixels. |
| `font_height` | `function font_height(font: Font, text: string, size: integer) -> integer` | Method `height` of `Font`. How high text is: from its first line's top to its last line's bottom. |
| `canvas_write` | `function canvas_write(update canvas: Canvas, font: Font, text: string, x: integer, y: integer, size: integer, color: Color)` | Method `write` of `Canvas`. Draws text in a font at `size`, the top left corner of its first line at `x`, `y`; a line break starts a line below. A character the font lacks shows as the font's glyph for those. |
| `Event` | `enum Event: closed, resized(width: integer, height: integer), key_down(key: string), key_up(key: string), pointer_down(x: integer, y: integer, button: integer), pointer_up(x: integer, y: integer, button: integer), pointer_moved(x: integer, y: integer), focused(focused: boolean), paused, resumed` | What happened to a window since the program last asked. A pointer is a mouse or a finger; its button is 1 for the primary one or a touch, 2 for the secondary, and 3 for the middle. Keys are named as web pages name them: "a" to "z", "0" to "9", " ", "Enter", "Escape", "Backspace", "Tab", "ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "Shift", "Control", "Alt", "Home", "End", "PageUp", "PageDown", "Insert", "Delete", "F1" to "F12", and "Unidentified" for the rest. |
| `Window` | `type Window = [handle: HostStream, open: boolean, next_frame: integer]` | A window, and when its next frame is due. |
| `FRAME` | `constant FRAME = 16_666_667` | The nanoseconds between frames: sixty a second. |
| `open` | `function open(title: string, width: integer, height: integer) -> Window` | Opens a window `width` by `height` points (the system's measure, which `window.scale()` turns into pixels), titled `title`. In a web page it is a canvas added to the page; in an Android app it is the app's screen, whatever its size. Its first event tells its size in pixels. Fails with GraphicsError where there is no screen, such as under WASI, in Node, or from `adb shell`. |
| `window_events` | `function window_events(update window: Window) -> Array<Event>` | Method `events` of `Window`. The events since the last call, in order. After `closed`, the window is no longer open; the program decides when to close it. |
| `window_present` | `function window_present(update window: Window, canvas: Canvas)` | Method `present` of `Window`. Shows a canvas in the window, its top left corner at the window's, over black, then waits until the next frame is due, letting other tasks run. A program whose frames take longer goes on at once, but in a web page it always waits a little: the page shows the frame, and takes the user's input, only while the program waits. |
| `window_width` | `function window_width(window: Window) -> integer` | Method `width` of `Window`. The width of the window's surface, in pixels. |
| `window_height` | `function window_height(window: Window) -> integer` | Method `height` of `Window`. The height of the window's surface, in pixels. |
| `window_scale` | `function window_scale(window: Window) -> float` | Method `scale` of `Window`. The screen's pixels for a point of the system's measure, such as 1.5 or 2 on screens of higher density. |
| `window_is_open` | `function window_is_open(window: Window) -> boolean` | Method `is_open` of `Window`. Whether the user has not asked to close the window. |
| `window_close` | `function window_close(own window: Window)` | Method `close` of `Window`. Closes the window. A window is also closed once nothing holds it. |

## standard/io.sayon

Console and file input and output.

| Export | Declaration | Description |
|---|---|---|
| `read_file` | `function read_file(path: string) -> string` | The whole content of a text file. Fails with IOError, or EncodingError when the file is not UTF-8. |
| `write_file` | `function write_file(path: string, content: string)` | Replaces a file's content, creating the file when needed. |
| `append_file` | `function append_file(path: string, content: string)` | Adds text to the end of a file, creating the file when needed. |
| `read_lines` | `function read_lines(path: string) -> Array<string>` | The lines of a text file, without their line endings. Accepts LF and CRLF. |
| `read_file_bytes` | `function read_file_bytes(path: string) -> Array<byte>` | The whole content of a file as bytes, whatever they are. |
| `write_file_bytes` | `function write_file_bytes(path: string, bytes: Array<byte>)` | Replaces a file's content with bytes, creating the file when needed. |
| `read_line` | `function read_line() -> string or nothing` | The next line of standard input without its line ending, or nothing at the end of input. |
| `write` | `function write(text: string)` | Writes text to standard output without adding a newline. |
| `write_error` | `function write_error(text: string)` | Writes a line to standard error. |
| `input` | `function input() -> File` | Standard input as a file, to read with `read_line`, `lines`, `read`, and `read_bytes`, sharing what it has read with `io.read_line`. It cannot `seek`, and closing it only stops this file from reading. |
| `output` | `function output() -> File` | Standard output as a file, for code that takes one; what it writes comes in order with what `print` does. Closing it only stops this file. |
| `errors` | `function errors() -> File` | Standard error as a file. Closing it only stops this file. |
| `LINE_LIMIT` | `constant LINE_LIMIT = 1_048_576` | The longest line, in bytes, that a file's `read_line` and `lines` accept unless its `line_limit` is changed. A longer line fails with IOError instead of filling memory. |
| `File` | `type File = [stream: HostStream, path: string, line_limit: integer]` | An open file, from `open`. Reading and writing go through a buffer. `file.close()` finishes with it and reports any error in writing what is still buffered; a file that is not closed is closed when it is no longer used, without a report. A file has one owner, so it cannot be cloned. |
| `open` | `function open(path: string, mode = "r", encoding = "utf-8") -> File` | Opens a file: with "r" to read it, "w" to write it from the start (creating it, or emptying it first), or "a" to write at its end (creating it when needed). Its text is in `encoding`: UTF-8 unless given, or one of the encodings of standard/strings's `encode`; in another than UTF-8, the file has no positions to `seek` to, and its bytes are those of its text in UTF-8. Fails with IOError when the file cannot be opened, and with EncodingError when it does not hold text in its encoding. |
| `file_read_line` | `function file_read_line(update file: File) -> string or nothing` | Method `read_line` of `File`. The next line without its line ending (LF or CRLF), or nothing at the end of the file. The last line need not end with a line ending. |
| `file_lines` | `function file_lines(update file: File) -> Lines` | Method `lines` of `File`. The lines that remain, for a `for` loop. The loop borrows the file until it finishes. |
| `file_read` | `function file_read(update file: File) -> string` | Method `read` of `File`. The rest of the file as text. Fails with EncodingError when it is not UTF-8. |
| `file_read_bytes` | `function file_read_bytes(update file: File, count: integer) -> Array<byte>` | Method `read_bytes` of `File`. Up to `count` bytes; fewer, or none, only at the end of the file. |
| `file_read_text` | `function file_read_text(update file: File, count: integer) -> string` | Method `read_text` of `File`. Up to `count` bytes of text, or a few more to end with a whole character; "" only at the end of the file. Reading a file a part at a time this way never cuts a character in two. |
| `file_write` | `function file_write(update file: File, text: string)` | Method `write` of `File`. Writes text, through the file's buffer. |
| `file_write_line` | `function file_write_line(update file: File, text: string)` | Method `write_line` of `File`. Writes text and a line ending. |
| `file_write_bytes` | `function file_write_bytes(update file: File, bytes: Array<byte>)` | Method `write_bytes` of `File`. Writes bytes as they are, through the file's buffer. |
| `file_flush` | `function file_flush(update file: File)` | Method `flush` of `File`. Passes what the file buffers to the system. This does not wait for it to reach the disk. |
| `file_position` | `function file_position(file: File) -> integer` | Method `position` of `File`. The position in the file, in bytes from its start. |
| `file_seek` | `function file_seek(update file: File, position: integer)` | Method `seek` of `File`. Moves to a position, in bytes from the start, for the next read or write. |
| `file_close` | `function file_close(own file: File)` | Method `close` of `File`. Finishes with a file: writes what it still buffers and closes it. Fails with IOError if writing fails. The file cannot be used afterwards. |
| `Lines` | `type Lines = [stream: HostStream, limit: integer]` | The lines of a file, from `file.lines()`. They are read from the file itself, so they end when it is closed. |
| `lines_next` | `function lines_next(lines: Lines) -> string or nothing` | Method `next` of `Lines`. The next line, or nothing at the end; a `for` loop calls it. |
| `copy` | `function copy(update source: File, update target: File) -> integer` | Copies the rest of `source` to the end of what was written to `target`, a part at a time, and returns the number of bytes copied. |

## standard/json.sayon

JSON: reading text into values, and writing values as text.

| Export | Declaration | Description |
|---|---|---|
| `Json` | `enum Json: null, flag(value: boolean), number(digits: string), text(value: string), list(items: Array<Json>), object(fields: Dict<string, Json>)` | A JSON value: `null`, a boolean, a number kept as the exact text it was written with, text, a list, or an object whose fields keep their order. |
| `parse` | `function parse(source: string) -> Json` | Reads JSON text into a value. |
| `encode` | `function encode(value: Json) -> string` | Writes a value as compact JSON text. |
| `pretty` | `function pretty(value: Json) -> string` | Writes a value as JSON text with one field or item per line, indented by two spaces for each level. |
| `number_value` | `function number_value(value: Json) -> integer or float or nothing` | A number's value, as `tonumber` reads its text, or nothing for another kind of value. |
| `text_value` | `function text_value(value: Json) -> string or nothing` | A text's string, or nothing for another kind of value. |
| `flag_value` | `function flag_value(value: Json) -> boolean or nothing` | A flag's boolean, or nothing for another kind of value. |
| `field` | `function field(value: Json, name: string) -> Json or nothing` | A copy of an object's field, or nothing when the value is not an object or has no such field. |
| `item` | `function item(value: Json, position: integer) -> Json or nothing` | A copy of a list's item, or nothing when the value is not a list or the position is outside it. |
| `from_integer` | `function from_integer(value: integer) -> Json` | A number from an integer. |
| `decode` | `function decode<T>(text: string) -> T` | Reads JSON text into a value of the type it is used as, such as `point: Point = json.decode(text)` or `json.decode<Array<Point>>(text)`. Fails with ParseError for text that is not JSON, and with DecodeError, naming where in the value, such as `$.points[2].x`, and what was expected, for JSON that does not fit the type. |
| `from_json` | `function from_json<T>(value: Json) -> T` | A Json value read into a value of the type it is used as, as `decode` reads text. |
| `to_json` | `function to_json<T>(value: T) -> Json` | A value as Json, in the form `decode` reads. Fails with ValueError for a float that is not a number JSON can hold (infinite or not a number). |
| `encode_value` | `function encode_value<T>(value: T) -> string` | A value as compact JSON text, in the form `decode` reads. |
| `Items` | `type Items = [stream: HostStream, buffer: Array<string>, position: integer, count: integer, started: boolean, finished: boolean]` | The items of the JSON array a file holds, from `items`, read as a loop goes through them. |
| `items` | `function items(update file: io.File) -> Items` | The items of the JSON array a file holds, for a `for` loop that reads them one at a time, so that the array need not fit in memory: |
| `items_next` | `function items_next(update items: Items) -> Json or nothing` | Method `next` of `Items`. The next item, or nothing after the last; a `for` loop calls it. |
| `Lines` | `type Lines = [stream: HostStream, line: integer]` | The values of a file of JSON Lines, from `lines`: one value on each line. |
| `lines` | `function lines(update file: io.File) -> Lines` | The values of a file of JSON Lines, one on each line, for a `for` loop that reads them one at a time. Blank lines are skipped. Fails with ParseError, naming the line, for a line that is not JSON. |
| `lines_next` | `function lines_next(update lines: Lines) -> Json or nothing` | Method `next` of `Lines`. The next value, or nothing at the end of the file; a `for` loop calls it. |
| `ItemWriter` | `type ItemWriter = [stream: HostStream, count: integer]` | Writes a JSON array to a file an item at a time, from `write_items`. `finish()` ends the array; the file is still to be closed. |
| `write_items` | `function write_items(update file: io.File) -> ItemWriter` | Starts writing a JSON array to a file, an item on each line. |
| `item_writer_add` | `function item_writer_add(update writer: ItemWriter, value: Json)` | Method `add` of `ItemWriter`. Writes a value as the array's next item, on a line of its own. |
| `item_writer_add_value` | `function item_writer_add_value<T>(update writer: ItemWriter, value: T)` | Method `add_value` of `ItemWriter`. Adds a value of any type JSON can hold, as `encode_value` writes it. |
| `item_writer_finish` | `function item_writer_finish(own writer: ItemWriter)` | Method `finish` of `ItemWriter`. Ends the array. |

## standard/math.sayon

Mathematical functions and constants.

| Export | Declaration | Description |
|---|---|---|
| `PI` | `constant PI = 3.141592653589793` | π, the ratio of a circle's circumference to its diameter. |
| `E` | `constant E = 2.718281828459045` | e, the base of the natural logarithm. |
| `TAU` | `constant TAU = 6.283185307179586` | τ, 2π: a whole turn in radians. |
| `sqrt` | `function sqrt(x: float) -> float` | The square root; NaN for a negative number. |
| `sin` | `function sin(x: float) -> float` | The sine of an angle in radians. |
| `cos` | `function cos(x: float) -> float` | The cosine of an angle in radians. |
| `tan` | `function tan(x: float) -> float` | The tangent of an angle in radians. |
| `asin` | `function asin(x: float) -> float` | The angle, from -PI/2 to PI/2, whose sine is `x`; NaN outside -1 to 1. |
| `acos` | `function acos(x: float) -> float` | The angle, from 0 to PI, whose cosine is `x`; NaN outside -1 to 1. |
| `atan` | `function atan(x: float) -> float` | The angle, from -PI/2 to PI/2, whose tangent is `x`. |
| `atan2` | `function atan2(y: float, x: float) -> float` | The angle of the point (x, y), from -PI to PI. |
| `exp` | `function exp(x: float) -> float` | e raised to the power `x`. |
| `log` | `function log(x: float) -> float` | The natural logarithm. |
| `log10` | `function log10(x: float) -> float` | The base-10 logarithm. |
| `log2` | `function log2(x: float) -> float` | The base-2 logarithm. |
| `pow` | `function pow(base: float, exponent: float) -> float` | `base` raised to the power `exponent`. |
| `floor` | `function floor(x: float) -> float` | The greatest whole number not above `x`. |
| `ceil` | `function ceil(x: float) -> float` | The least whole number not below `x`. |
| `round` | `function round(x: float) -> float` | Rounds half-way values away from zero. |
| `trunc` | `function trunc(x: float) -> float` | `x` without its fraction: toward zero. |
| `abs` | `function abs(x)` | The absolute value of any number. Fails for the most negative integer, whose absolute value does not fit. |
| `min` | `function min(a, b)` | The smaller of two numbers or strings; `a` when they are equal. |
| `max` | `function max(a, b)` | The larger of two numbers or strings; `a` when they are equal. |
| `clamp` | `function clamp(value, low, high)` | The value limited to the range from `low` to `high`. |
| `random` | `function random() -> float` | A float from 0 up to, but not including, 1. Not suitable for security. |
| `Generator` | `type Generator = [state: Array<integer>]` | A source of pseudo-random numbers that starts from a seed: generators made from the same seed give the same numbers, in the same order, on every device and target, in the interpreter and compiled. It is Pierre L'Ecuyer's MRG32k3a, whose numbers repeat only after about 2^191 of them. Not suitable for security; `crypto.random_bytes` is. |
| `new_generator` | `function new_generator(seed: integer) -> Generator` | A generator whose numbers follow from `seed`, any integer. The seed's SHA-256 digest gives the starting state, so nearby seeds give unrelated numbers. |
| `generator_float` | `function generator_float(update generator: Generator) -> float` | Method `float` of `Generator`. A float from 0 up to, but not including, 1, with all 53 bits of a float's precision. |
| `generator_integer` | `function generator_integer(update generator: Generator, low: integer, high: integer) -> integer` | Method `integer` of `Generator`. An integer from `low` up to, but not including, `high`, each as likely as the others: `integer(1, 7)` rolls a die. Fails with ValueError when `high` is not above `low`, and with OverflowError when the range is wider than an integer holds. |
| `generator_shuffle` | `function generator_shuffle<T>(update generator: Generator, update items: Array<T>)` | Method `shuffle` of `Generator`. Puts the items in a random order, every order as likely as the others. |

## standard/memory.sayon

Reading and writing the memory that C code shares with a program.

| Export | Declaration | Description |
|---|---|---|
| `address` | `function address(text: c_string) -> pointer` | Where a C string's text is, for a C function that takes a `pointer`. The address is valid while the C string lives. |
| `read_text` | `unsafe function read_text(address: pointer) -> string` | The text of the C string at `address`, up to the NUL byte that ends it. Fails with EncodingError when the bytes are not UTF-8. |
| `read` | `unsafe function read<T>(address: pointer, offset = 0) -> T` | The number, pointer, or C structure (a `foreign type`) stored `offset` bytes after `address`. |
| `write` | `unsafe function write<T>(address: pointer, value: T, offset = 0)` | Stores a number, pointer, or C structure `offset` bytes after `address`. |
| `read_bytes` | `unsafe function read_bytes(address: pointer, count: integer) -> Array<byte>` | `count` bytes starting at `address`. |
| `write_bytes` | `unsafe function write_bytes(address: pointer, bytes: Array<byte>)` | Stores `bytes` starting at `address`. |
| `offset` | `function offset(address: pointer, bytes: integer) -> pointer` | The address `bytes` after `address`, or before it when `bytes` is negative. |
| `size` | `function size<T>() -> integer` | The bytes a value of type `T` takes in memory: a number, a pointer, or a C structure, as C lays it out. An array of them in C memory has one every `size<T>()` bytes. |

## standard/network.sayon

TCP connections and listeners, UDP datagrams, TLS, and an HTTP client.

| Export | Declaration | Description |
|---|---|---|
| `Address` | `type Address = [host: string, port: integer]` | A host and a port. The host is a name, or a numeric address such as "127.0.0.1" or "::1". |
| `address_text` | `function address_text(address: Address) -> string` | Method `text` of `Address`. The address as text, as in "127.0.0.1:80", or "[::1]:80". |
| `resolve` | `function resolve(host: string) -> Array<string>` | The numeric addresses of a host, in the order the system prefers them. Fails with AddressError for a host that has none. Finding them waits for the system, without letting other tasks run. |
| `Connection` | `type Connection = [stream: HostStream, line_limit: integer]` | A TCP connection, from `connect` or from a listener's `accept`. It is read and written like a file of standard/io, in lines, text, and bytes, and what it reads is what the other side has sent so far. `close()` waits for what was written to go, and closes it; a connection that is not closed is closed when it is no longer used. A connection has one owner, so it cannot be cloned. |
| `connect` | `function connect(host: string, port: integer, timeout = time.seconds(30)) -> Connection` | Connects to a port of a host, on the first of its addresses that answers. Fails with ConnectionError when nothing accepts the connection, AddressError when the host has no address, and TimeoutError after `timeout`. |
| `connect_secure` | `function connect_secure(host: string, port: integer, timeout = time.seconds(30)) -> Connection` | Connects as `connect` does, and starts TLS with the host, which must show a certificate for its name that the system trusts. Fails as `connect` does, and with TlsError when TLS fails. |
| `connection_secure` | `function connection_secure(update connection: Connection, server: string)` | Method `secure` of `Connection`. Starts TLS on the connection, with the server named `server`: what is read and written after it is encrypted. The system must trust the server's certificate, which must be for that name; otherwise it fails with TlsError. A protocol that starts in plain text and then turns to TLS calls it when the other side agrees to. |
| `connection_read_line` | `function connection_read_line(update connection: Connection) -> string or nothing` | Method `read_line` of `Connection`. The next line without its line ending (LF or CRLF), or nothing once the other side has finished sending. |
| `connection_lines` | `function connection_lines(update connection: Connection) -> io.Lines` | Method `lines` of `Connection`. The lines to come, for a `for` loop, until the other side finishes. |
| `connection_read` | `function connection_read(update connection: Connection) -> string` | Method `read` of `Connection`. Everything the other side sends until it finishes, as text. |
| `connection_read_bytes` | `function connection_read_bytes(update connection: Connection, count: integer) -> Array<byte>` | Method `read_bytes` of `Connection`. Up to `count` bytes: as many as have arrived, waiting for one when none has. None once the other side has finished sending. |
| `connection_read_exactly` | `function connection_read_exactly(update connection: Connection, count: integer) -> Array<byte>` | Method `read_exactly` of `Connection`. `count` bytes, waiting until they have all arrived, however the other side sent them; fewer only when it finishes sending first. |
| `connection_write` | `function connection_write(update connection: Connection, text: string)` | Method `write` of `Connection`. Writes text, as UTF-8. |
| `connection_write_line` | `function connection_write_line(update connection: Connection, text: string)` | Method `write_line` of `Connection`. Writes text and a line ending, LF. |
| `connection_write_bytes` | `function connection_write_bytes(update connection: Connection, bytes: Array<byte>)` | Method `write_bytes` of `Connection`. Writes bytes as they are. |
| `connection_flush` | `function connection_flush(update connection: Connection)` | Method `flush` of `Connection`. Waits until what was written has gone to the system. |
| `connection_finish_sending` | `function connection_finish_sending(update connection: Connection)` | Method `finish_sending` of `Connection`. Finishes sending: the other side reads to the end of what was written, and this side can still read. |
| `connection_set_timeout` | `function connection_set_timeout(update connection: Connection, timeout: time.Duration)` | Method `set_timeout` of `Connection`. How long each read or write may wait; zero for no limit, which is how a connection starts. |
| `connection_local_address` | `function connection_local_address(connection: Connection) -> Address` | Method `local_address` of `Connection`. This computer's address and port of the connection. |
| `connection_peer_address` | `function connection_peer_address(connection: Connection) -> Address` | Method `peer_address` of `Connection`. The other side's address and port. |
| `connection_close` | `function connection_close(own connection: Connection)` | Method `close` of `Connection`. Waits for what was written to go, and closes the connection. |
| `Listener` | `type Listener = [stream: HostStream]` | A listener for connections to a port of this computer, from `listen`. |
| `listen` | `function listen(host: string, port: integer) -> Listener` | Listens on a port: 0 picks a free one, which `address()` tells. A host of "127.0.0.1" or "localhost" accepts connections only from this computer, and "0.0.0.0" from any. Fails with AddressError when the port is in use. |
| `listener_accept` | `function listener_accept(update listener: Listener) -> Connection` | Method `accept` of `Listener`. The next connection made to the listener, waiting for one. |
| `listener_address` | `function listener_address(listener: Listener) -> Address` | Method `address` of `Listener`. The address and port the listener listens on. |
| `listener_set_timeout` | `function listener_set_timeout(update listener: Listener, timeout: time.Duration)` | Method `set_timeout` of `Listener`. How long `accept` may wait; zero for no limit, which is how a listener starts. |
| `listener_close` | `function listener_close(own listener: Listener)` | Method `close` of `Listener`. Stops listening. |
| `DatagramSocket` | `type DatagramSocket = [stream: HostStream]` | A UDP socket, from `bind`, which sends and receives datagrams: messages of bytes, each delivered whole or not at all, in no promised order. |
| `Datagram` | `type Datagram = [bytes: Array<byte>, sender: Address]` | A datagram received, and where it came from. |
| `bind` | `function bind(host: string, port: integer) -> DatagramSocket` | A socket for datagrams on a port of this computer; 0 picks a free one. |
| `datagram_socket_send` | `function datagram_socket_send(update socket: DatagramSocket, bytes: Array<byte>, host: string, port: integer)` | Method `send` of `DatagramSocket`. Sends one datagram to a port of a host. |
| `datagram_socket_receive` | `function datagram_socket_receive(update socket: DatagramSocket, size = 65536) -> Datagram` | Method `receive` of `DatagramSocket`. The next datagram, waiting for one. Up to `size` bytes of it are kept. |
| `datagram_socket_address` | `function datagram_socket_address(socket: DatagramSocket) -> Address` | Method `address` of `DatagramSocket`. The address and port the socket uses. |
| `datagram_socket_set_timeout` | `function datagram_socket_set_timeout(update socket: DatagramSocket, timeout: time.Duration)` | Method `set_timeout` of `DatagramSocket`. How long `receive` and `send` may wait; zero for no limit, which is how a socket starts. |
| `datagram_socket_close` | `function datagram_socket_close(own socket: DatagramSocket)` | Method `close` of `DatagramSocket`. Closes the socket; it cannot be used afterwards. |
| `Response` | `type Response = [status: integer, reason: string, headers: Dict<string, string>, body: Array<byte>, url: string]` | The answer to an HTTP request: its status (such as 200 or 404) and the text that comes with it, its headers by lowercase name, its body, and the URL that gave it, after any redirects. |
| `response_text` | `function response_text(response: Response) -> string` | Method `text` of `Response`. The body as text. Fails with EncodingError when it is not UTF-8. |
| `response_header` | `function response_header(response: Response, name: string) -> string or nothing` | Method `header` of `Response`. A header's value, by its name in any letter case, or nothing when the response has none of that name. |
| `response_succeeded` | `function response_succeeded(response: Response) -> boolean` | Method `succeeded` of `Response`. Whether the status is a success, 200 to 299. |
| `get` | `function get(url: string, headers: Dict<string, string> = []) -> Response` | Gets a URL. |
| `post` | `function post(url: string, body: string, content_type = "text/plain; charset=utf-8") -> Response` | Posts text to a URL. |
| `request` | `function request(method: string, url: string, headers: Dict<string, string> = [], body: Array<byte> = [], timeout = time.seconds(30), redirects = 10, limit = 16_777_216) -> Response` | Sends an HTTP/1.1 request and gives the response, following up to `redirects` redirects. Each step of connecting, sending, and receiving may wait for `timeout`; a body longer than `limit` bytes fails with HttpError, as do a response that is not HTTP and a URL that is neither `http://` nor `https://`, which goes through TLS. A status that reports a failure, such as 404, is a response like any other; `response.succeeded()` tells it apart. |

## standard/os.sayon

The program's environment.

| Export | Declaration | Description |
|---|---|---|
| `arguments` | `function arguments() -> Array<string>` | The arguments given to the program, without the program's own name. With `sayon run`, these are the words after `--`. |
| `environment` | `function environment(name: string) -> string or nothing` | The value of an environment variable, or nothing when it is not set. |
| `platform` | `function platform() -> string` | The operating system: "windows", "linux", "macos", or "android". |
| `current_directory` | `function current_directory() -> string` | The directory the program is running in. |
| `Usage` | `type Usage = [peak_memory: integer, processor_time: time.Duration]` | What a finished program used: the most memory it held at once, in bytes, and the processor time it took, its own and the system's on its behalf, added up over its threads. Both are 0 where the system does not say. |
| `Completed` | `type Completed = [status: integer, output: string, errors: string, usage: Usage]` | What a program run by `run` did: its exit status (-1 when it was stopped without one), what it wrote to its standard output and error, and what it used. |
| `run` | `function run(program: string, arguments: Array<string>, directory = "", input = "", environment: Dict<string, string> = [], inherit_environment = true) -> Completed` | Runs a program and waits for it to finish, letting other tasks run. `program` is a path, or a name found on the system's search path; the arguments are passed exactly as they are, without a shell, so no character in them has a special meaning. The program reads `input`, and runs in `directory`, or in the current directory when it is "". It has this program's environment variables and those of `environment`, or with `inherit_environment = false` only those. Fails with IOError when it cannot be started, and with EncodingError when its output is not UTF-8. |
| `Process` | `type Process = [handle: HostStream, input: io.File, output: io.File, errors: io.File]` | A program started with `start`: what it reads from `input`, and what it writes to `output` and `errors`, are files of standard/io, which can be read and written while it runs. Reading them, and waiting for the program, let other tasks run. |
| `start` | `function start(program: string, arguments: Array<string>, directory = "", environment: Dict<string, string> = [], inherit_environment = true) -> Process` | Starts a program, as `run` does, without waiting for it to finish. |
| `process_finish_input` | `function process_finish_input(update process: Process)` | Method `finish_input` of `Process`. Ends the program's input once what was written to it has gone: the program then reads to its end. |
| `process_wait` | `function process_wait(update process: Process) -> integer` | Method `wait` of `Process`. Waits for the program to finish, and gives its exit status: -1 when it was stopped without one. |
| `process_usage` | `function process_usage(process: Process) -> Usage` | Method `usage` of `Process`. What the program used, once it has finished. Fails with ValueError while it is still running. |
| `process_stop` | `function process_stop(update process: Process)` | Method `stop` of `Process`. Stops the program at once, unless it has finished. |
| `process_id` | `function process_id(process: Process) -> integer` | Method `id` of `Process`. The number the system knows the program by. |
| `exit` | `function exit(code = 0)` | Ends the program with an exit status from 0 (success) to 255. The program ends in order: `finally` blocks run and values are cleaned up on the way out, though `catch` and `try` do not stop it. Fails with ValueError for a status outside 0 to 255. |

## standard/strings.sayon

Operations on text. Positions and lengths count Unicode characters (code points); `graphemes` works with what readers see as characters, and `bytes` with the UTF-8 encoding.

| Export | Declaration | Description |
|---|---|---|
| `split` | `function split(text: string, separator: string) -> Array<string>` | Splits text at every occurrence of a non-empty separator. |
| `lines` | `function lines(text: string) -> Array<string>` | The lines of a text, accepting LF and CRLF line endings. |
| `join` | `function join(parts: Array<string>, separator: string) -> string` | Joins parts with a separator between them. |
| `trim` | `function trim(text: string) -> string` | The text without leading and trailing whitespace: the characters Unicode calls white space, such as spaces, tabs, and line breaks. |
| `trim_start` | `function trim_start(text: string) -> string` | The text without leading whitespace. |
| `trim_end` | `function trim_end(text: string) -> string` | The text without trailing whitespace. |
| `upper` | `function upper(text: string) -> string` | The text in uppercase, by Unicode's rules for any language: `ß` becomes `SS`. |
| `lower` | `function lower(text: string) -> string` | The text in lowercase, by Unicode's rules for any language. |
| `replace` | `function replace(text: string, from: string, to: string) -> string` | Replaces every occurrence of a non-empty piece of text. |
| `starts_with` | `function starts_with(text: string, prefix: string) -> boolean` | Whether the text begins with `prefix`. |
| `ends_with` | `function ends_with(text: string, suffix: string) -> boolean` | Whether the text ends with `suffix`. |
| `contains` | `function contains(text: string, part: string) -> boolean` | Whether `part` occurs anywhere in the text. |
| `find` | `function find(text: string, part: string) -> integer or nothing` | The position of the first occurrence of `part`, or nothing. |
| `repeat` | `function repeat(text: string, count: integer) -> string` | The text repeated `count` times. |
| `pad_start` | `function pad_start(text: string, width: integer, fill = " ") -> string` | The text with `fill`, one character, put before it until it is `width` characters long: `pad_start("7", 3, "0")` is "007". Longer text stays as it is. |
| `pad_end` | `function pad_end(text: string, width: integer, fill = " ") -> string` | The text with `fill`, one character, put after it until it is `width` characters long: `pad_end("ab", 4)` is "ab  ". Longer text stays as it is. |
| `slice` | `function slice(text: string, start: integer, end: integer) -> string` | The characters from `start` up to, but not including, `end`. |
| `characters` | `function characters(text: string) -> Array<string>` | Each character as a one-character string. |
| `graphemes` | `function graphemes(text: string) -> Array<string>` | The user-perceived characters of a text, its extended grapheme clusters: a letter with its accents, a whole emoji sequence, or a flag is one, so `graphemes("e\u{301}")` is ["é"] while `characters` gives two strings. The rules are those of Unicode 17.0. |
| `grapheme_count` | `function grapheme_count(text: string) -> integer` | The number of user-perceived characters, as `graphemes` finds them. |
| `bytes` | `function bytes(text: string) -> Array<byte>` | The text's UTF-8 encoding; `bytelength(text)` is its length. |
| `byte_view` | `function byte_view(text: string, start = 0, end = bytelength(text)) -> View<byte>` | The UTF-8 bytes of a text from `start` up to, but not including, `end`, without copying them. The view borrows the text for as long as it is used. |
| `from_bytes` | `function from_bytes(bytes: Array<byte>) -> string` | The text that UTF-8 bytes encode. Fails with EncodingError when they are not valid UTF-8. |
| `from_bytes_lossy` | `function from_bytes_lossy(bytes: Array<byte>) -> string` | The text that UTF-8 bytes encode, with U+FFFD, the replacement character, in place of each invalid sequence. |
| `encode` | `function encode(text: string, encoding: string) -> Array<byte>` | Text in an encoding: "utf-8", "utf-16" (little-endian after a byte-order mark), "utf-16le", "utf-16be", "latin-1", "ascii", or "windows-1252". Fails with EncodingError for a character the encoding has no form for. |
| `decode` | `function decode(bytes: Array<byte>, encoding: string) -> string` | Bytes of an encoding as text; "utf-16" follows a byte-order mark, and is little-endian without one. Fails with EncodingError for bytes that are not valid in the encoding. |
| `decode_lossy` | `function decode_lossy(bytes: Array<byte>, encoding: string) -> string` | Bytes of an encoding as text, with U+FFFD for the bytes that are not valid in it. |
| `code` | `function code(character: string) -> integer` | The Unicode code of a one-character string, such as 65 for "A". |
| `character` | `function character(code: integer) -> string` | The one-character string with a Unicode code, such as "A" for 65. |
| `parse_integer` | `function parse_integer(text: string, base = 10) -> integer` | The integer that `text` writes in `base`, from 2 to 36: an optional `+` or `-`, then digits, which from 10 on are the letters `a` to `z` in either case. `parse_integer("ff", 16)` is 255, and `parse_integer("-101", 2)` is -5. Fails with ParseError for other text, and for a number that does not fit in an integer. |
| `decimal` | `function decimal(value: float, places: integer) -> string` | The number written with `places` digits after the decimal point, from 0 to 100, rounded to the nearest: `decimal(3.14159, 2)` is "3.14". A tie goes to the even digit, and since the float nearest a decimal is a little more or less than it, `decimal(2.675, 2)` is "2.67". NaN and the infinities are written as `tostring` writes them. |

## standard/threads.sayon

Tasks that run together, and channels between them.

| Export | Declaration | Description |
|---|---|---|
| `ReceiveResult` | `enum ReceiveResult<T>: message(value: T), closed` | What `receive_result` gives: a message, or the end of a closed channel. |
| `SendResult` | `enum SendResult<T>: sent, unsent(message: T, problem: Error)` | What `recoverable_send` gives: the message was sent, or it comes back with the error that kept it from being sent. |
| `spawn` | `function spawn<T>(own body: function() -> T) -> Task<T>` | Starts a task running `body`, and gives its handle. |
| `spawn_parallel` | `function spawn_parallel<T>(own body: function() -> T) -> Task<T>` | Starts a task running `body` on a thread of its own, and gives its handle, which `join` and `detach` take like any other. Fails with ThreadError when no thread can be started. |
| `join` | `function join<T>(own task: Task<T>) -> T` | Waits for a task to finish and gives its result. When the task failed, fails with its error. |
| `detach` | `function detach<T>(own task: Task<T>)` | Lets a task run on without a handle. The program still waits for it to finish, or cancels it, when the main task ends. |
| `pause` | `function pause()` | Lets the other tasks that are ready run before this one continues. |
| `new_channel` | `function new_channel<T>(capacity = 64) -> Channel<T>` | A channel of messages, kept in the order they were sent. Up to `capacity` messages wait in it; then `send` waits for room. A capacity of 0 makes each `send` wait until its message is received. |
| `new_unbounded_channel` | `function new_unbounded_channel<T>() -> Channel<T>` | A channel without a limit, so that `send` never waits for room. The messages that wait in it take memory without bound, so prefer a limit when senders can outpace receivers. |
| `send` | `function send<T>(channel: Channel<T>, own message: T)` | Sends a message, waiting while the channel is full. Fails with ChannelClosedError when the channel is closed; the message is then destroyed. |
| `recoverable_send` | `function recoverable_send<T>(channel: Channel<T>, own message: T) -> SendResult<T>` | Sends a message like `send`, but when the channel is closed, gives the message back as `unsent(message, problem)` instead of destroying it. |
| `receive` | `function receive<T>(channel: Channel<T>) -> T` | The next message, waiting while the channel is empty. Fails with ChannelClosedError when the channel is closed and has none left. |
| `receive_result` | `function receive_result<T>(channel: Channel<T>) -> ReceiveResult<T>` | The next message, or `closed` at the end of a closed channel, which tells the end apart from any message. |
| `after` | `function after<T>(delay: time.Duration, own message: T) -> Channel<T>` | A channel that receives `message` once `delay` has passed: a timer, which `select` waits for along with other channels, so that waiting for a message can end when it takes too long. |
| `close` | `function close<T>(channel: Channel<T>)` | Closes a channel: no more messages can be sent, and receivers get the ones already sent and then ChannelClosedError. |
| `select` | `function select<T>(channels: Array<Channel<T>>, update handler: function(own T, integer))` | Waits until one of the channels has a message or is closed and empty, receives from it, and calls `handler` with the message and the channel's position. A closed, empty channel fails with ChannelClosedError. When several are ready, the choice goes round, starting after the last one this task chose. |

## standard/time.sayon

Clocks, durations, and dates.

| Export | Declaration | Description |
|---|---|---|
| `Duration` | `type Duration = [nanoseconds: integer]` | A length of time, which may be negative. |
| `Instant` | `type Instant = [nanoseconds: integer]` | A reading of the monotonic clock. |
| `DateTime` | `type DateTime = [year: integer, month: integer, day: integer, hour: integer, minute: integer, second: integer, nanosecond: integer]` | A moment of the wall clock, as a date and time in UTC. |
| `nanoseconds` | `function nanoseconds(count: integer) -> Duration` | A duration of `count` nanoseconds. |
| `microseconds` | `function microseconds(count: integer) -> Duration` | A duration of `count` microseconds. |
| `milliseconds` | `function milliseconds(count: integer) -> Duration` | A duration of `count` milliseconds. |
| `seconds` | `function seconds(count: integer) -> Duration` | A duration of `count` seconds. |
| `minutes` | `function minutes(count: integer) -> Duration` | A duration of `count` minutes. |
| `hours` | `function hours(count: integer) -> Duration` | A duration of `count` hours. |
| `add` | `function add(a: Duration, b: Duration) -> Duration` | The two durations together. |
| `subtract` | `function subtract(a: Duration, b: Duration) -> Duration` | The first duration less the second. |
| `scale` | `function scale(duration: Duration, factor: integer) -> Duration` | The duration `factor` times over. |
| `in_seconds` | `function in_seconds(duration: Duration) -> float` | The duration in seconds, with a fraction. |
| `in_milliseconds` | `function in_milliseconds(duration: Duration) -> integer` | The duration in whole milliseconds, rounded toward zero. |
| `describe` | `function describe(duration: Duration) -> string` | Readable text for a duration, in the largest unit that keeps it at least 1: `2h 5m 3s`, `1.25s`, `350ms`, `12µs`, or `40ns`. |
| `start` | `function start() -> Instant` | Reads the monotonic clock. |
| `elapsed` | `function elapsed(since: Instant) -> Duration` | The time since an earlier reading of the monotonic clock. |
| `between` | `function between(earlier: Instant, later: Instant) -> Duration` | The time from one reading to a later one. |
| `sleep` | `function sleep(duration: Duration)` | Waits for at least a duration, letting other tasks run. |
| `Deadline` | `type Deadline = [at: Instant]` | A moment of the monotonic clock that work is to be done by. |
| `deadline` | `function deadline(after: Duration) -> Deadline` | The moment a duration from now. |
| `deadline_remaining` | `function deadline_remaining(deadline: Deadline) -> Duration` | Method `remaining` of `Deadline`. The time left until the deadline; none once it has passed. |
| `deadline_passed` | `function deadline_passed(deadline: Deadline) -> boolean` | Method `passed` of `Deadline`. Whether the deadline has passed. |
| `deadline_wait` | `function deadline_wait(deadline: Deadline)` | Method `wait` of `Deadline`. Waits until the deadline, letting other tasks run. |
| `Ticker` | `type Ticker = [next: integer, interval: Duration]` | Beats at a steady interval, for work done every so often: the beats keep to the interval however long the work between them takes. |
| `ticker` | `function ticker(interval: Duration) -> Ticker` | A ticker whose first beat is an interval from now. Fails with ValueError for an interval that is not longer than zero. |
| `ticker_wait` | `function ticker_wait(update ticker: Ticker) -> integer` | Method `wait` of `Ticker`. Waits for the next beat, letting other tasks run, and gives how many beats have come: 1, or more when the work since the last beat took longer than the interval and beats were missed. |
| `now` | `function now() -> DateTime` | The current date and time, in UTC. |
| `unix_seconds` | `function unix_seconds() -> integer` | Seconds since 1970-01-01T00:00:00Z, the Unix epoch. |
| `from_unix` | `function from_unix(count: integer) -> DateTime` | The date and time a number of seconds after the Unix epoch. |
| `to_unix` | `function to_unix(moment: DateTime) -> integer` | Seconds from the Unix epoch to a date and time, ignoring its nanoseconds. |
| `shift` | `function shift(moment: DateTime, by: Duration) -> DateTime` | A date and time moved by a duration. |
| `difference` | `function difference(earlier: DateTime, later: DateTime) -> Duration` | The duration from one date and time to another. |
| `weekday` | `function weekday(moment: DateTime) -> integer` | The day of the week, from 1 for Monday to 7 for Sunday. |
| `is_leap_year` | `function is_leap_year(year: integer) -> boolean` | Whether a year of the Gregorian calendar, extended backwards, has 366 days. |
| `days_in_month` | `function days_in_month(year: integer, month: integer) -> integer` | How many days a month, 1 to 12, of a year has. |
| `format` | `function format(moment: DateTime) -> string` | ISO 8601 text in UTC, such as `2026-10-18T09:30:00Z`. Fractions of a second are written only when there are any, with as few digits as needed. |
| `Zone` | `type Zone = [name: string]` | A time zone of the IANA time zone database, such as "Europe/Paris", with the rules the system has for its offsets from UTC. |
| `ZonedTime` | `type ZonedTime = [local: DateTime, offset: Duration, zone: Zone]` | A date and time as a zone's clocks show it: the local date and time, and their offset from UTC. |
| `zone` | `function zone(name: string) -> Zone` | A time zone by its name. Fails with TimeZoneError for a name the system's time zone database does not have. |
| `local_zone` | `function local_zone() -> Zone` | The system's own time zone, or UTC when it has none. |
| `zones_version` | `function zones_version() -> string` | The version of the system's time zone database, such as "2025b", or "unknown". |
| `zone_offset` | `function zone_offset(zone: Zone, moment: DateTime) -> Duration` | Method `offset` of `Zone`. The zone's offset from UTC at a moment, positive east of UTC: one hour in Paris in winter. |
| `zone_at` | `function zone_at(zone: Zone, moment: DateTime) -> ZonedTime` | Method `at` of `Zone`. A moment as the zone's clocks show it. |
| `zone_moment` | `function zone_moment(zone: Zone, local: DateTime) -> DateTime` | Method `moment` of `Zone`. The moment that the zone's clocks show as a local date and time. A local time in a gap, skipped when the clocks go forward, is read with the offset before the gap, so it falls after it; one in an overlap, which the clocks show twice, is the earlier of the two. |
| `zoned_time_moment` | `function zoned_time_moment(time: ZonedTime) -> DateTime` | Method `moment` of `ZonedTime`. The moment in UTC. |
| `zoned_time_format` | `function zoned_time_format(time: ZonedTime) -> string` | Method `format` of `ZonedTime`. ISO 8601 text with the offset, such as `2026-10-18T11:30:00+02:00`. |
| `parse` | `function parse(text: string) -> DateTime` | Reads ISO 8601 text: a date, `2026-10-18`, or a date and time with `Z` or an offset from UTC, `2026-10-18T09:30:00.5+02:00`. The result is in UTC. Text that is not such a date raises a ParseError. |
