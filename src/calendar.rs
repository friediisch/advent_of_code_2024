use rayon::prelude::*;
use std::{
    collections::HashSet,
    fs::read_to_string,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

pub fn day01() {
    println!("Day 1:");
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

    println!("The distance of all location pairs is {}", result);

    let unique_vec1: HashSet<i32> = vec1.clone().into_iter().collect();
    let mut running_sum = 0;

    for i in vec2 {
        if unique_vec1.contains(&i) {
            running_sum += i;
        }
    }

    println!("The similarity is {}", running_sum);
}

fn check_safety(report: &Vec<i32>) -> bool {
    return (report.iter().is_sorted_by(|a, b: &&i32| a < b)
        || report.iter().is_sorted_by(|a, b: &&i32| a > b))
        && !(report.windows(2).any(|pair| (pair[1] - pair[0]).abs() > 3));
}

pub fn day02() {
    println!("Day 2:");
    let file = read_to_string("src/data/day02.tsv").expect("Unable to open file");

    let safe_report_count_without_problem_dampener = AtomicUsize::new(0);
    let safe_report_count_with_problem_dampener = AtomicUsize::new(0);
    file.lines()
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|line| {
            let report_iter = line.split_whitespace();
            let report: Vec<i32> = report_iter
                .map(|s| s.parse().expect("Unable to parse number"))
                .collect();
            if check_safety(&report) {
                safe_report_count_without_problem_dampener.fetch_add(1, Ordering::Relaxed);
                safe_report_count_with_problem_dampener.fetch_add(1, Ordering::Relaxed);
            } else {
                let safe_with_dampener = AtomicBool::new(false);
                report
                    .clone()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .par_iter()
                    .enumerate()
                    .for_each(|(j, _level)| {
                        let mut report_permutation = report.clone();
                        report_permutation.remove(j);
                        if check_safety(&report_permutation) {
                            safe_with_dampener.store(true, Ordering::Relaxed);
                        };
                    });
                if safe_with_dampener.load(Ordering::Relaxed) {
                    safe_report_count_with_problem_dampener.fetch_add(1, Ordering::Relaxed);
                };
            }
        });
    println!(
        "Safe reports without problem dampener: {:?}",
        safe_report_count_without_problem_dampener
    );
    println!(
        "Safe reports with problem dampener: {:?}",
        safe_report_count_with_problem_dampener
    );
}
