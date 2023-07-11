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

fn get_score(values: &Vec<&str>) -> i32 {
    let mut score = 0;
    let my_action = values[1];
    let opponent_action = values[0];

    match my_action {
        "X" => {
            score += 1;
            match opponent_action {
                "A" => score += 3,
                "B" => score += 0,
                "C" => score += 6,
                _ => (),
            }
        }
        "Y" => {
            score += 2;
            match opponent_action {
                "A" => score += 6,
                "B" => score += 3,
                "C" => score += 0,
                _ => (),
            }
        }
        "Z" => {
            score += 3;
            match opponent_action {
                "A" => score += 0,
                "B" => score += 6,
                "C" => score += 3,
                _ => (),
            }
        }
        _ => (),
    }

    return score;
}

fn get_modified_score(values: &Vec<&str>) -> i32 {
    let mut score = 0;
    let my_action = values[1];
    let opponent_action = values[0];

    match my_action {
        "X" => {
            score += 0;
            match opponent_action {
                "A" => score += 3,
                "B" => score += 1,
                "C" => score += 2,
                _ => (),
            }
        }
        "Y" => {
            score += 3;
            match opponent_action {
                "A" => score += 1,
                "B" => score += 2,
                "C" => score += 3,
                _ => (),
            }
        }
        "Z" => {
            score += 6;
            match opponent_action {
                "A" => score += 2,
                "B" => score += 3,
                "C" => score += 1,
                _ => (),
            }
        }
        _ => (),
    }

    return score;
}

fn calculate_results(contents: String) -> (i32, i32) {
    let mut score: i32 = 0;
    let mut actual_score: i32 = 0;
    for line in contents.lines() {
        let split = line.split_whitespace();
        let vec: Vec<&str> = split.collect();
        score += get_score(&vec);
        actual_score += get_modified_score(&vec);
    }
    return (score, actual_score);
}

fn main() {
    let file_name = "data/data.txt";
    let contents = read_file(file_name);
    let (score, actual_score)= calculate_results(contents);
    println!("expected: {}, actual: {}", score, actual_score);
}
