use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let lines: Vec<&str> = contents.split('\n').collect();

    //Split into a vector of character iterators and a vector of operators (both reversed)
    let mut num_lines_chars = vec![];
    let mut rev_operators: Vec<char> = vec![];
    for line in lines{
        if line.len() > 1{
            let first_char = line.split(' ').filter(|x| !x.is_empty()).collect::<Vec<&str>>()[0].chars().nth(0).expect("Failed to get first character of line");
            if first_char == '*' || first_char == '+'{
                let mut operators = line.split(' ').filter(|x| !x.is_empty()).collect::<Vec<&str>>();
                operators.reverse();
                for operator in operators.iter(){
                    rev_operators.push(operator.chars().nth(0).expect("Failed to get operator"));
                }
            }
            else{
                num_lines_chars.push(line.chars().rev());
            }
        }
    }

    //Iterate over all input strings simultaneously, generating strings for integers as we go. If all aligned spaces occur, move onto next problem
    let mut problem_inputs: Vec<Vec<u64>> = vec![vec![]];
    let mut problem_index: usize = 0;
    let mut number_string: String;
    for _ in num_lines_chars[0].clone(){
        //Get characters in same row from each column in order, append to a string if not a blank space
        //Cast to u64 if non-empty (i.e. theres at least one digit), else move onto next problem (as we have all blank space vertical line)
        number_string = String::from("");
        for i in 0..num_lines_chars.len(){
            let char_to_add = num_lines_chars[i].nth(0).expect("Failed to get character").to_string();
            number_string += if char_to_add == " " {""} else {&char_to_add};
        }
        
        if !number_string.is_empty(){
            problem_inputs[problem_index].push(number_string.parse::<u64>().expect("Failed to cast to u64"));
        }
        else{
            problem_index += 1;
            problem_inputs.push(vec![]);
        }
    }

    assert!(problem_inputs.len() == rev_operators.len());

    //Now do problems
    let mut grand_total: u64 = 0;
    for i in 0..rev_operators.len(){
        let mut problem_total: u64;
        if rev_operators[i] == '+'{
            problem_total = 0;
        }
        else{
            problem_total = 1;
        }

        for j in 0..problem_inputs[i].len(){
            if rev_operators[i] == '+'{
                problem_total += problem_inputs[i][j];
            }
            else{
                problem_total *= problem_inputs[i][j];
            }
        }
        grand_total += problem_total;
    }
    println!("Grand total: {}", grand_total);


}
