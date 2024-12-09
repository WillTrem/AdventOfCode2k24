use regex::Regex;

use std::fs;
fn main() {
    part_1();
    part_2();
}

fn part_1(){
    let mem = fs::read_to_string("input.txt").expect("Could not read file");
    // println!("{mem}");
    let pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let iter: Vec<&str> = pattern.find_iter(&mem).map(|m| m.as_str()).collect();
    let mut sum = 0;
    for command in iter{
        let mut iter2 = command.split(",");
        let first = iter2.next().unwrap();
        let first: i32 = first.rsplit("(").next().unwrap().parse().unwrap();
        let second = iter2.next().unwrap();
        let second: i32 = second.split(")").next().unwrap().parse().unwrap();
        sum += first * second;
    }
    println!("1) The sum of valid operations is {sum}");
}
fn part_2(){
    let mem = fs::read_to_string("input.txt").expect("Could not read file");
    // println!("{mem}");
    let pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let iter: Vec<&str> = pattern.find_iter(&mem).map(|m| m.as_str()).collect();
    let mut sum = 0;
    let mut last_condition = true;
    for command in iter{
        if command == "do()" {
            last_condition = true;
            continue;
        }else if command == "don't()"{
            last_condition = false;
            continue;
        }
        if !last_condition {
            continue;
        }
        let mut iter2 = command.split(",");
        let first = iter2.next().unwrap();
        let first: i32 = first.rsplit("(").next().unwrap().parse().unwrap();
        let second = iter2.next().unwrap();
        let second: i32 = second.split(")").next().unwrap().parse().unwrap();
        sum += first * second;
    }
    println!("2) The sum of valid operations is {sum}");
}
