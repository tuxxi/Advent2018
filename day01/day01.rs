use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "day01/day01_input";
    let mut contents = String::new();

    File::open(filename)
        .expect("file not found")
        .read_to_string(&mut contents).unwrap();

    let freqs: Vec<&str> = contents.split("\n").collect();

    part_one(&freqs);
    part_two(&freqs);
}

fn part_one(freqs: &Vec<&str>) {
    let mut total: i32 = 0;
    for f in freqs {

        // get the delta
        let mut delta = 0;
        match f.parse::<i32>() {
            Ok(n) =>  delta = n,
            Err(_e) => eprintln!("could not parse int")
        }

        total += delta;

    }
    println!("total is {}", total);

}

fn part_two(freqs: &Vec<&str>) {
    let mut totals: Vec<i32> = Vec::new();

    let mut total = 0;
    let mut counter = 0;
    'outer: loop {

        println!("iteration: {} totals: {} current total: {}",
                 counter, totals.len(), total);
        counter += 1;

        for freq in freqs {

            // get the delta
            let mut delta = 0;
            match freq.parse::<i32>() {
                Ok(n) =>  delta = n,
                Err(_e) => eprintln!("could not parse int")
            }
            total += delta;
            if totals.contains(&total) {
                break 'outer
            }
            if counter <= 1 {
                totals.push(total);
            }
        }
    }
    println!("Duplicate frequency: {} at count: {}", total, counter);
}