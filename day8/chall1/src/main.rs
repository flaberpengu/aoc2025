use std::fs;

const FILENAME: &str = "input.txt";
const NUM_PAIRS: usize = 1000;
const NUM_MULT: usize = 3;

#[derive(Debug)]
struct JunctionBox{
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Circuit{
    ids: Vec<usize>,
}

// ===================== HELPER FUNCTIONS =====================

fn insert_pair_into_ordered_vec(elem: (usize, usize, f64), vec: &mut Vec<(usize, usize, f64)>){
    let mut index: usize = 0;
    let mut inserted: bool = false;
    while !inserted{
        if elem.2 <= vec[index].2{
            vec.insert(index, elem);
            vec.pop();
            inserted = true;
        }
        index += 1;
    }
}

fn insert_circuit_into_ordered_vec((circuit_len, circuit_id): (usize, usize), vec: &mut Vec<(usize, usize)>){
    let mut index = 0;
    let mut inserted: bool = false;
    while !inserted{
        if circuit_len >= vec[index].0{
            vec.insert(index, (circuit_len, circuit_id));
            vec.pop();
            inserted = true;
        }
        index += 1;
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
            });
        }
    }

    //Find distance between all pairs of boxes
    //If in top NUM_PAIRS shortest distances, insert into list in order + remove largest distance
    let mut dists: Vec<f64> = vec![];
    let mut shortest_pairs: Vec<(usize, usize, f64)> = vec![(0, 0, f64::MAX); NUM_PAIRS];
    for i in 0..junction_boxes.len(){
        for j in (i+1)..junction_boxes.len(){
            let a: u64 = junction_boxes[i].x.abs_diff(junction_boxes[j].x);
            let b: u64 = junction_boxes[i].y.abs_diff(junction_boxes[j].y);
            let c: u64 = junction_boxes[i].z.abs_diff(junction_boxes[j].z);
            let dist: f64 = ((a.pow(2) + b.pow(2) + c.pow(2)) as f64).sqrt();
            dists.push(dist);
            if dist < shortest_pairs[NUM_PAIRS-1].2{
                insert_pair_into_ordered_vec((i, j, dist), &mut shortest_pairs);
            }
        }
    }

    //Create list of circuits after we have allconnections we care about
    let mut circuits: Vec<Circuit> = vec![];
    let mut pair_0_circuit: usize;
    let mut pair_1_circuit: usize;
    for pair in shortest_pairs.iter(){
        pair_0_circuit = &circuits.len()+1;
        pair_1_circuit = &circuits.len()+1;
        let mut circuit_index: usize = 0;

        //Find which circuits both elements of the pair are in
        for circuit in &circuits{
            if circuit.ids.contains(&pair.0){
                pair_0_circuit = circuit_index;
            }
            if circuit.ids.contains(&pair.1){
                pair_1_circuit = circuit_index;
            }
            circuit_index += 1;
        }

        //Handle elements cases - new circuit if neither present, append other to circuit if one present, join circuits if required, leave if in same circuit
        if pair_0_circuit >= circuits.len() && pair_1_circuit >= circuits.len(){
            circuits.push(Circuit { ids: vec![pair.0, pair.1] });
        }
        else if pair_0_circuit >= circuits.len(){
            circuits[pair_1_circuit].ids.push(pair.0);
        }
        else if pair_1_circuit >= circuits.len(){
            circuits[pair_0_circuit].ids.push(pair.1);
        }
        else if pair_0_circuit != pair_1_circuit{
            let mut ids_to_move = circuits[pair_1_circuit].ids.clone();
            circuits[pair_0_circuit].ids.append(&mut ids_to_move);
            circuits.remove(pair_1_circuit);
        }
    }

    //Get NUM_MULT largest circuits
    //vec of (size of circuit, index in circuits)
    let mut largest_circuits: Vec<(usize, usize)> = vec![(0, 0); NUM_MULT];
    let mut circuit_index: usize = 0;
    for circuit in &circuits{
        if circuit.ids.len() > largest_circuits[NUM_MULT-1].0{
            insert_circuit_into_ordered_vec((circuit.ids.len(), circuit_index), &mut largest_circuits);
        }
        circuit_index += 1;
    }

    //Calculate multiplication
    let mut mult_total: u64 = 1;
    for circuit in largest_circuits{
        mult_total *= circuit.0 as u64;
    }
    println!("Multiplication total: {}", mult_total);
}