use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let mut lines_str: Vec<&str> = contents.split('\n').collect();

    //Remove last line if blank
    if lines_str[lines_str.len()-1].len() <= 1{
        lines_str.pop();
    }

    //Convert lines into vectors of chars
    let mut lines: Vec<Vec<char>> = vec![];
    for line in lines_str{
        lines.push(line.chars().collect());
    }

    //Find starting beam position
    let mut beam_positions: Vec<usize> = vec![];
    for i in 0..lines[0].len(){
        if lines[0][i] == 'S'{
            beam_positions.push(i);
        }
    }

    //Per line, check whether current beams will split next line. If so, append split beams new list
    //If not, append existing beam to new list
    let mut total_splits: u64 = 0;
    for i in 0..lines.len()-1{
        let mut new_beam_positions: Vec<usize> = vec![];
        for pos in beam_positions.iter(){
            if lines[i+1][*pos] == '^'{
                if *pos != 0 && !new_beam_positions.contains(&(pos-1)){
                    new_beam_positions.push(pos-1);
                }
                if *pos != lines[i+1].len()-1 && !new_beam_positions.contains(&(pos+1)){
                    new_beam_positions.push((*pos)+1);
                }
                total_splits += 1;
            }
            else if !new_beam_positions.contains(pos){
                new_beam_positions.push(*pos);
            }
        }
        beam_positions = new_beam_positions.clone();
    }
    println!("Total splits: {}", total_splits);
}
