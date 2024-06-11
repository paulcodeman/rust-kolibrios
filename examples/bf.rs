#![no_std]
#![no_main]

use kos::{dll::Console, threads::exit};
extern crate alloc;

const TAPE_SIZE: usize = 30000;

fn interpret_bf(code: &str) {
    let mut tape = [0u8; TAPE_SIZE];
    let mut ptr = 0;
    let mut pc = 0;
    let mut loop_stack = [0usize; 16];
    let mut loop_stack_ptr = 0;

    while pc < code.len() {
        match code.as_bytes()[pc] as char {
            '>' => ptr = (ptr + 1) % TAPE_SIZE,
            '<' => ptr = (ptr + TAPE_SIZE - 1) % TAPE_SIZE,
            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            '.' => {
                let con_lib = Console::import(None).unwrap();
                con_lib.write_char(tape[ptr] as char);
            }
            ',' => {
                let con_lib = Console::import(None).unwrap();
                tape[ptr] = con_lib.gets().as_bytes()[0];
            }
            '[' => {
                if tape[ptr] == 0 {
                    let mut open_brackets = 1;
                    while open_brackets > 0 {
                        pc += 1;
                        if code.as_bytes()[pc] as char == '[' {
                            open_brackets += 1;
                        } else if code.as_bytes()[pc] as char == ']' {
                            open_brackets -= 1;
                        }
                    }
                } else {
                    loop_stack[loop_stack_ptr] = pc;
                    loop_stack_ptr += 1;
                }
            }
            ']' => {
                if tape[ptr] != 0 {
                    pc = loop_stack[loop_stack_ptr - 1];
                } else {
                    loop_stack_ptr -= 1;
                }
            }
            _ => {}
        }
        pc += 1;
    }
}

#[no_mangle]
pub fn kol_main() {
    let con_lib = Console::import(None).unwrap();
    con_lib.init(u32::MAX, u32::MAX, u32::MAX, u32::MAX, c"BrainF*ck");

    loop {
        con_lib.write_string("Enter BF code: ");
        let code = con_lib.gets();
        interpret_bf(&code);
        con_lib.write_string("\n");
    }

    con_lib.exit(false);
    exit();
}