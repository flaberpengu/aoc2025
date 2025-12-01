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
    let mut num_times_zero: u32 = 0;
    for command in commands{
        chars = command.chars();
        direction = chars.next();
        digit_str = chars.collect();
        turn_amount = digit_str.parse::<i32>().unwrap();
        if direction == Some('L'){
            pos -= turn_amount;
            while pos < 0{
                pos += 100;
            }
        }
        else{
            pos += turn_amount;
            while pos >= 100{
                pos -= 100;
            }
        }
        if pos == 0{
            num_times_zero += 1;
        }
    }
    println!("Password: {}", num_times_zero);
}
