use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "day02/day02_input";
    let mut contents = String::new();

    File::open(filename)
        .expect("file not found")
        .read_to_string(&mut contents).unwrap();

    let ids: Vec<&str> = contents.split("\n").collect();

    day_one(&ids);
}

fn day_one(ids: &Vec<&str>) {
    
}