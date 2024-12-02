use std::{collections::HashSet, fs::read_to_string};

pub fn day01() {
    let file = read_to_string("src/data/day01.tsv").expect("Unable to open file");
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in file.lines() {
        let mut iter = line.split_whitespace();
        vec1.push(
            iter.next()
                .unwrap()
                .parse()
                .expect("Unable to parse number"),
        );
        vec2.push(
            iter.next()
                .unwrap()
                .parse()
                .expect("Unable to parse number"),
        );
    }
    vec1.sort();
    vec2.sort();

    let result: i32 = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("The total distance of all pairs is {}", result);

    let unique_vec1: HashSet<i32> = vec1.clone().into_iter().collect();
    let mut running_sum = 0;

    for i in vec2 {
        if unique_vec1.contains(&i) {
            running_sum += i;
        }
    }

    println!("The running sum is {}", running_sum);
}

pub fn day02() {
    let file = read_to_string("src/data/day02.tsv").expect("Unable to open file");

    let mut safe_report_count = 0;
    for line in file.lines() {
        let report_iter = line.split_whitespace();
        let report: Vec<i32> = report_iter
            .map(|s| s.parse().expect("Unable to parse number"))
            .collect();

        if !(report.iter().is_sorted_by(|a, b| a < b) || report.iter().is_sorted_by(|a, b| a > b)) {
            continue;
        }
        if report.windows(2).any(|pair| (pair[1] - pair[0]).abs() > 3) {
            continue;
        }
        safe_report_count += 1
    }
    println!("Number of safe reports: {}", safe_report_count);
}
