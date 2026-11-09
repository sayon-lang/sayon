# json

Writes a JSON document of 100,000, 300,000, and a million records as one long string, the largest about 90 MB, then reads it back with a small parser that walks the bytes. Each record has a number, a name, a list of two tags, a price, and a boolean. The parser counts the records, adds up the numbers, and counts the letters of the strings and the `true`s.

## What's the same in all three

The same document, built by adding each record to the end of one string, and the same parser, which looks at one byte at a time and calls itself for the values inside objects and lists. None of them uses a JSON library: Sayon has one, but C++ has none in its standard library, so all three use the same hand-written parser instead.

## What differs

Writing a record. C++ joins temporary strings with `+` and appends the result, Rust writes straight into the document with `write!`, and Sayon builds each record with interpolation and appends it. Each is the usual way in its language.

Reading the bytes. C++ indexes its `std::string`, and Rust a byte slice of its `String`. Sayon takes a view of the string's bytes with `strings.byte_view`, which copies nothing, and indexes that, checking each index against the view.

## Results

<!-- results: run.sayon writes this part -->

<!-- end of results -->
