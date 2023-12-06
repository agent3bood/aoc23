use std::fs::File;
use std::io::{BufReader};

pub fn read_file(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader
}
