#![forbid(unsafe_code)]

// TODO: your code goes here.

use std::collections::HashSet;
use std::{fs, process::exit};
use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    // TODO: your code goes here.
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Not enough arguments");
        exit(1);
    }
    let first_filename = &args[1];
    let first_file_size = fs::metadata(first_filename)
        .expect("Can get meta for the first file")
        .len();
    let second_filename = &args[2];
    let second_file_size = fs::metadata(second_filename)
        .expect("Can get meta for the second file")
        .len();
    let (small_filename, big_filename) = match first_file_size > second_file_size {
        true => (second_filename, first_filename),
        false => (first_filename, second_filename),
    };

    // println!("Opening the small file");
    let mut smallfile_lines_set: HashSet<String> = HashSet::new();
    let mut lines_intersect: HashSet<String> = HashSet::new();
    let small_file = File::open(small_filename).unwrap();
    let small_reader = BufReader::new(small_file);
    for line in small_reader.lines() {
        smallfile_lines_set.insert(line.expect("Can not read a line from the first file"));
    }
    // println!("Opening the big file");
    let big_file = File::open(big_filename).unwrap();
    let big_reader = BufReader::new(big_file);
    for line in big_reader.lines() {
        let bigfile_line = &line.expect("Can not read a line from the second file");
        if smallfile_lines_set.contains(bigfile_line) {
            lines_intersect.insert(bigfile_line.clone());
        }
    }
    lines_intersect.iter().for_each(|l| println!("{}", l));
}
