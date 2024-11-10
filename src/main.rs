use rand::prelude::*;
use std::io;
fn main() {
    let num = generate_number();
    game_loop(num);
}
fn generate_number() -> u32 {
    let mut rng = thread_rng();
    let generated_num = rng.gen_range(1..1000);
    generated_num
}
fn game_loop(num: u32) {
    loop {
        let user_input = get_input();
        match user_input {
            Some(value) => {
                println!("You entered {}", value);
                if value < num {
                    println!("which is less than the correct number");
                } else if value > num {
                    println!("which is greater than the correct number");
                } else {
                    println!("You won");
                    break;
                }
            }
            None => {
                println!("Please enter a valid number");
            }
        }
    }
}
fn get_input() -> Option<u32> {
    let mut user_input: String = String::new();
    let res = io::stdin().read_line(&mut user_input);
    match res {
        Ok(_size) => {
            return convert_to_num(&user_input.trim());
        }
        Err(_err) => {
            return None;
        }
    }
}
fn convert_to_num(input: &str) -> Option<u32> {
    let mut num: u32 = 0;
    for i in input.chars() {
        if i as u32 >= '0' as u32 && i as u32 <= '9' as u32 {
            let unicode_value: u32 = i as u32 - '0' as u32;
            num = num * 10 + unicode_value;
        } else {
            return None;
        }
    }
    Some(num)
}
