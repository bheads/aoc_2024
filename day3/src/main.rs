use std::fs;

use atoi::atoi;
use regex::Regex;

const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const SAMPLE2: &str =
    "do()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() {
    let source = fs::read_to_string("day3/source.txt").expect("Cant read the file");

    println!("Part 1: {}", extract(&source));
    println!("Part 2: {}", extract2(&source));
}

fn extract(haystack: &str) -> i64 {
    let re = Regex::new(r"mul\((?<lhs>\d{1,3})\,(?<rhs>\d{1,3})\)").unwrap();
    re.captures_iter(haystack)
        .map(|caps| {
            let lhs = atoi::<i64>(caps.name("lhs").unwrap().as_str().as_bytes()).unwrap();
            let rhs = atoi::<i64>(caps.name("rhs").unwrap().as_str().as_bytes()).unwrap();
            lhs * rhs
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

fn extract2(haystack: &str) -> i64 {
    let re =
        Regex::new(r"mul\((?<lhs>\d{1,3})\,(?<rhs>\d{1,3})\)|(?<off>don\'t\(\))|(?<on>do\(\))")
            .unwrap();
    let mut okay = true;
    re.captures_iter(haystack)
        .map(|caps| {
            if let Some(_) = caps.name("off") {
                okay = false;
            } else if let Some(_) = caps.name("on") {
                okay = true;
            } else if okay {
                let lhs = atoi::<i64>(caps.name("lhs").unwrap().as_str().as_bytes()).unwrap();
                let rhs = atoi::<i64>(caps.name("rhs").unwrap().as_str().as_bytes()).unwrap();
                return lhs * rhs;
            }
            return 0;
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

#[cfg(test)]
mod test {
    use assert2::check;

    use super::*;

    #[test]
    fn extract_sample() {
        let actual = extract(SAMPLE);
        check!(actual == 161);
    }

    #[test]
    fn extract2_sample2() {
        let actual = extract2(SAMPLE2);
        check!(actual == 48);
    }
}
