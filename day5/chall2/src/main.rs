use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let mut lines = contents.split('\n');
    let mut in_range_mode = true;
    let mut line: &str;
    let mut ranges: Vec<(u64, u64)> = vec![];

    //Extract ranges from input
    while in_range_mode{
        line = lines.next().expect("Failed to get next line");
        if line.len() < 1{
            in_range_mode = false;
        }
        else{
            let range_bounds: Vec<&str> = line.split('-').collect();
            let lower = range_bounds[0].parse::<u64>().expect("Failed to cast lower bound to u64");
            let upper = range_bounds[1].parse::<u64>().expect("Failed to cast upper bound to u64");
            ranges.push((lower, upper));
        }
    }

    //Taking approach of range shrinking!
    //If range overlaps with others, shorten it and add to new ranges
    //If range subsumes another, split into two ranges either end and add to initial ranges to try for overlaps
    //If range entirely within another, don't add to new ranges
    let mut adjusted_ranges: Vec<(u64, u64)> = vec![];
    let mut range_index: usize = 0;
    let mut push_range: bool;
    while ranges.len() > 0{
        range_index %= ranges.len();
        push_range = true;
        let mut modified_lower = ranges[range_index].0;
        let mut modified_upper = ranges[range_index].1;
        for range in &adjusted_ranges{
            if modified_lower >= range.0  && modified_lower <= range.1 && modified_upper >= range.0 && modified_upper <= range.1{
                push_range = false;
            }
            else if modified_lower >= range.0 && modified_lower <= range.1 && modified_upper > range.1{
                modified_lower = range.1 + 1;
            }
            else if modified_upper >= range.0 && modified_upper <= range.1 && modified_lower < range.0{
                modified_upper = range.0 - 1;
            }
            else if modified_lower < range.0 && modified_upper > range.1{
                ranges.push((modified_lower, range.0-1));
                ranges.push((range.1+1, modified_upper));
                push_range = false;
            }
        }
        if push_range{
            adjusted_ranges.push((modified_lower, modified_upper));
        }
        ranges.remove(range_index);
        range_index += 1;
    }

    //Sum all ranges
    let mut sum_fresh_ingredients: u64 = 0;
    for range in &adjusted_ranges{
        sum_fresh_ingredients += (range.1 - range.0) + 1;
    }
    println!("Total number of fresh ingredients possible: {}", sum_fresh_ingredients);
}