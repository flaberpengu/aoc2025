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

    //Beam positions will hold the number of beams per position for the current row
    let mut beam_positions: Vec<u64> = vec![];

    //Find starting beam position(s) - initialise beam positions
    for i in 0..lines[0].len(){
        if lines[0][i] == 'S'{
            beam_positions.push(1);
        }
        else{
            beam_positions.push(0);
        }
    }

    //Per line, check whether current beams will split in the next "step" (i.e. will collide in the next line)
    //If they will, add count to both pos-1 and pos+1 for next line (i.e. left and right of ^)
    //If not, leave total 
    for i in 0..lines.len()-1{
        let mut new_beam_positions = beam_positions.clone();
        for (pos, count) in beam_positions.iter().enumerate(){
            if lines[i+1][pos] == '^'{
                if pos != 0{
                    new_beam_positions[pos-1] = new_beam_positions[pos-1] + count;
                }
                if pos != lines[i+1].len()-1{
                    new_beam_positions[pos+1] = new_beam_positions[pos+1] + count;
                }
                new_beam_positions[pos] = new_beam_positions[pos] - count;
            }
        }
        beam_positions = new_beam_positions.clone();
    }

    //Calculate total timelines as sum of all on-going beams
    let mut total_timelines: u64 = 0;
    for count in beam_positions.iter(){
        total_timelines += count;
    }
    println!("Total timelines: {}", total_timelines);
}