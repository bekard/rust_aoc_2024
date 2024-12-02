pub mod utils;

use std::io::BufRead;

fn is_report_safe(report: &str) -> bool {
    let diffs: Vec<i32> = report
        .split_whitespace()
        .collect::<Vec<&str>>()
        .windows(2)
        .map(|w| w[1].parse::<i32>().unwrap() - w[0].parse::<i32>().unwrap())
        .collect();

    let is_safe = diffs.iter().all(|&v| v.is_positive() && v.abs() < 4)
        || diffs.iter().all(|&v| v.is_negative() && v.abs() < 4);

    is_safe
}

#[test]
fn test_report_safety() {
    let data = std::collections::HashMap::from([
        ("7 6 4 2 1", true),
        ("1 2 7 8 9", false),
        ("9 7 6 2 1", false),
        ("1 3 2 4 5", false),
        ("8 6 4 4 1", false),
        ("1 3 6 7 9", true),
    ]);

    for (&report, &is_safe) in &data {
        assert_eq!(is_report_safe(report), is_safe);
    }
}

fn main() {
    let mut reader = utils::get_file_reader_from_args().unwrap();

    let mut line = String::new();

    let mut res = 0u32;
    while reader.read_line(&mut line).unwrap() > 0 {
        if is_report_safe(line.as_str()) {
            res += 1;
        }

        line.clear();
    }

    println!("result: {}", res);
}
