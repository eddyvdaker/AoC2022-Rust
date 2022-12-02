use std::fs::File;
use std::io::{BufReader, BufRead};

fn calories(calorie_lists: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut sums: Vec<i32> = Vec::new();
    for sub_list in calorie_lists {
        let mut sub_list_sum = 0;
        for item in sub_list {
            sub_list_sum += item;
        }
        sums.push(sub_list_sum);
    }
    sums.sort();
    sums
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
    let sums = calories(&lists);
    println!("Top 3 amount of calories:\n1) {}\n2) {}\n3) {}", sums[sums.len() - 1],
        sums[sums.len() - 2], sums[sums.len() - 3]);
    let top_3_total = sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3];
    println!("Top 3 total: {}", top_3_total);
}