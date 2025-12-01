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

//Take in a direction to turn, the number of spaces to turn, and the position. 
//Return the updated position and the number of times it passed zero
fn turn(direction: char, turn_amount: i32, mut pos: i32) -> (i32, u32) {
    let initial_pos = pos;
    let mut num_times_zero: u32 = 0;

    //If we wrap around left, add 1 to counter for every time we pass
    //If we start at 0 though, we want to remove 1 (ie 0 L5 -> 95 should result in 0 added to counter, 0 L105 -> 95 should add 1, etc.)
    //But we want 0 L100 -> 0 to be 1, not 0, so need final pos == 0 check
    if direction == 'L'{
        pos -= turn_amount;
        while pos < 0{
            pos += 100;
            num_times_zero += 1;
        }
        if initial_pos == 0{
            num_times_zero -= 1;
        }
    }
    //If we wrap around right, add 1 to counter for every time we pass
    //If we end on 0, we'll add 1 in final check *and* 1 in the wrap, so subtract 1 in anticipation (i.e. 90 R10 -> 0 should add 1, not 2)
    //But we want 0 R100 -> 0 to still add 1, so need final pos == 0 check
    else if direction == 'R'{
        pos += turn_amount;
        while pos >= 100{
            pos -= 100;
            num_times_zero += 1;
        }
        if pos == 0{
            num_times_zero -= 1;
        }
    }
    if pos == 0{
        num_times_zero += 1;
    }
    return (pos, num_times_zero);
}