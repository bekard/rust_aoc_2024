use std::env;
use std::fs::File;
use std::io::{BufReader, Result};

pub fn get_file_reader(file_path: &str) -> Result<BufReader<File>> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    Ok(reader)
}

pub fn get_file_reader_from_args() -> Result<BufReader<File>> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let file_path: &str = &args[1];
    get_file_reader(file_path)
}
