use std::env;
//use std::fs;
use crate::cli::parse_args;
use crate::processing::image::process_image;

pub mod cli;
pub mod processing;

fn main() {
    let args: Vec<String> = env::args().collect();

    parse_args(&args);
    process_image();
//    let file_path = parse_args;
//    let contents = fs::read_to_string(input_file).expect("File could not be read");
//    println!("With text:\n{contents}");
}
