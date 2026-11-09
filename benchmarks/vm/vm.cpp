/*
VM: run a little program on a stack machine, an interpreter that decodes one
instruction at a time. The program adds up i * i modulo a prime for every i
below n, seventeen instructions for each i.
*/
#include <cstdio>
#include <cstdlib>
#include <vector>

enum Code : long long { PUSH, LOAD, STORE, ADD, MULTIPLY, REMAINDER, LESS, JUMP_UNLESS, JUMP, STOP };

static void run(const std::vector<long long>& code, std::vector<long long>& variables) {
    std::vector<long long> stack(16);
    size_t top = 0;
    size_t at = 0;
    while (true) {
        switch (code[at]) {
            case PUSH:
                stack[top++] = code[at + 1];
                at += 2;
                break;
            case LOAD:
                stack[top++] = variables[code[at + 1]];
                at += 2;
                break;
            case STORE:
                variables[code[at + 1]] = stack[--top];
                at += 2;
                break;
            case ADD:
                --top;
                stack[top - 1] = stack[top - 1] + stack[top];
                at += 1;
                break;
            case MULTIPLY:
                --top;
                stack[top - 1] = stack[top - 1] * stack[top];
                at += 1;
                break;
            case REMAINDER:
                --top;
                stack[top - 1] = stack[top - 1] % stack[top];
                at += 1;
                break;
            case LESS:
                --top;
                stack[top - 1] = stack[top - 1] < stack[top] ? 1 : 0;
                at += 1;
                break;
            case JUMP_UNLESS:
                --top;
                at = stack[top] == 0 ? static_cast<size_t>(code[at + 1]) : at + 2;
                break;
            case JUMP:
                at = static_cast<size_t>(code[at + 1]);
                break;
            default:
                return;
        }
    }
}

int main(int argc, char** argv) {
    long long limit = argc > 1 ? std::atoll(argv[1]) : 1000;
    /* Variable 0 is i, 1 the total, and 2 the limit. */
    std::vector<long long> code = {
        PUSH, 0, STORE, 0,
        PUSH, 0, STORE, 1,
        LOAD, 0, LOAD, 2, LESS,
        JUMP_UNLESS, 37,
        LOAD, 1, LOAD, 0, LOAD, 0, MULTIPLY, ADD, PUSH, 1000000007, REMAINDER, STORE, 1,
        LOAD, 0, PUSH, 1, ADD, STORE, 0,
        JUMP, 8,
        STOP,
    };
    std::vector<long long> variables = {0, 0, limit};
    run(code, variables);
    std::printf("%lld for %lld\n", variables[1], limit);
}
