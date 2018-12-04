use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashSet};

const ALPHABET: &str = "abcedfghijklmnopqrstuvwxyz";

fn main() {
    let filename = "day02/day02_input";
    let mut contents = String::new();

    File::open(filename)
        .expect("file not found")
        .read_to_string(&mut contents).unwrap();

    let ids: Vec<&str> = contents.split("\n").collect();

    let checksum = part_one(&ids);
    println!("checksum: {}", checksum);
    let common = part_two(&ids);
    println!("common: {}", common);
}

/// calculates a 'checksum' of the input strings
fn part_one(ids: &Vec<&str>) -> i32 {
    let mut num_twos = 0;
    let mut num_threes = 0;
    for &id in ids {
        let mut counts: HashSet<u32> = HashSet::new();

        for letter in ALPHABET.chars() {
            let num = id.matches(letter).count() as u32;
            counts.insert(num);
        }
        //only count one of each 2 or 3 letter recurrence
        num_twos += counts.contains(&2) as i32;
        num_threes += counts.contains(&3) as i32;
    }
    return num_twos * num_threes;
}

fn part_two(ids: &Vec<&str>) -> String {

    let mut result:(&str, &str) = ("", "");
    let mut max_commons = 0;
    for &id in ids {
        for &other in ids {
            if *other != *id {
                let mut num_common = 0;
                for (c1, c2) in id.chars().zip(other.chars()) {
                    if c1 == c2 {
                        num_common += 1;
                    }
                    if num_common > max_commons {
                        max_commons = num_common;
                        result = (id, other);
                    }
                }
            }
        }
    }
    println!("Max chars in common: {}", max_commons);
    println!("between IDs: {} and {}", result.0, result.1);

    let mut intersection = String::new();
    for (c1, c2) in result.0.chars().zip(result.1.chars()) {
        if c1 == c2 {
            intersection += &c1.to_string();
        }
    }
    return intersection;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let items = vec!["abcdef",
                     "bababc",
                     "abbcde",
                     "abcccd",
                     "aabcdd",
                     "abcdee",
                     "ababab"];
        assert_eq!(part_one(&items), 12)
    }

    #[test]
    fn test_part_two() {
        let items = vec![
                     "abcde",
                     "fghij",
                     "klmno",
                     "pqrst",
                     "fguij",
                     "axcye"];
        assert_eq!(part_two(&items), "fgij".to_owned());
    }
}