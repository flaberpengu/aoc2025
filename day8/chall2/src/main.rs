use std::fs;

const FILENAME: &str = "input.txt";

#[derive(Debug)]
struct JunctionBox{
    x: i64,
    y: i64,
    z: i64,
    circuit: Option::<usize>,
}

// ===================== HELPER FUNCTIONS =====================

fn insert_pair_into_ordered_vec(elem: (usize, usize, f64), vec: &mut Vec<(usize, usize, f64)>){
    if vec.len() == 0{
        vec.push(elem);
    }
    else{
        let mut index: usize = 0;
        let mut inserted: bool = false;
        while !inserted && index < vec.len(){
            if elem.2 <= vec[index].2{
                vec.insert(index, elem);
                // vec.pop();
                inserted = true;
            }
            index += 1;
        }
        if !inserted{
            vec.push(elem);
        }
    }
}

// ===================== END OF HELPER FUNCTIONS =====================

fn main() {
    let contents = fs::read_to_string(FILENAME).expect("Failed to read file :(");

    //Extract junction boxes
    let mut junction_boxes: Vec<JunctionBox> = vec![];
    for jbox in contents.split('\n'){
        if jbox.len() > 1{
            let coords: Vec<&str> = jbox.split(',').collect();
            junction_boxes.push(JunctionBox{
                x: coords[0].parse::<i64>().expect("Failed to parse x coord"),
                y: coords[1].parse::<i64>().expect("Failed to parse y coord"),
                z: coords[2].parse::<i64>().expect("Failed to parse z coord"),
                circuit: None,
            });
        }
    }

    //Find distance between all pairs of boxes
    //If in top NUM_PAIRS shortest distances, insert into list in order + remove largest distance
    let mut shortest_pairs: Vec<(usize, usize, f64)> = vec![];
    for i in 0..junction_boxes.len(){
        for j in (i+1)..junction_boxes.len(){
            let a: u64 = junction_boxes[i].x.abs_diff(junction_boxes[j].x);
            let b: u64 = junction_boxes[i].y.abs_diff(junction_boxes[j].y);
            let c: u64 = junction_boxes[i].z.abs_diff(junction_boxes[j].z);
            let dist: f64 = ((a.pow(2) + b.pow(2) + c.pow(2)) as f64).sqrt();
            insert_pair_into_ordered_vec((i, j, dist), &mut shortest_pairs);
        }
    }

    //Create list of circuits after we have all connections we care about
    let mut new_circuit_id: usize = 0;
    let mut pair_index: usize = 0;
    let mut pair: &(usize, usize, f64);
    let mut last_pair: (usize, usize) = (0, 0);
    let mut found_last: bool = false;
    while !found_last{
        pair = &shortest_pairs[pair_index];

        //Handle elements cases - new circuit if neither present, append other to circuit if one present, join circuits if required, leave if in same circuit
        if junction_boxes[pair.0].circuit.is_none() && junction_boxes[pair.1].circuit.is_none(){
            junction_boxes[pair.0].circuit = Some(new_circuit_id);
            junction_boxes[pair.1].circuit = Some(new_circuit_id);
            new_circuit_id += 1;
        }
        else if junction_boxes[pair.0].circuit.is_none(){
            junction_boxes[pair.0].circuit = junction_boxes[pair.1].circuit;
        }
        else if junction_boxes[pair.1].circuit.is_none(){
            junction_boxes[pair.1].circuit = junction_boxes[pair.0].circuit;
        }
        else if junction_boxes[pair.0].circuit != junction_boxes[pair.1].circuit{
            for i in 0..junction_boxes.len(){
                if junction_boxes[i].circuit == junction_boxes[pair.1].circuit{
                    junction_boxes[i].circuit = junction_boxes[pair.0].circuit;
                }
            }
        }
        
        //Check whether all boxes are in same circuit
        let mut check_circuit: Option::<usize> = None;
        let mut same: bool = true;
        let mut jbox_index: usize = 0;
        while same && jbox_index < junction_boxes.len(){
            if junction_boxes[jbox_index].circuit.is_none(){
                same = false;
            }
            else{
                if check_circuit == None{
                    check_circuit = junction_boxes[jbox_index].circuit;
                }
                else if junction_boxes[jbox_index].circuit != check_circuit{
                    same = false;
                }
            }
            jbox_index += 1;
        }
        if same{
            last_pair = (pair.0, pair.1);
            found_last = true;
        }

        pair_index += 1;
    }

    let mult = junction_boxes[last_pair.0].x * junction_boxes[last_pair.1].x;
    println!("Multiplication: {}", mult);
}