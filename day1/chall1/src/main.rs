use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let commands = contents.split('\n');
    let mut pos: i32 = 50i32;
    let mut chars: std::str::Chars<'_>;
    let mut direction: Option<char>;
    let mut digit_str: String;
    let mut turn_amount: i32;
    let mut increase_amount: u32;
    let mut num_times_zero: u32 = 0;
    for command in commands{
        chars = command.chars();
        direction = chars.next();
        match direction{
            None => {},
            Some(x) => {
                digit_str = chars.collect();
                turn_amount = digit_str.parse::<i32>().unwrap();
                
                (pos, increase_amount) = turn(x, turn_amount, pos);
                num_times_zero += increase_amount;
            }
        }
    }
    println!("Password: {}", num_times_zero);
}

//Take in number of spaces to turn and a direction, return the updated position and the number of times it passed 0
fn turn(direction: char, turn_amount: i32, mut pos: i32) -> (i32, u32){
    if direction == 'L'{
        pos -= turn_amount;
        while pos < 0{
            pos += 100;
        }
    }
    else if direction == 'R'{
        pos += turn_amount;
        while pos >= 100{
            pos -= 100;
        }
    }
    if pos == 0 {
        return (pos, 1);
    }
    return (pos, 0);
}