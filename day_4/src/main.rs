use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_file(path: &str) -> String {
    let mut file = File::open(&Path::new(path)).expect("file not found");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("something went wrong reading the file");
    return s;
}

fn gen_pairs(data: String) -> Vec<[(u16, u16); 2]> {
    let mut pairs: Vec<[(u16, u16); 2]> = Vec::new();
    for line in data.lines() {
        let mut pair_array: [(u16, u16); 2] = [(0, 0), (0, 0)];
        let mut i: usize = 0;
        for pair in line.split(',') {
            let mut values: (u16, u16) = (0, 0);
            let mut j = 0;
            for num in pair.split('-') {
                if j == 0 {
                    values.0 = num.parse::<u16>().unwrap();
                } else {
                    values.1 = num.parse::<u16>().unwrap();
                }
                j += 1;
            }
            pair_array[i] = values;
            i += 1;
        }
        pairs.push(pair_array);
    }
    return pairs;
}

fn is_contained_range(pair1: (u16, u16), pair2: (u16, u16)) -> bool {
    let contained_in_pair2 = pair1.0 >= pair2.0 && pair1.1 <= pair2.1;
    let contained_in_pair1 = pair2.0 >= pair1.0 && pair2.1 <= pair1.1;

    return contained_in_pair2 || contained_in_pair1;
}
fn is_overlapping_range(pair1: (u16, u16), pair2: (u16, u16)) -> bool {
    let overlap1 = pair1.0 <= pair2.1 && pair2.0 <= pair1.1;
    let overlap2 = pair2.0 <= pair1.1 && pair1.0 <= pair2.1;

    return overlap1 || overlap2
}

fn main() {
    // let file_name = "data/dummy_data.txt";
    let file_name = "data/data.txt";
    let data: String = read_file(file_name);
    let pairs = gen_pairs(data);
    let mut contained: u16 = 0;
    let mut overlapping: u16 = 0;
    for pair in pairs.iter() {
        if is_contained_range(pair[0], pair[1]) {
            contained += 1;
        }
        if is_overlapping_range(pair[0], pair[1]) {
            overlapping += 1;
        }
    }
    println!("Contained: {}", contained);
    println!("Overlapping: {}", overlapping);
}
