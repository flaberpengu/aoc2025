use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let bank_lines = contents.split('\n');
    let mut digit: u32;
    let mut first_digit: u32;
    let mut second_digit: u32;
    let mut index: usize;
    let mut sum: u32 = 0;
    let mut max_joltage: String;
    for bank in bank_lines{
        if bank.len() > 1{
            index = 0;
            first_digit = 0;
            second_digit = 0;
            for digit_char in bank.chars(){
                digit = digit_char.to_string().parse::<u32>().expect("Failed to cast to u32");
                if digit > first_digit && index+1 < bank.len(){
                    first_digit = digit;
                    second_digit = 0;
                }
                else if digit > second_digit{
                    second_digit = digit;
                }
                index += 1;
            }
            max_joltage = format!("{}{}", first_digit, second_digit);
            sum += max_joltage.parse::<u32>().unwrap();
        }
    }
    println!("Sum: {}", sum);
}
