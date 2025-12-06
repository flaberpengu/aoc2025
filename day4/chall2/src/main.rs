use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");
    let mut grid_lines = contents.split('\n');
    let first_line = grid_lines.next().unwrap();
    let mut grid: Vec<Vec<u8>> = vec![];

    //Handle first line of 0s + first actual line
    grid.push(vec![0; first_line.len() + 2]);
    grid.push(vec![0]);
    for char in first_line.chars(){
        grid[1].push(if char == '.'{0} else {1});
    }
    grid[1].push(0);

    //Iterate over all other lines to build grid
    let mut final_index = 1;
    for line in grid_lines{
        if line.len() > 1{
            grid.push(vec![0]);
            final_index += 1;
            for char in line.chars(){
                grid[final_index].push(if char == '.'{0} else {1});
            }
            grid[final_index].push(0);
        }
    }

    //Add final line
    grid.push(vec![0; grid[0].len()]);

    //Iterate over all (real) entries in grid to determine if theyre valid until no more can be removed
    let mut valid_rolls: u64 = 0;
    let mut surrounding_rolls: u8;
    let mut rolls_for_removal: Vec<(usize, usize)> = vec![(0,0)];
    while rolls_for_removal.len() > 0{
        rolls_for_removal = vec![];
        for i in 1..grid.len()-1{
            for j in 1..grid[i].len()-1{
                if grid[i][j] == 1{
                    surrounding_rolls = grid[i-1][j-1] + grid[i-1][j] + grid[i-1][j+1] + grid[i][j-1] + grid[i][j+1] + grid[i+1][j-1] + grid[i+1][j] + grid[i+1][j+1];
                    if surrounding_rolls < 4{
                        valid_rolls += 1;
                        rolls_for_removal.push((i, j));
                    }
                }
            }
        }
        //Set all cleared rolls in grid to 0
        for (roll_x, roll_y) in &rolls_for_removal{
            grid[*roll_x][*roll_y] = 0;
        }
    }
    println!("Numer of accessible rolls: {}", valid_rolls);
}
