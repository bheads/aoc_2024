use std::iter::zip;

#[cfg(test)] // only used is tests, tests are run in the project folder
const SAMPLE: &str = "sample.csv";
const PUZZLE: &str = "day1/source.csv";

fn main() {
    println!("Part 1: {}", part_1(PUZZLE).expect("That it works"));
    println!("Part 2: {}", part_2(PUZZLE).expect("That it works"));
}
type Record = (i64, i64);

fn load_data(filename: &str) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
    Ok(csv::ReaderBuilder::new().has_headers(false).from_path(filename)?.deserialize::<Record>().map(|row| row.expect("The row is valid")).unzip())
}

fn part_1(filename: &str) -> anyhow::Result<i64> {
    let (mut left, mut right) = load_data(filename)?;

    left.sort();
    right.sort();

    let result = zip(left.iter(), right.iter()).map(|(a, b)| (a - b).abs()).reduce(|a, b| a + b).unwrap();

    Ok(result)
}

fn part_2(filename: &str) -> anyhow::Result<i64> {
    let (left, right) = load_data(filename)?;

    let total = left.iter().fold(0, |acc, &a| acc + (a * right.iter().filter(|v| a == **v).count() as i64));

    Ok(total)
}

//////////// TESTS ////////////////////

#[cfg(test)]
mod test {
    use assert2::check;

    use super::*;

    #[test]
    fn part_1_check() {
        let result = part_1(SAMPLE).expect("That it worked");
        check!(result == 11);
    }

    #[test]
    fn part_2_check() {
        let result = part_2(SAMPLE).expect("That it worked");
        check!(result == 31);
    }

    #[test]
    fn load_data_check() {
        let (a, b) = load_data(SAMPLE).expect("That it worked");
        check!(a == vec![3, 4, 2, 1, 3, 3]);
        check!(b == vec![4, 3, 5, 3, 9, 3]);
    }
}
