use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use multimap::MultiMap;

fn main() {
    let (rules, mut updates) = load("day5/source.txt");

    let part1 = updates
        .clone()
        .into_iter()
        .filter(|update| apply_rules(&rules, update))
        .map(|v| middle(&v))
        .reduce(|a, b| a + b);
    println!("part1: {:?}", part1.unwrap());

    ////////

    let part2 = updates
        .iter_mut()
        .filter(|update| !apply_rules(&rules, update)) // could merge the check and the fix
        .map(|v| fix(&rules, v))
        .map(|v| middle(&v))
        .reduce(|a, b| a + b);
    println!("part2: {:?}", part2.unwrap());
}

fn middle(v: &Vec<String>) -> i32 {
    v[v.len() >> 1].parse::<i32>().expect("This is a number")
}

fn apply_rules(rules: &MultiMap<String, String>, update: &Vec<String>) -> bool {
    for idx in 1..update.len() {
        for rdx in 0..idx {
            if let Some(v) = rules.get_vec(&update[idx]) {
                if v.contains(&update[rdx]) {
                    return false;
                }
            }
        }
    }
    true
}

fn fix(rules: &MultiMap<String, String>, update: &mut Vec<String>) -> Vec<String> {
    for idx in 1..update.len() {
        for rdx in 0..idx {
            if let Some(v) = rules.get_vec(&update[idx]) {
                if v.contains(&update[rdx]) {
                    update.swap(rdx, idx);
                }
            }
        }
    }
    update.clone()
}

fn load(path: &str) -> (MultiMap<String, String>, Vec<Vec<String>>) {
    let lines = BufReader::new(File::open(path).expect("Cannot open the file")).lines();

    let mut rules = MultiMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();

    for l in lines {
        if let Ok(line) = l {
            if line == "" {
                continue;
            }

            if let Some((a, b)) = line.split_once('|') {
                rules.insert(a.to_string(), b.to_string());
            } else {
                updates.push(line.split(',').map(|v| v.to_string()).collect());
            }
        }
    }
    (rules, updates)
}

#[cfg(test)]
mod test {
    use assert2::check;

    use super::*;

    #[test]
    fn part1_filter_works() {
        let (rules, updates) = load("sample.txt");
        let part1: Vec<Vec<String>> = updates
            .into_iter()
            .filter(|update| apply_rules(&rules, update))
            .collect();
        check!(part1 == vec![vec!["75", "47", "61", "53", "29"], vec!["97", "61", "53", "29", "13"], vec!["75", "29", "13"]]);
    }

    #[test]
    fn part2_filter_works() {
        let (rules, updates) = load("sample.txt");
        let part2: Vec<Vec<String>> = updates
            .into_iter()
            .filter(|update| !apply_rules(&rules, update))
            .collect();
        check!(part2 == vec![vec!["75", "97", "47", "61", "53"], vec!["61", "13", "29"], vec!["97", "13", "75", "29", "47"]]);
    }
}
