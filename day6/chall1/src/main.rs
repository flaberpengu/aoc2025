use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let lines = contents.split('\n');

    //Get input
    let mut inputs: Vec<Vec<u64>> = vec![];
    let mut operators: Vec<char> = vec![];
    let mut input_index: usize = 0;
    for line in lines{
        if line.len() > 1{
            let mut parts = line.split(' ').filter(|x| !x.is_empty());
            let first_part = parts.next().unwrap();
            if first_part == "*" || first_part == "+"{
                operators.push(first_part.chars().nth(0).expect("Failed to get operator character"));
                for operator in parts{
                    operators.push(operator.chars().nth(0).expect("Failed to get operator character"));
                }
            }
            else{
                inputs.push(vec![first_part.parse::<u64>().expect("Failed to cast to u64")]);
                for input in parts{
                    inputs[input_index].push(input.parse::<u64>().expect("Failed to cast to u64"));
                }
                input_index += 1;
            }
        }
    }

    //Do problems
    let mut grand_total: u64 = 0;
    for j in 0..operators.len(){
        let mut problem_total: u64;
        if operators[j] == '+'{
            problem_total = 0;
        }
        else{
            problem_total = 1;
        }
        for i in 0..inputs.len(){
            if operators[j] == '+'{
                problem_total += inputs[i][j];
            }
            else{
                problem_total *= inputs[i][j];
            }
        }
        grand_total += problem_total;
    }
    println!("Grand total: {}", grand_total);
}
