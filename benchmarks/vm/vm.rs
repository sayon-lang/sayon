/*
VM: run a little program on a stack machine, an interpreter that decodes one
instruction at a time. The program adds up i * i modulo a prime for every i
below n, seventeen instructions for each i.
*/

const PUSH: i64 = 0;
const LOAD: i64 = 1;
const STORE: i64 = 2;
const ADD: i64 = 3;
const MULTIPLY: i64 = 4;
const REMAINDER: i64 = 5;
const LESS: i64 = 6;
const JUMP_UNLESS: i64 = 7;
const JUMP: i64 = 8;
const STOP: i64 = 9;

fn run(code: &[i64], variables: &mut [i64]) {
    let mut stack = vec![0i64; 16];
    let mut top = 0;
    let mut at = 0;
    loop {
        match code[at] {
            PUSH => {
                stack[top] = code[at + 1];
                top += 1;
                at += 2;
            }
            LOAD => {
                stack[top] = variables[code[at + 1] as usize];
                top += 1;
                at += 2;
            }
            STORE => {
                top -= 1;
                variables[code[at + 1] as usize] = stack[top];
                at += 2;
            }
            ADD => {
                top -= 1;
                stack[top - 1] += stack[top];
                at += 1;
            }
            MULTIPLY => {
                top -= 1;
                stack[top - 1] *= stack[top];
                at += 1;
            }
            REMAINDER => {
                top -= 1;
                stack[top - 1] %= stack[top];
                at += 1;
            }
            LESS => {
                top -= 1;
                stack[top - 1] = (stack[top - 1] < stack[top]) as i64;
                at += 1;
            }
            JUMP_UNLESS => {
                top -= 1;
                at = if stack[top] == 0 { code[at + 1] as usize } else { at + 2 };
            }
            JUMP => at = code[at + 1] as usize,
            _ => return,
        }
    }
}

fn main() {
    let limit: i64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(1000);
    /* Variable 0 is i, 1 the total, and 2 the limit. */
    let code = [
        PUSH, 0, STORE, 0,
        PUSH, 0, STORE, 1,
        LOAD, 0, LOAD, 2, LESS,
        JUMP_UNLESS, 37,
        LOAD, 1, LOAD, 0, LOAD, 0, MULTIPLY, ADD, PUSH, 1000000007, REMAINDER, STORE, 1,
        LOAD, 0, PUSH, 1, ADD, STORE, 0,
        JUMP, 8,
        STOP,
    ];
    let mut variables = [0, 0, limit];
    run(&code, &mut variables);
    println!("{} for {limit}", variables[1]);
}
