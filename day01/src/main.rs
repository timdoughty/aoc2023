use ferris_says::say;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::stdout;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::fs::File;

fn main() {
    let stdout: std::io::Stdout = stdout();
    let message: String = String::from(extract_calibration_words().unwrap());
    let width: usize = message.chars().count();

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn extract_calibration() -> Result<String, Box<dyn std::error::Error>> {
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut all_values: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line: String = line?;
        let mut forward: std::str::Chars<'_> = line.chars();
        let mut backward: std::iter::Rev<std::str::Chars<'_>> = line.chars().rev();
        let mut first: Option<u32> = None;
        let mut second: Option<u32> = None;

        while let (Some(f), Some(b)) = (forward.next(), backward.next()) {
            if forward.size_hint().0 < backward.size_hint().0 {
                break;
            }
            if first.is_none() && f.is_ascii_digit() {
                first = Some(f.to_digit(10).unwrap());
            }
            if second.is_none() && b.is_ascii_digit() {
                second = Some(b.to_digit(10).unwrap());
            }
        }
        let combined = format!("{}{}", first.unwrap(), second.unwrap());
        let combined_value = combined.parse::<u32>().unwrap();
        all_values.push(combined_value);
    }

    if all_values.len() > 0 {
        let mut message: String = String::from("Sum of all calibration numbers: ");
        message.push_str(&all_values.iter().sum::<u32>().to_string());
        return Ok(String::from(message));
    } else {
        return Err("Something broke...".into());
    }
}

fn extract_calibration_words() -> Result<String, Box<dyn std::error::Error>> {
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut all_values: Vec<u32> = Vec::new();
    let mut number_words: HashMap<&str, u32> = HashMap::new();
    number_words.insert("one", 1);
    number_words.insert("two", 2);
    number_words.insert("three", 3);
    number_words.insert("four", 4);
    number_words.insert("five", 5);
    number_words.insert("six", 6);
    number_words.insert("seven", 7);
    number_words.insert("eight", 8);
    number_words.insert("nine", 9);
    let mut count = 0;

    for line in reader.lines() {
        let line: String = line?;
        let mut first: Option<u32> = None;
        let mut second: Option<u32> = None;

        for (j, jot) in line.chars().enumerate() {
            if jot.is_ascii_digit() {
                if first.is_none() {
                    first = Some(jot.to_digit(10).unwrap());
                } else {
                    second = Some(jot.to_digit(10).unwrap());
                }
            }
            for (key, &value) in &number_words {
                if line[j..].starts_with(key) {
                    if first.is_none() {
                        first = Some(value);
                    } else {
                        second = Some(value);
                    }
                }
            }
        }
        if second.is_none() {
            second = first;
        }
        count += 1;
        println!("Line: {}", count);
        println!("\tFirst value: {}", first.unwrap());
        println!("\tLast value: {}", second.unwrap());
        let combined = format!("{}{}", first.unwrap(), second.unwrap());
        let combined_value = combined.parse::<u32>().unwrap();
        println!("\tCalibration Value: {}", combined_value);
        all_values.push(combined_value);
    }

    if all_values.len() > 0 {
        let mut message: String = String::from("Sum of all calibration numbers: ");
        message.push_str(&all_values.iter().sum::<u32>().to_string());
        return Ok(String::from(message));
    } else {
        return Err("Something broke...".into());
    }
}