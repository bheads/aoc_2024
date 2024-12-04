use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    //let sample = load_data("day4/sample.txt");
    let source = load_data("day4/source.txt");

    println!("PART 1: {:?}", part1_word_search(&source));
    println!("PART 2: {:?}", part2_word_search(&source));
}

fn load_data(filename: &str) -> Vec<Vec<u8>> {
    BufReader::new(File::open(filename).expect("Cannot open the file"))
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect()
}

fn part1_word_search(data: &Vec<Vec<u8>>) -> i32 {
    let mut cnt = 0;
    let height = data.len();
    let width = data[0].len();

    for y in 0..height {
        for x in 0..width {
            cnt += if y >= 3 && data[y][x] == b'X' && data[y - 1][x] == b'M' && data[y - 2][x] == b'A' && data[y - 3][x] == b'S' { 1 } else { 0 }; // North
            cnt += if y >= 3 && x + 3 < width && data[y][x] == b'X' && data[y - 1][x + 1] == b'M' && data[y - 2][x + 2] == b'A' && data[y - 3][x + 3] == b'S' { 1 } else { 0 }; // North East

            cnt += if x + 3 < width && data[y][x] == b'X' && data[y][x + 1] == b'M' && data[y][x + 2] == b'A' && data[y][x + 3] == b'S' { 1 } else { 0 }; // East
            cnt += if y + 3 < height && x + 3 < width && data[y][x] == b'X' && data[y + 1][x + 1] == b'M' && data[y + 2][x + 2] == b'A' && data[y + 3][x + 3] == b'S' { 1 } else { 0 }; // South East

            cnt += if y + 3 < height && data[y][x] == b'X' && data[y + 1][x] == b'M' && data[y + 2][x] == b'A' && data[y + 3][x] == b'S' { 1 } else { 0 }; // South
            cnt += if y + 3 < height && x >= 3 && data[y][x] == b'X' && data[y + 1][x - 1] == b'M' && data[y + 2][x - 2] == b'A' && data[y + 3][x - 3] == b'S' { 1 } else { 0 }; // South West

            cnt += if x >= 3 && data[y][x] == b'X' && data[y][x - 1] == b'M' && data[y][x - 2] == b'A' && data[y][x - 3] == b'S' { 1 } else { 0 }; // West
            cnt += if y >= 3 && x >= 3 && data[y][x] == b'X' && data[y - 1][x - 1] == b'M' && data[y - 2][x - 2] == b'A' && data[y - 3][x - 3] == b'S' { 1 } else { 0 };
            // North West
        }
    }

    cnt
}

fn part2_word_search(data: &Vec<Vec<u8>>) -> i32 {
    let mut cnt = 0;
    let height = data.len();
    let width = data[0].len();

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            // middle must be an A
            if data[y][x] == b'A' {
                // NW to SE -> MAS / SAM
                if (data[y - 1][x - 1] == b'M' && data[y + 1][x + 1] == b'S') || (data[y - 1][x - 1] == b'S' && data[y + 1][x + 1] == b'M') {
                    // NE to SW -> MAS / SAM
                    if (data[y + 1][x - 1] == b'M' && data[y - 1][x + 1] == b'S') || (data[y + 1][x - 1] == b'S' && data[y - 1][x + 1] == b'M') {
                        cnt += 1;
                    }
                }
            }
        }
    }

    cnt
}

#[cfg(test)]
mod test {
    use assert2::check;

    use super::*;

    #[test]
    fn load_works() {
        let sample = load_data("sample.txt"); // tests run in the day4 path, not the root path :shurg
        check!(sample.len() == 10);
        check!(String::from_utf8(sample[0].clone()).unwrap() == "MMMSXXMASM");
    }

    #[test]
    fn p1_test() {
        let sample = load_data("sample.txt");
        check!(part1_word_search(&sample) == 18);
    }

    #[test]
    fn p2_test() {
        let sample = load_data("sample.txt");
        check!(part2_word_search(&sample) == 9);
    }
}
