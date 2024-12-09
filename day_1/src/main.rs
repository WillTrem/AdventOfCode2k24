use std::{
    fs::File, io::{prelude::*, BufReader}
};



fn main() {
    part_1();
    part_2();
}

fn part_1(){
    let lines = read_input_to_lines("input.txt");
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for line in lines{
        let mut iter = line.split_whitespace();
        let value = iter.next().expect("Could not parse the first value of the line");
        first_list.push(value.trim().parse().unwrap());    
        let value = iter.next().expect("Could not parse the first value of the line");
        second_list.push(value.trim().parse().unwrap());    
    }

    first_list.sort();   
    second_list.sort();   

    let mut sum = 0;

    for i in 0..first_list.len() {
        sum += (first_list[i] - second_list[i]).abs()
    }
    println!("The sum of the differences is {sum}")
    
}

fn read_input_to_lines(file_name: &str) -> Vec<String>{
    let file = File::open(file_name).expect("Not able to read input file.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse the line"))
        .collect()
}

fn part_2(){
    let lines = read_input_to_lines("input.txt");
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for line in lines{
        let mut iter = line.split_whitespace();
        let value = iter.next().expect("Could not parse the first value of the line");
        first_list.push(value.trim().parse().unwrap());    
        let value = iter.next().expect("Could not parse the first value of the line");
        second_list.push(value.trim().parse().unwrap());    
    }

    let mut sim_score = 0;

    for num_1 in first_list{
        let count = second_list.iter().filter(|&n| *n == num_1).count();
        sim_score += num_1 * count as i32;
    }

    println!("The similarity score is {sim_score}")
}