use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_file(path: &str) -> String {
    let mut file = File::open(&Path::new(path)).expect("file not found");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("something went wrong reading the file");
    return s
}

#[derive(Clone)]
struct ElfRation {
    id: u32,
    ration: Vec<u32>
}

impl ElfRation {
    fn new(id: u32) -> ElfRation {
        ElfRation {
            id: id,
            ration: Vec::new()
        }
    }

    fn add_ration(&mut self, ration: u32) {
        self.ration.push(ration);
    }

    fn total_rations(&self) -> u32 {
        let mut total = 0;
        for ration in &self.ration {
            total += ration;
        }
        return total;
    }
}

fn read_data(contents: String) -> Vec<ElfRation> {
    let mut elf_rations: Vec<ElfRation> = Vec::new();
    let mut id = 0;
    for line in contents.lines() {
        if line.is_empty() {
            id += 1;
            continue;
        } 
        if elf_rations.len() <= id {
            elf_rations.push(ElfRation::new(id as u32));
        }

        let ration: u32 = line.parse().unwrap();
        elf_rations[id].add_ration(ration);
        
    }
    return elf_rations;
}

fn find_elf_with_most_rations(elf_rations: &Vec<ElfRation>) -> u32 {
    let mut max_rations = 0;
    let mut max_rations_elf_id = 0;
    for elf_ration in elf_rations {
        let total_rations = elf_ration.total_rations();
        if total_rations > max_rations {
            max_rations = total_rations;
            max_rations_elf_id = elf_ration.id;
        }
    }
    return max_rations_elf_id;
}

fn top_three_elf_rations(elf_rations: &Vec<ElfRation>) -> Vec<ElfRation> {
    let mut top_three: Vec<ElfRation> = Vec::new();
    for elf_ration in elf_rations {
        if top_three.len() < 3 {
            top_three.push(elf_ration.clone());
            continue;
        }

        let mut min_ration = top_three[0].total_rations();
        let mut min_ration_elf_id: usize = 0;

        if top_three[1].total_rations() < min_ration {
            min_ration = top_three[1].total_rations();
            min_ration_elf_id = 1;
        }
        if top_three[2].total_rations() < min_ration {
            min_ration = top_three[2].total_rations();
            min_ration_elf_id = 2;
        }

        if elf_ration.total_rations() > min_ration {
            top_three[min_ration_elf_id] = elf_ration.clone();
        }
    }
    return top_three;
}

fn main() {
    let file_name = "misc/Elf_Rations.txt";
    let contents = read_file(file_name);
    let elf_rations = read_data(contents);
    for elf_ration in &elf_rations {
        println!("Elf {} has {} rations", elf_ration.id, elf_ration.total_rations());
    }
    println!("");
    let elf_with_most_rations = find_elf_with_most_rations(&elf_rations);
    println!("Elf {} has the most rations: {}", elf_with_most_rations, elf_rations[elf_with_most_rations as usize].total_rations());

    let top_three = top_three_elf_rations(&elf_rations);
    let mut top_three_rations: u32 = 0;
    for elf in &top_three {
        top_three_rations += elf.total_rations();
    }
    println!("Top three elves have {} rations", top_three_rations);
}
