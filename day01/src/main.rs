use ferris_says::say;
use std::io::BufRead;
use std::io::stdout;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::fs::File;

fn main() {
    let stdout: std::io::Stdout = stdout();
    let message: String = String::from(extract_calibration().unwrap());
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

    if true {
        let mut message: String = String::from("Sum of all calibration numbers: ");
        message.push_str(&all_values.iter().sum::<u32>().to_string());
        return Ok(String::from(message));
    } else {
        return Err("Something broke...".into());
    }
}