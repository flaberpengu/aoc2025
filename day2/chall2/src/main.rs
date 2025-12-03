use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    // Get ranges from file
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let ranges_line = contents.split('\n').next().expect("Error getting ranges line");
    let mut ranges: Vec<(u128, u128)> = vec![];
    let mut range_vals: Vec<u128>;
    let mut max_val: u128 = 0;
    for range in ranges_line.split(','){
        range_vals = range.split('-').map(|x| x.parse::<u128>().expect("Error casting to u128")).collect();
        ranges.push((range_vals[0], range_vals[1]));
        if range_vals[1] > max_val{
            max_val = range_vals[1];
        }
    }

    //Find max number to try duplicating
    let mut max_val_str = max_val.to_string();
    let max_to_try: usize;
    if max_val_str.len() % 2 == 0{
        max_to_try = max_val_str[..max_val_str.len() / 2].parse::<usize>().unwrap();
    }
    else{
        max_val *= 10;
        max_val_str = max_val.to_string();
        max_to_try = max_val_str[..max_val_str.len() / 2].parse::<usize>().unwrap();
    }

    //Sort ranges, then try every possible up to max
    ranges.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut sum: u128 = 0;
    let mut try_val_str: String;
    let mut try_val: u128;
    let mut found: bool;
    let mut range_index: usize;
    let mut found_nums: Vec<u128> = vec![];
    for x in 1..max_to_try{
        try_val_str = String::from("");
        try_val_str += &x.to_string();
        try_val_str += &x.to_string();
        try_val = try_val_str.parse::<u128>().unwrap();
        while try_val <= max_val{
            if !found_nums.contains(&try_val){
                range_index = 0;
                found = false;
                while range_index < ranges.len() && !found{
                    if try_val >= ranges[range_index].0 && try_val <= ranges[range_index].1{
                        sum += try_val;
                        found = true;
                        found_nums.push(try_val);
                    }
                    range_index += 1;
                }
            }
            try_val_str += &x.to_string();
            try_val = try_val_str.parse::<u128>().unwrap();
        }
    }
    println!("Sum: {}", sum);
}