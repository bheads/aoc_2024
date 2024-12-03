const SOURCE: &str = "day2/source.csv";

fn main() {
    println!("Part 1: {}", part1(SOURCE));
    println!("Part 2: {}", part2(SOURCE));
}

fn part1(path: &str) -> i64 {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .flexible(true)
        .from_path(path)
        .expect("Can read")
        .deserialize::<Vec<i64>>()
        .map(|row| p1_proc(&row.expect("Cannot access")))
        .reduce(|a, b| a + b)
        .expect("We got a value")
}

fn p1_proc(report: &Vec<i64>) -> i64 {
    // determine the direction of change from the first to elemenets, we will detect the zero case in the match
    let decreasing = report[0] > report[1];
    for idx in 1..report.len() {
        match report[idx - 1] - report[idx] {
            1..=3 if decreasing => continue,
            -3..=-1 if !decreasing => continue,
            _ => {
                //println!("x {:?}", report);
                return 0;
            }
        }
    }
    //println!("✓ {:?}", report);
    1
}

fn part2(path: &str) -> i64 {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .flexible(true)
        .from_path(path)
        .expect("Can read")
        .deserialize::<Vec<i64>>()
        .map(|row| p2_proc(row.expect("Cannot access")))
        .reduce(|a, b| a + b)
        .expect("We got a value")
}

fn p2_proc(report: Vec<i64>) -> i64 {
    if p1_proc(&report) == 1 {
        return 1;
    }
    for idx in 0..report.len() {
        let v = if idx == 0 {
            report[1..].to_vec()
        } else if idx == report.len() - 1 {
            report[0..report.len() - 1].to_vec()
        } else {
            [&report[0..idx], &report[idx + 1..]].concat()
        };
        if p1_proc(&v) == 1 {
            return 1;
        }
    }

    // println!("x {:?}", report);
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use assert2::check;

    #[test]
    fn part1_works() {
        check!(part1("sample.csv") == 2);
    }

    #[test]
    fn part2_works() {
        check!(part2("sample.csv") == 4);
    }

    #[test]
    fn p1_proc_works() {
        check!(p1_proc(&vec![7, 6, 4, 2, 1]) == 1);
        check!(p1_proc(&vec![7, 7, 4, 2, 1]) == 0);
        check!(p1_proc(&vec![7, 6, 4, 2, -4]) == 0);
        check!(p1_proc(&vec![1, 2, 7, 8, 9]) == 0);
        check!(p1_proc(&vec![1, 3, 6, 7, 9]) == 1);
    }

    #[test]
    fn p2_proc_works() {
        check!(p2_proc(vec![7, 6, 4, 2, 1]) == 1);
        check!(p2_proc(vec![7, 7, 4, 2, 1]) == 1);
        check!(p2_proc(vec![7, 6, 4, 2, -4]) == 1);
        check!(p2_proc(vec![1, 2, 7, 8, 9]) == 0);
        check!(p2_proc(vec![1, 3, 6, 7, 9]) == 1);
        check!(p2_proc(vec![1, 3, 2, 4, 5]) == 1);
        check!(p2_proc(vec![90, 89, 91, 93, 95, 94]) == 0);
        check!(p2_proc(vec![8, 7, 8, 10, 13, 15, 17]) == 1);
    }
}
