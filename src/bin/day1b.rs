pub mod utils;

use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let mut reader = utils::get_file_reader_from_args().unwrap();

    let mut line = String::new();

    let mut first_map = HashMap::<u32, u32>::new();
    let mut second_map = HashMap::<u32, u32>::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        let nums: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(nums.len(), 2);

        let first: u32 = nums.first().unwrap().parse().unwrap();
        let last: u32 = nums.last().unwrap().parse().unwrap();

        first_map
            .entry(first)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        second_map
            .entry(last)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        line.clear();
    }

    let mut res: u32 = 0;

    for (num, count) in &first_map {
        println!("{}\t{}", num, count);
    }

    for (num, _) in &first_map {
        match second_map.get(&num) {
            Some(second_count) => res += num * second_count,
            None => {}
        }
    }

    println!("result: {}", res);
}
