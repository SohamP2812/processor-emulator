use std::io::Write;

use computer::{Computer, GENERAL_REGISTER_NAMES, SPECIAL_REGISTER_NAMES};

mod computer;

fn main() {
    let mut computer = Computer::new();

    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();

        let read_line_result = std::io::stdin().read_line(&mut buffer);
        match read_line_result {
            Ok(_) => {},
            Err(error) => panic!("Problem reading line: {:?}", error),
        };

        let result = execute_command(&mut computer, buffer);

        if result == 0 {
            break;
        }
    }
}

fn execute_command(computer: &mut Computer, command: String) -> u8 {
    let tokens: Vec<&str> = command.trim().split(" ").collect();

    if tokens.len() == 0 {
        println!("Empty command");
    } 
    
    let command = tokens[0];

    match command {
        "SET" => {
            if tokens.len() != 3 {
                println!("Invalid number of arguments");
            }

            if index_if_contains(tokens[1], GENERAL_REGISTER_NAMES, GENERAL_REGISTER_NAMES.len()) != -1 {
                computer.cpu.general_registers[index_if_contains(tokens[1], GENERAL_REGISTER_NAMES, GENERAL_REGISTER_NAMES.len()) as usize].value = u8::from_str_radix(tokens[2], 16).unwrap();
            } else if index_if_contains(tokens[1], SPECIAL_REGISTER_NAMES, SPECIAL_REGISTER_NAMES.len()) != -1 {
                computer.cpu.special_registers[index_if_contains(tokens[1], SPECIAL_REGISTER_NAMES, SPECIAL_REGISTER_NAMES.len()) as usize].value = u16::from_str_radix(tokens[2], 16).unwrap();
            } else {
                println!("Invalid register code");
            }
        }, 
        "GET" => {
            if tokens.len() != 2 {
                println!("Invalid number of arguments");
            }

            if index_if_contains(tokens[1], GENERAL_REGISTER_NAMES, GENERAL_REGISTER_NAMES.len()) != -1 {
                println!("{:#04X}", computer.cpu.general_registers[index_if_contains(tokens[1], GENERAL_REGISTER_NAMES, GENERAL_REGISTER_NAMES.len()) as usize].value);
            } else if index_if_contains(tokens[1], SPECIAL_REGISTER_NAMES, SPECIAL_REGISTER_NAMES.len()) != -1 {
                println!("{:#06X}", computer.cpu.special_registers[index_if_contains(tokens[1], SPECIAL_REGISTER_NAMES, SPECIAL_REGISTER_NAMES.len()) as usize].value);
            } else {
                println!("Invalid register code");
            }
        },
        "READ" => {
            if tokens.len() != 2 {
                println!("Invalid number of arguments");
            }

            let address = u16::from_str_radix(tokens[1], 16).unwrap();

            println!("{:#04X}", computer.memory.read(address));
        },
        "WRITE" => {
            if tokens.len() != 3 {
                println!("Invalid number of arguments");
            }

            let address = u16::from_str_radix(tokens[1], 16).unwrap();
            let data = u8::from_str_radix(tokens[2], 16).unwrap();

            computer.memory.write(address, data);
        },
        "DUMP" => {
            if tokens.len() != 1 {
                println!("Invalid number of arguments");
            }

            computer.dump();
        },
        "MEMDUMP" => {
            if tokens.len() != 3 {
                println!("Invalid number of arguments");
            }

            let address = u16::from_str_radix(tokens[1], 16).unwrap();

            let bytes = tokens[2].parse::<usize>().unwrap();

            computer.mem_dump(address, bytes);
        },
        "END" => {
            return 0;
        }
        _ => {}
    }

    1
}

fn index_if_contains<T: std::cmp::PartialEq>(target: T, array: &[T], size: usize) -> isize {
    for i in 0..size {
        if array[i] == target {
            return i as isize;
        }
    } 

    return -1;
}