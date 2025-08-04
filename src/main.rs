use std::collections::HashMap;
use std::fs;
use std::io;

fn calculate_difference(list: &str) {
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    for line in list.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            left_side.push(left);
            right_side.push(right);
        }
    }
    left_side.sort();
    right_side.sort();

    let result: i32 = left_side.iter().zip(right_side.iter()).map(|(left, right)| (right - left).abs()).sum();
    println!("{}", result);
}


fn calculate_difference_2(list: &str) {
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    for line in list.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                left_side.push(left);
                right_side.push(right);
            }
        }
    }

    let mut right_counts = HashMap::new();
    for num in right_side {
        *right_counts.entry(num).or_insert(0) += 1;
    }
    let total: i32 = left_side.iter()
        .map(|num| {
            right_counts.get(num).unwrap_or(&0) * num}).sum();

    println!("{}", total);
}

fn main() -> io::Result<()> {
        let data = fs::read_to_string("src/coordinate.txt")?;
        calculate_difference(&data);
        calculate_difference_2(&data);
        Ok(())
}
