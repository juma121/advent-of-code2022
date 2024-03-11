use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str;

fn main() {
    

    

    //X Loose,
    //Y Draw,
    //Z Win,

    //Lose => 0
    //Draw => 3
    //Win => 6

    

    let input_file = std::fs::File::open("./input-2").unwrap();

    let reader = BufReader::new(input_file);

    let all_lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mut total_score = 0;

    for line in all_lines {
        line.trim();
        line.split(' ');
        let v: Vec<&str> = line.split(' ').collect();
        let score_for_outcome = score_for_outcome(v[0], v[1]);
        // let score_for_choice = score_for_choice(v[1]);

        total_score += score_for_outcome;
        // total_score += score_for_choice;
    }
    //
    println!("total_score: {}", total_score);
}




//A Rock
//B Paper
//C Scissors

//X Loose,
//Y Draw,
//Z Win,

//X Rock => 1
//Y Paper => 2
//Z Scissors => 3


fn score_for_outcome(oppenent_choice: &str, your_choice: &str) -> i32 {
    match (oppenent_choice, your_choice) {
        ("A", "X") => 0 + 3,
        ("A", "Y") => 3 + 1,
        ("A", "Z") => 6 + 2,
        ("B", "X") => 0 + 1,
        ("B", "Y") => 3 + 2,
        ("B", "Z") => 6 + 3,
        ("C", "X") => 0 + 2,
        ("C", "Y") => 3 + 3,
        ("C", "Z") => 6 + 1,
        _ => panic!("unexpected input")
    }
}

fn score_for_choice(choice: &str) -> i32 {
    match choice {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("unnown char found")
    }
}