mod day01;
mod day02;

use std::io::{self, Write};

fn main() {
    loop {
        command_prompt();
    }
}

fn command_prompt() {
    let selection: i32 = get_day();

    match selection {
        1 => day01::run(get_part()),
        2 => day02::run(get_part()),
        _ => println!("Day not implemented yet!"),
    };
}

/*
    Returns the user input once they enter a number between 1 and 30 (inclusive).w
*/
fn get_day() -> i32 {
    loop {
        print!("Enter the day you'd like to run (1 to 30): ");
        io::stdout().flush().unwrap();
    
        let mut selection: String = String::new();
    
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read stdin");
    
        match selection.trim().parse() {
            Ok(num) => {
                if num > 30 || num < 1 {
                    println!("Error: Please enter a number from 1 to 30!");
                    continue;
                }
                break num
            },
            Err(_) => {
                println!("Error: Please enter a number from 1 to 30!");
                continue;
            }
        };
    }
}

/*
    Returns the user input once they enter either 1 or 2.
*/
fn get_part() -> i32 {
    loop {
        print!("Enter the part (1 or 2): ");
        io::stdout().flush().unwrap();

        let mut part: String = String::new();

        io::stdin()
            .read_line(&mut part)
            .expect("Failed to read stdin");

        match part.trim().parse() {
            Ok(num) => {
                if num != 1 && num != 2 {
                    println!("Error: Please enter 1 or 2!");
                    continue;
                }
                break num
            },
            Err(_) => {
                println!("Error: Please enter 1 or 2!");
                continue;
            }
        }
    }
}
