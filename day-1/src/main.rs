use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {

    let input_file = std::fs::File::open("./input-1.txt").unwrap();

    let reader = BufReader::new(input_file);

    let all_fields = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();


    let mut elf_wrapper: Vec<Vec<i32>> = Vec::new();
    let mut group = Vec::new();

    for field in all_fields {
        if let Ok(cal) = field.parse::<i32>() {
            group.push(cal);
        } else {
            elf_wrapper.push(group.clone());
            group.clear();
        }
    }

    let mut sums: Vec<i32> = elf_wrapper.into_iter().map(|v| v.iter().sum()).collect();
    sums.sort();
    sums.reverse();

    // dbg!(sums);

    let total: i32 = sums[..3].into_iter().sum();
    dbg!(total);

    // let max = sums.iter().max();
    // dbg!(max);

    

 
    // let lines = input_file.lines();

}
