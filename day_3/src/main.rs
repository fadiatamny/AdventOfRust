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

fn get_char_value(c: char) -> u32 {
    if c.is_uppercase() {
        return c as u32 - 65 + 27;
    } else {
        return c as u32 - 96;
    }
}

fn find_duplicate(sack: &str) -> Option<char> {
    for i in 0..sack.len() / 2 {
        for j in 0..sack.len() / 2 {
            if sack.chars().nth(i) == sack.chars().nth(j + sack.len() / 2) {
                return Some(sack.chars().nth(i).unwrap());
            }
        }
    }

    return None;
}

fn find_duplicates(contents: &String) -> u32 {
    let mut score: u32 = 0;

    for line in contents.lines() {
        let sack: String = String::from(line);
        match find_duplicate(&sack) {
            Some(c) => {
                score += get_char_value(c);
            }
            None => {}
        }
    }

    return score;
}

fn batch_by_three(content: &String) -> Vec<Vec<String>> {
    let mut batches: Vec<Vec<String>> = Vec::new();
    let mut batch: Vec<String> = Vec::new();
    let mut count: u32 = 0;

    for line in content.lines() {
        if count == 3 {
            batches.push(batch);
            batch = Vec::new();
            count = 0;
        }
        batch.push(line.to_string());
        count += 1;
    }

    if count != 0 {
        batches.push(batch);
    }

    return batches;
}

fn find_common_letter(batch: &Vec<String>) -> Option<char> {
    for i in 0..=26 {
        let char = (i + 65) as u8 as char;
        if (batch[0].contains(char)) && (batch[1].contains(char)) && (batch[2].contains(char)) {
            return Some(char);
        }
    }
    for i in 0..=26 {
        let char = (i + 97) as u8 as char;
        if (batch[0].contains(char)) && (batch[1].contains(char)) && (batch[2].contains(char)) {
            return Some(char);
        }
    }

    return None;
}

fn find_common_letters(batches: &Vec<Vec<String>>) -> Vec<char> {
    let mut common_letters: Vec<char> = Vec::new();

    for (index, batch) in batches.iter().enumerate() {
        match find_common_letter(batch) {
            Some(c) => {
                common_letters.push(c);
            }
            None => {
                println!("No common, {}", index);
                for line in batch {
                    println!("- {}", line);
                }
            }
        }
    }

    return common_letters;
}

fn find_letters_priority_sum(letters: &Vec<char>) -> u32 {
    let mut sum: u32 = 0;

    for letter in letters {
        sum += get_char_value(*letter);
    }

    return sum;
}

fn main() {
    // let file_name = "data/dummy_data.txt";
    let file_name = "data/data.txt";
    let data = read_file(file_name);
    let score = find_duplicates(&data);
    let batched = batch_by_three(&data);
    let common = find_common_letters(&batched);
    let sum = find_letters_priority_sum(&common);
    println!("Part 1!, {}", score);
    println!("Part 2!, {:?}", sum);
}
