use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let lines = contents.split('\n');
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut in_range_mode = true;
    let mut fresh_ingredients: u64 = 0;
    for line in lines{
        //If blank middle line, switch mode
        if line.len() < 1{
            in_range_mode = false;
        }
        else{
            //Handle case where given input line is a range
            if in_range_mode{
                let range_bounds: Vec<&str> = line.split('-').collect();
                ranges.push((range_bounds[0].parse::<u64>().expect("Failed to cast lower bound to u64"), range_bounds[1].parse::<u64>().expect("Failed to cast upper bound to u64")));
            }
            //Handle case where given input line is an ingredient ID
            else{
                let id = line.parse::<u64>().expect("Failed to cast ingredient ID to u64");
                let mut in_range: bool = false;
                for (lower, upper) in &ranges{
                    if id >= *lower && id <= *upper{
                        in_range = true;
                    }
                }
                if in_range{
                    fresh_ingredients += 1;
                }
            }
        }
    }
    println!("Number of fresh ingredients: {}", fresh_ingredients);
}
