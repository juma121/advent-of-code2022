use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str;

fn main() {

    let input_file = std::fs::File::open("./input-3").unwrap();

    let reader = BufReader::new(input_file);

    let all_lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let score_map = get_score_map();
    let mut total_sum = 0;


    let elf_groups = all_lines.chunks(3);

    for group in elf_groups {
        let duplicate_in_group = find_duplicate_in_elf_groups(&group[0],&group[1],&group[2]);
        let value_of_dublicate = score_map.get(&duplicate_in_group).unwrap();
        total_sum += value_of_dublicate;        
    }

    println!("total_sum = {}", total_sum);
}

fn find_duplicate(compartment_1: &str, compartment_2: &str) -> char {

    for c in compartment_2.chars() {
        if compartment_1.contains(c) {
            return c;
        }
    }
    panic!("no duplicate found");
}


fn find_duplicate_in_elf_groups(elf_1: &str, elf_2: &str, elf_3: &str) -> char {

    let hs_elf_1: HashSet<char> = elf_1.chars().collect();
    let hs_elf_2: HashSet<char> = elf_2.chars().collect();

    for c in elf_3.chars() {
        if hs_elf_1.contains(&c) && hs_elf_2.contains(&c) {
            return c;
        }
    }

    panic!("no duplicate found");
}

fn get_score_map() -> HashMap<char, i32> {
    let mut score_map: HashMap<char, i32> = HashMap::new();

    let all_letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let mut letter_count = 1;
    for letter in all_letters.chars() {
        score_map.insert(letter, letter_count);
        letter_count +=1;
    }

    score_map
}
