// Word count: make up a long text from a small set of syllables, count how
// often each word comes up, and list the most common.
#include <algorithm>
#include <cstdio>
#include <cstdlib>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

int main(int argc, char** argv) {
    long total = argc > 1 ? std::atol(argv[1]) : 100000;
    const char* parts[] = {"ka", "lo", "mi", "ne", "ru", "sa", "ti", "vo", "ze", "pa"};
    std::unordered_map<std::string, long> counts;
    long long state = 42;
    for (long word_index = 0; word_index < total; ++word_index) {
        state = (state * 1103515245 + 12345) % 2147483648LL;
        std::string word;
        long syllables = 1 + state / 65536 % 3;
        for (long part = 0; part < syllables; ++part) {
            state = (state * 1103515245 + 12345) % 2147483648LL;
            word += parts[state / 65536 % 10];
        }
        ++counts[word];
    }
    std::vector<std::pair<std::string, long>> found(counts.begin(), counts.end());
    std::sort(found.begin(), found.end(), [](const auto& a, const auto& b) {
        return a.second > b.second || (a.second == b.second && a.first < b.first);
    });
    std::printf("%zu different words\n", counts.size());
    for (int index = 0; index < 5; ++index) std::printf("%s %ld\n", found[index].first.c_str(), found[index].second);
}
