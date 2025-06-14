use std::collections::HashMap;
use std::io;

fn main() {
    let mut tape: Vec<u8> = vec![0; 30_000];
    let mut cell_index = 0_usize;
    let program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    let mut ip = 0;

    let mut loop_table = HashMap::new();
    let mut loop_stack: Vec<usize> = Vec::new();
    let program_chars: Vec<char> = program.chars().collect();

    for (i, &c) in program_chars.iter().enumerate() {
        if c == '[' {
            loop_stack.push(i);
        } else if c == ']' {
            let start = loop_stack.pop().unwrap();
            loop_table.insert(start, i);
            loop_table.insert(i, start);
        }
    }

    let mut input_buffer = String::new();
    let mut input_index = 0;

    while ip < program_chars.len() {
        match program_chars[ip] {
            '>' => {
                cell_index += 1;
                if cell_index >= tape.len() {
                    tape.push(0);
                }
            }
            '<' => {
                if cell_index > 0 {
                    cell_index -= 1;
                }
            }
            '+' => tape[cell_index] = tape[cell_index].wrapping_add(1),
            '-' => tape[cell_index] = tape[cell_index].wrapping_sub(1),
            '.' => print!("{}", tape[cell_index] as char),
            ',' => {
                if input_index >= input_buffer.len() {
                    input_buffer.clear();
                    io::stdin().read_line(&mut input_buffer).unwrap();
                    input_index = 0;
                }
                if let Some(c) = input_buffer.chars().nth(input_index) {
                    tape[cell_index] = c as u8;
                    input_index += 1;
                }
            }
            '[' => {
                if tape[cell_index] == 0 {
                    ip = *loop_table.get(&ip).unwrap();
                }
            }
            ']' => {
                if tape[cell_index] != 0 {
                    ip = *loop_table.get(&ip).unwrap();
                }
            }
            '#' => {
                println!("Debug: cell[{}] = {}", cell_index, tape[cell_index]);
            }
            _ => {}
        }
        ip += 1;
    }
}
