use std::fs;

fn main() {
    let input = fs::read_to_string("C:/Users/timot/Documents/Code/aoc2023/day03/input.txt").unwrap();
    let (parts, line_length, line_count) = get_parts(&input);
    let numbers = get_numbers(&input);
    let mut part_sum = 0;
    let mut gear_ratios = 0;

    for (_p, part) in parts.iter().enumerate() {
        let index = part.1;
        let line = part.2;
        let line_min = if line == 0 { line } else { line - 1 };
        let line_max = if line == line_count { line } else { line + 1 };
        let index_min = if index == 0 { index } else { index - 1 };
        let index_max = if index == line_length { index } else { index + 1 };
        let number: Vec<_> = numbers.clone().into_iter().filter(|&n| n.3 >= line_min && n.3 <= line_max && ((n.1 >= index_min && n.1 <= index_max) || (n.2 - 1 >= index_min && n.2 - 1 <= index_max))).collect();
        let mut gear_ratio = 1;
        for (_i, num) in number.iter().enumerate() {
            let part_num = num.0.parse::<usize>().unwrap();
            part_sum += part_num;
            if part.0 == "*" && number.len() > 1 {
                gear_ratio *= part_num;
            }
        }
        if gear_ratio > 1 {
            gear_ratios += gear_ratio;
        }
    }
    println!("Sum of part numbers: {}", part_sum);
    println!("Gear ratios: {}", gear_ratios);
}


fn get_parts(input: &str) -> (Vec<(&str, usize, usize)>, usize, usize) {
    let mut parts = Vec::new();
    let mut start_index = 0;
    let mut end_index = 0;
    let mut line_length = 0;
    let mut line_count = 0;
    
    for line in input.lines() {
        if line_length == 0 {
            line_length = line.len() + 1;
        } else {
            start_index = 0;
            end_index = 0;
        }
        for (j, jot) in line.char_indices() {
            if jot == '.' || jot.is_ascii_digit() {
                if start_index != j {
                    parts.push((&line[start_index..j], start_index, line_count));
                }
                start_index = j + 1;
            }
            end_index = j;
        }

        if start_index <= end_index {
            parts.push((&line[start_index..end_index], start_index, line_count));
        }
        line_count += 1;
    }
    (parts, line_length, line_count)
}

fn get_numbers(input: &str) -> Vec<(&str, usize, usize, usize)> {
    let mut numbers = Vec::new();
    let mut start_index = 0;
    let mut end_index = 0;
    let mut line_count = 0;
    let mut line_length = 0;

    for (_i, line) in input.lines().enumerate() {
        if line_length == 0 {
            line_length = line.len() + 1;
        } else {
            start_index = 0;
            end_index = 0;
        }
        for (j, jot) in line.char_indices() {
            if !jot.is_ascii_digit() {
                if start_index != j {
                    let num = &line[start_index..j];
                    numbers.push((num, start_index, start_index + num.len(), line_count));
                }
                start_index = j + 1;
            }
            end_index = j;
        }

        if start_index <= end_index {
            let num = &line[start_index..end_index+1];
            numbers.push((num, start_index, start_index + num.len(), line_count));
        }

        line_count += 1;
    }
    numbers
}