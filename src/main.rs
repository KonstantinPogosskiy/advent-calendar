use std::collections::HashMap;
use std::fs;
use std::io;

fn part_1_day_1() -> Result<(), Box<dyn std::error::Error>> {
    let list = fs::read_to_string("src/input/day_1.txt")?;
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
    left_side.sort();
    right_side.sort();

    let result: i32 = left_side.iter().zip(right_side.iter()).map(|(left, right)| (right - left).abs()).sum();
    println!("{}", result);
    Ok(())
}


fn part_2_day_1() -> Result<(), Box<dyn std::error::Error>> {
    let list = fs::read_to_string("src/input/day_1.txt")?;

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
    for num in &right_side {
        *right_counts.entry(num).or_insert(0) += 1;
    }
    let total: i32 = left_side.iter()
        .map(|num| {
            right_counts.get(num).unwrap_or(&0) * num
        })
        .sum();

    println!("{}", total);
    Ok(())
}

fn part_1_day_2() -> Result<(), Box<dyn std::error::Error>> {
    let list = fs::read_to_string("src/input/day_2.txt")?;
    let array = list.lines()
        .map(|arr| {
            arr.split_whitespace()
                .filter_map(|item| item.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|arr| {
            let steps_array: Vec<i32> = 
            //сначала написал вариант с zip, он рабочий но тумач для этого, так как создает два итератора

            // arr.iter().zip(arr.iter().skip(1))
            // .map(|(a, b)| b - a)

            //вариант с windows создает окно с парами элементов и делает сдвиг на 1
                arr.windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect();
            let direction = steps_array[0].signum();
            steps_array.iter().all(|&step| {
                step.abs() >= 1 && step.abs() <= 3 && step.signum() == direction
            })
        })

        //.len() есть только у коллекциий, можно посчитать длину элементов самого итератора .count() без создания в памяти коллекции
        // .collect::<Vec<_>>().len();
        .count();

    println!("{}", array);
    Ok(())
}

fn part_2_day_2() -> Result<(), Box<dyn std::error::Error>> {
    let list = fs::read_to_string("src/input/day_2.txt")?;
    
    fn problem_dampener(arr: &[i32]) -> bool {
        if arr.len() < 2 { return true; }
        let steps: Vec<i32> = arr.windows(2).map(|pair: &[i32]| pair[1] - pair[0]).collect();
        let dir = steps[0].signum();
        steps.iter().all(|&step| step.abs() >= 1 && step.abs() <= 3 && step.signum() == dir)
    }
    
    let safe_calculation = list.lines()
        .map(|arr| arr.split_whitespace().filter_map(|item| item.parse().ok()).collect::<Vec<i32>>())
        .filter(|arr| {
            if problem_dampener(arr) {
                return true;
            }
            for i in 0..arr.len() {
                let mut temp_arr = arr.clone();
                temp_arr.remove(i);
                if problem_dampener(&temp_arr) {
                    return true;
                }
            }
            false
        })
        // и if-ы и сам for можно семант упростить до:

        // .filter(|arr| problem_dampener(arr) || (0..arr.len()).any(|i| {
        //     let mut temp_arr = arr.clone();
        //     temp_arr.remove(i);
        //     problem_dampener(&temp_arr)
        // }))
        .count();

    println!("{}", safe_calculation);
    Ok(())
}

fn main() -> io::Result<()> {
        part_1_day_1();
        part_2_day_1();
        part_1_day_2();
        part_2_day_2();
        Ok(())
}
