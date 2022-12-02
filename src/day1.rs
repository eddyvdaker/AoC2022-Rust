use std::fs::File;
use std::io::{BufReader, BufRead};

fn most_calories(calorie_lists: &Vec<Vec<i32>>) -> i32 {
    let mut max_calories = 0;
    for sub_list in calorie_lists {
        let mut sub_list_sum = 0;
        for item in sub_list {
            sub_list_sum += item;
        }
        if sub_list_sum > max_calories {
            max_calories = sub_list_sum;
        }
    }
    max_calories
}

fn read_file(file_path: &str) -> Vec<Vec<i32>> {
    let mut lists: Vec<Vec<i32>> = Vec::new();

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut placeholder_vec: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line_content = line.unwrap();
        match &line_content as &str {
            "" => {
                lists.push(placeholder_vec.clone());
                placeholder_vec = Vec::new();
            },
            _ => {
                let value_i32 = line_content.parse::<i32>().unwrap();
                placeholder_vec.push(value_i32);
            }
        }
    }
    lists
}

pub fn run_day1() {
    let lists = read_file("./inputs/day1-input.txt");
    let result = most_calories(&lists);
    println!("most amount of calories {}", result);
}