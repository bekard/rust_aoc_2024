pub mod utils;

use std::io::BufRead;

fn main() {
    let mut reader = utils::get_file_reader_from_args().unwrap();

    let mut line = String::new();

    let mut first_vec = Vec::<u32>::new();
    let mut second_vec = Vec::<u32>::new();

    let mut count = 1u32;

    while reader.read_line(&mut line).unwrap() > 0 {
        let nums: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(nums.len(), 2);

        let first: u32 = nums.first().unwrap().parse().unwrap();
        let last: u32 = nums.last().unwrap().parse().unwrap();

        first_vec.push(first);
        second_vec.push(last);

        line.clear();
    }

    first_vec.sort();
    second_vec.sort();
    assert_eq!(first_vec.len(), second_vec.len());

    let mut res: u32 = 0;

    for i in 0..first_vec.len() {
        res += second_vec[i].abs_diff(first_vec[i]);
    }

    println!("result: {}", res);
}
