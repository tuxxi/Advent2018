// std
use std::fs;
// crates
use regex::Regex;
use lazy_static::*;

#[derive(Debug)]
struct Claim {
    id: u32,
    offset: (u32, u32), // left, top offset in inches
    dims: (u32, u32),   // width, height of rectangle in inches
}

impl Claim {
    /// construct claim from input string
    fn from(string: &str) -> Claim {
        // only compile this once, use static version for everything else
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d)+").unwrap();
        }
        // collect all matches into vec of u32
        let v: Vec<u32> = RE
            .find_iter(string)
            .map(|mat| mat.as_str().parse::<u32>().unwrap())
            .collect();
        // construct claim
        Claim {
            id: v[0],
            offset: (v[1], v[2]),
            dims: (v[3], v[4]),
        }
    }
}

fn read_file(filename: &str) -> Vec<Claim> {
    let raw = fs::read_to_string(filename).expect("unable to read");
    // construct vec of claims from lines
    raw.lines().map(|line| Claim::from(line)).collect()
}

/// computes how many square inches of fabric are within two or more claims
fn part_one(claims: &Vec<Claim>) -> u32 {
    // 1024^2 for now since i dont feel like dynamic 2d vectors
    let mut state = [[0u32; 1024]; 1024];

    for claim in claims {
        let xbegin = claim.offset.0 + 1;
        let ybegin = claim.offset.1 + 1;
        let xend = xbegin + claim.dims.0;
        let yend = ybegin + claim.dims.1;
        for x in xbegin..xend {
            for y in ybegin..yend {
                state[x as usize][y as usize] += 1;
            }
        }
    }
    let mut total = 0;
    for xs in state.iter() {
        for y in xs.iter() {
            if y > &1 {
                total += 1;
            }
        }
    }
    return total;
}

///// computes the ID of the claim that does not overlap with other claims
//fn part_two(claims: &Vec<Claim>) -> u32 {
//
//}

fn main() {
    let data = read_file("day03/day03_input");
    println!("total overlapping: {:?}", part_one(&data));
    //println!("ID that doesn't overlap: {:?}", part_two(&data));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let items = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
            .iter()
            .map(|v| Claim::from(v))
            .collect();
        assert_eq!(part_one(&items), 4)
    }
//    #[test]
//    fn test_part_two() {
//        let items = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
//            .iter()
//            .map(|v| Claim::from(v))
//            .collect();
//        assert_eq!(part_two(&items), 3)
//    }

}
