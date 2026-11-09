/*
JSON: write a document of n records as one long string, then read it back
with a parser that walks its bytes, counting the records, adding up the
numbers, and counting the letters of the strings and the trues.
*/
#include <cstdio>
#include <cstdlib>
#include <string>

struct Totals {
    long long objects = 0;
    long long numbers = 0;
    long long letters = 0;
    long long trues = 0;
};

static void skip_space(const std::string& text, size_t& at) {
    while (at < text.size() && text[at] == ' ') ++at;
}

static void read_string(const std::string& text, size_t& at, Totals& totals) {
    at += 1;
    size_t start = at;
    while (text[at] != '"') ++at;
    totals.letters += static_cast<long long>(at - start);
    at += 1;
}

static void read_number(const std::string& text, size_t& at, Totals& totals) {
    long long number = 0;
    while (at < text.size() && text[at] >= '0' && text[at] <= '9') {
        number = number * 10 + (text[at] - '0');
        ++at;
    }
    totals.numbers += number;
}

static void read_value(const std::string& text, size_t& at, Totals& totals) {
    skip_space(text, at);
    char letter = text[at];
    if (letter == '{') {
        totals.objects += 1;
        at += 1;
        skip_space(text, at);
        if (text[at] == '}') {
            at += 1;
            return;
        }
        while (true) {
            skip_space(text, at);
            read_string(text, at, totals);
            skip_space(text, at);
            at += 1;
            read_value(text, at, totals);
            skip_space(text, at);
            at += 1;
            if (text[at - 1] != ',') return;
        }
    } else if (letter == '[') {
        at += 1;
        skip_space(text, at);
        if (text[at] == ']') {
            at += 1;
            return;
        }
        while (true) {
            read_value(text, at, totals);
            skip_space(text, at);
            at += 1;
            if (text[at - 1] != ',') return;
        }
    } else if (letter == '"') {
        read_string(text, at, totals);
    } else if (letter == 't') {
        totals.trues += 1;
        at += 4;
    } else if (letter == 'f') {
        at += 5;
    } else {
        read_number(text, at, totals);
    }
}

int main(int argc, char** argv) {
    long long count = argc > 1 ? std::atoll(argv[1]) : 1000;
    std::string text = "[";
    long long state = 42;
    for (long long index = 0; index < count; ++index) {
        if (index > 0) text += ", ";
        state = (state * 1103515245 + 12345) % 2147483648LL;
        text += "{\"id\": " + std::to_string(index) + ", \"name\": \"item" + std::to_string(state % 1000) +
                "\", \"tags\": [\"red\", \"green\"], \"price\": " + std::to_string(state / 16 % 100000) +
                ", \"ok\": " + (state % 2 == 0 ? "true" : "false") + "}";
    }
    text += "]";
    Totals totals;
    size_t at = 0;
    read_value(text, at, totals);
    std::printf("%zu bytes, %lld objects, numbers adding up to %lld, %lld letters, %lld true\n", text.size(),
                totals.objects, totals.numbers, totals.letters, totals.trues);
}
