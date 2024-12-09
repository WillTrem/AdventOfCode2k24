use std::{fs::File, io::{prelude::*, BufReader}};
fn main() {
    // println!("Hello, world!");
    part_1();
    part_2();
}

fn read_input_to_lines(file_name: &str) -> Vec<String>{
    let file = File::open(file_name).expect("Not able to read input file.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse the line"))
        .collect()
}

fn part_1(){
    let lines = read_input_to_lines("input.txt");
    let mut safe_count = 0;

    for line in lines{
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse::<i32>().expect("Could not parse number"))
            .collect();
        let mut decreasing = false;
        for i in 0..nums.len()-1{
            let diff = nums[i] - nums[i+1];

            if diff.abs() > 3 || diff.abs() < 1 {
                break;
            }

            if i == 0{ 
                if diff > 0 { 
                    decreasing = true;
                }
            }else if i == nums.len()-2{
                if (decreasing && diff > 0) || (!decreasing && diff < 0){
                    safe_count += 1;
                    continue;
                }
            }else{
                if (decreasing && diff > 0) || (!decreasing && diff < 0){
                    continue;
                }else{
                    break;
                }
            }
        }
    }

    println!("1) The number of safe reports is {safe_count}.")
}
fn part_2(){
    let lines = read_input_to_lines("input.txt");
    let mut safe_count = 0;

    for line in lines{
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse::<i32>().expect("Could not parse number"))
            .collect();
        let mut safe = is_report_safe(&nums);
        
        if safe {
            safe_count +=1;
        }
        else {
            println!("Nums: {:?}, len:{}", nums, nums.len()-1);
            for i in 0..(nums.len()){
                let mut nums_part = nums.clone();
                nums_part.remove(i);
                println!("Nums_part{:?}, i:{i}", nums_part);
                safe = is_report_safe(&nums_part);
                if safe {
                    safe_count +=1;
                    println!("safe: {safe}");
                    break;
                }
            }
        }
    }

    println!("2) The number of safe reports is {safe_count}.")
}

fn is_report_safe(nums: &Vec<i32>)->bool{
    let mut decreasing = false;
        let mut safe = false;

        for i in 0..nums.len()-1{
            let diff = nums[i] - nums[i+1];

            if diff.abs() > 3 || diff.abs() < 1 {
                break;
            }

            if i == 0{ 
                if diff > 0 { 
                    decreasing = true;
                }
            }else if i == nums.len()-2{
                if (decreasing && diff > 0) || (!decreasing && diff < 0){
                    safe = true;
                    continue;
                }
            }else{
                if (decreasing && diff > 0) || (!decreasing && diff < 0){
                    continue;
                }else{
                    break;
                }
            }
        }
    return safe;
}