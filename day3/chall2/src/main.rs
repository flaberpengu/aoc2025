use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let bank_lines = contents.split('\n');
    let mut digit: u32;
    let mut digits: Vec<u32>;
    let mut index: usize;
    let mut sum: u128 = 0;
    let mut max_joltage: String;
    for bank in bank_lines{
        if bank.len() > 1{
            index = 0;
            digits = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            for digit_char in bank.chars(){
                digit = digit_char.to_string().parse::<u32>().expect("Failed to cast to u32");
                if digit > digits[0] && index+11 < bank.len(){
                    digits[0] = digit;
                    for i in 1..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[1] && index+10 < bank.len(){
                    digits[1] = digit;
                    for i in 2..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[2] && index+9 < bank.len(){
                    digits[2] = digit;
                    for i in 3..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[3] && index+8 < bank.len(){
                    digits[3] = digit;
                    for i in 4..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[4] && index+7 < bank.len(){
                    digits[4] = digit;
                    for i in 5..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[5] && index+6 < bank.len(){
                    digits[5] = digit;
                    for i in 6..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[6] && index+5 < bank.len(){
                    digits[6] = digit;
                    for i in 7..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[7] && index+4 < bank.len(){
                    digits[7] = digit;
                    for i in 8..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[8] && index+3 < bank.len(){
                    digits[8] = digit;
                    for i in 9..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[9] && index+2 < bank.len(){
                    digits[9] = digit;
                    for i in 10..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[10] && index+1 < bank.len(){
                    digits[10] = digit;
                    for i in 11..12{
                        digits[i] = 0;
                    }
                }
                else if digit > digits[11]{
                    digits[11] = digit;
                }
                index += 1;
            }
            max_joltage = String::from("");
            for i in 0..12{
                max_joltage += &digits[i].to_string();
            }
            sum += max_joltage.parse::<u128>().unwrap();
        }
    }
    println!("Sum: {}", sum);
}
