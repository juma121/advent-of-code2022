use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::{self, Chars};

fn main() {

    let mut stack_1 = vec!['G', 'T', 'R', 'W'];
    let mut stack_2 = vec!['G', 'C', 'H', 'P', 'M', 'S', 'V', 'W'];
    let mut stack_3 = vec!['C', 'L', 'T', 'S', 'G', 'M'];
    let mut stack_4 = vec!['J', 'H', 'D', 'M', 'W', 'R', 'F'];
    let mut stack_5 = vec!['P', 'Q', 'L', 'H', 'S', 'W', 'F', 'J'];
    let mut stack_6 = vec!['P', 'J', 'D', 'N', 'F', 'M', 'S'];
    let mut stack_7= vec!['Z', 'B', 'D', 'F', 'G', 'C', 'S', 'J'];
    let mut stack_8 = vec!['R', 'T', 'B'];
    let mut stack_9 = vec!['H', 'N', 'W', 'L', 'C'];

    let input_file = std::fs::File::open("./input-5").unwrap();

    let reader = BufReader::new(input_file);

    let all_lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    for line in all_lines {
        let formated = line.replace("move ", "").replace(" from ", ",").replace(" to ", ",");
        let separated: Vec<&str> = formated.split(',').collect();
        let cuantitiy = separated[0].parse::<i32>().unwrap();
        let origin = separated[1].parse::<i32>().unwrap();
        let target = separated[2].parse::<i32>().unwrap();

        let mut moved_stack: Vec<char> = Vec::new();
        for time in 0..cuantitiy {
            let letter = match origin {
                1 => stack_1.pop().unwrap(),
                2 => stack_2.pop().unwrap(),
                3 => stack_3.pop().unwrap(),
                4 => stack_4.pop().unwrap(),
                5 => stack_5.pop().unwrap(),
                6 => stack_6.pop().unwrap(),
                7 => stack_7.pop().unwrap(),
                8 => stack_8.pop().unwrap(),
                9 => stack_9.pop().unwrap(),
                _ => panic!("failed1")
            };
            moved_stack.push(letter);
        }

        moved_stack.reverse();
        

        match target {
            1 => { 
                stack_1 = stack_1.into_iter().chain(moved_stack).collect();
            },
            2 => { 
                stack_2 = stack_2.into_iter().chain(moved_stack).collect();
            },
            3 => { 
                stack_3 = stack_3.into_iter().chain(moved_stack).collect();
            },
            4 => {
                stack_4 = stack_4.into_iter().chain(moved_stack).collect();
            },
            5 => { 
                stack_5 = stack_5.into_iter().chain(moved_stack).collect();
            },
            6 => { 
                stack_6 = stack_6.into_iter().chain(moved_stack).collect();
            },
            7 => { 
                stack_7 = stack_7.into_iter().chain(moved_stack).collect();
            },
            8 => { 
                stack_8 = stack_8.into_iter().chain(moved_stack).collect();
            },
            9 => { 
                stack_9 = stack_9.into_iter().chain(moved_stack).collect();
            },
            _ => panic!("failed2")
        }

    }

    let stacks = vec![stack_1, stack_2, stack_3, stack_4, stack_5, stack_6, stack_7, stack_8, stack_9];

    let mut answer = String::new(); 
    
    for stack in stacks {
        answer.push(*stack.last().unwrap());
    } 

    println!("total: {}", answer);

}
