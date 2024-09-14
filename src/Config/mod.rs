pub mod structs;
use crate::Config::structs::Config;
use std::fs;
use std::io::Write;

pub fn run (config: Config) {
    let contents = fs::read_to_string(config.in_file_path).expect("Couldn't read the file");
    let mut output_file = fs::File::create(config.out_file_path).expect("Output file creation failed");

    for line in search(&config.query, &contents) {
        output_file.write(line.as_bytes()).expect("write failed");
    }

}

pub fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
            results.push("\n");
        }
    }

    results
}