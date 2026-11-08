// Text: format many lines into one string, split it back into an array of
// lines, and look through them, counting characters as Sayon's `length`
// does.
#include <cstdio>
#include <cstdlib>
#include <string>
#include <vector>

// The characters in UTF-8 text: every byte but those that continue one.
static long characters_in(const std::string& text) {
    long count = 0;
    for (unsigned char byte : text) {
        if ((byte & 0xC0) != 0x80) ++count;
    }
    return count;
}

int main(int argc, char** argv) {
    long total = argc > 1 ? std::atol(argv[1]) : 100000;
    std::vector<std::string> lines;
    long long state = 42;
    for (long i = 0; i < total; ++i) {
        state = (state * 1103515245 + 12345) % 2147483648LL;
        lines.push_back("item " + std::to_string(i) + ": " + std::to_string(state / 65536));
    }
    std::string text;
    for (size_t i = 0; i < lines.size(); ++i) {
        if (i > 0) text += '\n';
        text += lines[i];
    }
    std::vector<std::string> pieces;
    size_t start = 0;
    while (true) {
        size_t end = text.find('\n', start);
        pieces.push_back(text.substr(start, end == std::string::npos ? std::string::npos : end - start));
        if (end == std::string::npos) break;
        start = end + 1;
    }
    long sevens = 0, characters = 0;
    for (const std::string& line : pieces) {
        characters += characters_in(line);
        if (line.find('7') != std::string::npos) ++sevens;
    }
    std::printf("%ld characters, %ld in lines, %ld lines with a 7\n", characters_in(text), characters, sevens);
}
