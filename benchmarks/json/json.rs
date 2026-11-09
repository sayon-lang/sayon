/*
JSON: write a document of n records as one long string, then read it back
with a parser that walks its bytes, counting the records, adding up the
numbers, and counting the letters of the strings and the trues.
*/
use std::fmt::Write;

#[derive(Default)]
struct Totals {
    objects: i64,
    numbers: i64,
    letters: i64,
    trues: i64,
}

fn skip_space(text: &[u8], at: &mut usize) {
    while *at < text.len() && text[*at] == b' ' {
        *at += 1;
    }
}

fn read_string(text: &[u8], at: &mut usize, totals: &mut Totals) {
    *at += 1;
    let start = *at;
    while text[*at] != b'"' {
        *at += 1;
    }
    totals.letters += (*at - start) as i64;
    *at += 1;
}

fn read_number(text: &[u8], at: &mut usize, totals: &mut Totals) {
    let mut number: i64 = 0;
    while *at < text.len() && text[*at].is_ascii_digit() {
        number = number * 10 + (text[*at] - b'0') as i64;
        *at += 1;
    }
    totals.numbers += number;
}

fn read_value(text: &[u8], at: &mut usize, totals: &mut Totals) {
    skip_space(text, at);
    let letter = text[*at];
    if letter == b'{' {
        totals.objects += 1;
        *at += 1;
        skip_space(text, at);
        if text[*at] == b'}' {
            *at += 1;
            return;
        }
        loop {
            skip_space(text, at);
            read_string(text, at, totals);
            skip_space(text, at);
            *at += 1;
            read_value(text, at, totals);
            skip_space(text, at);
            *at += 1;
            if text[*at - 1] != b',' {
                return;
            }
        }
    } else if letter == b'[' {
        *at += 1;
        skip_space(text, at);
        if text[*at] == b']' {
            *at += 1;
            return;
        }
        loop {
            read_value(text, at, totals);
            skip_space(text, at);
            *at += 1;
            if text[*at - 1] != b',' {
                return;
            }
        }
    } else if letter == b'"' {
        read_string(text, at, totals);
    } else if letter == b't' {
        totals.trues += 1;
        *at += 4;
    } else if letter == b'f' {
        *at += 5;
    } else {
        read_number(text, at, totals);
    }
}

fn main() {
    let count: i64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(1000);
    let mut text = String::from("[");
    let mut state: i64 = 42;
    for index in 0..count {
        if index > 0 {
            text.push_str(", ");
        }
        state = (state * 1103515245 + 12345) % 2147483648;
        let _ = write!(
            text,
            "{{\"id\": {index}, \"name\": \"item{}\", \"tags\": [\"red\", \"green\"], \"price\": {}, \"ok\": {}}}",
            state % 1000,
            state / 16 % 100000,
            state % 2 == 0
        );
    }
    text.push(']');
    let mut totals = Totals::default();
    let mut at = 0;
    read_value(text.as_bytes(), &mut at, &mut totals);
    println!(
        "{} bytes, {} objects, numbers adding up to {}, {} letters, {} true",
        text.len(),
        totals.objects,
        totals.numbers,
        totals.letters,
        totals.trues
    );
}
