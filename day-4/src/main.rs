use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str;

fn main() {

    

    let input_file = std::fs::File::open("./input-4").unwrap();

    let reader = BufReader::new(input_file);

    let all_lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mut total = 0;

    for line in all_lines {
        let elfs: Vec<&str> = line.split(',').collect();
        let left_elf_min_max: Vec<&str> = elfs[0].split('-').collect();
        let right_elf_min_max: Vec<&str> = elfs[1].split('-').collect();
        let left_elf_min = left_elf_min_max[0].parse::<i32>().unwrap();
        let left_elf_max = left_elf_min_max[1].parse::<i32>().unwrap();
        let right_elf_min = right_elf_min_max[0].parse::<i32>().unwrap();
        let right_elf_max = right_elf_min_max[1].parse::<i32>().unwrap();

        if left_elf_min <= right_elf_min && left_elf_max >= right_elf_min {
            total+=1;
            continue;
        }

        if left_elf_min >= right_elf_min && left_elf_min <= right_elf_max {
            total+=1;
            continue;
        }
    }

    println!("total: {}", total);

}