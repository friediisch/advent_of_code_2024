use rayon::prelude::*;
use std::{
    fs::read_to_string,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

// TODO: Change brute force solution to a more efficient one.
fn check_report_safety_with_problem_dampener(report: &Vec<i32>) -> bool {
    let safe_with_dampener = AtomicBool::new(false);
    report
        .clone()
        .par_iter()
        .enumerate()
        .for_each(|(j, _level)| {
            if !safe_with_dampener.load(Ordering::Relaxed) {
                let mut report_permutation = report.clone();
                report_permutation.remove(j);
                if check_report_safety(&report_permutation) {
                    safe_with_dampener.store(true, Ordering::Relaxed);
                };
            }
        });
    safe_with_dampener.load(Ordering::Relaxed)
}

fn check_report_safety(report: &Vec<i32>) -> bool {
    return (report.iter().is_sorted_by(|a, b| a < b) || report.iter().is_sorted_by(|a, b| a > b))
        && !(report.windows(2).any(|pair| (pair[1] - pair[0]).abs() > 3));
}

pub fn day02() {
    print!("    .--'~  ,* ~ |        |  >o<   \\_\\_\\|_/__/   |  ");
    let file = read_to_string("src/data/day02.tsv").expect("Unable to open file");

    let safe_report_count_without_problem_dampener = AtomicU64::new(0);
    let safe_report_count_with_problem_dampener = AtomicU64::new(0);
    file.lines()
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|line| {
            let report_iter = line.split_whitespace();
            let report: Vec<i32> = report_iter
                .map(|s| s.parse().expect("Unable to parse number"))
                .collect();
            if check_report_safety(&report) {
                safe_report_count_without_problem_dampener.fetch_add(1, Ordering::Relaxed);
                safe_report_count_with_problem_dampener.fetch_add(1, Ordering::Relaxed);
            } else {
                if check_report_safety_with_problem_dampener(&report) {
                    safe_report_count_with_problem_dampener.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
    print!(
        "safe reports: {:?}, ",
        safe_report_count_without_problem_dampener
    );
    println!(
        "safe reports with problem dampener: {:?}",
        safe_report_count_with_problem_dampener
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_report_safety() {
        assert_eq!(check_report_safety(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(check_report_safety(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(check_report_safety(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(check_report_safety(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(check_report_safety(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(check_report_safety(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_check_report_safety_with_problem_dampener() {
        assert_eq!(
            check_report_safety_with_problem_dampener(&vec![7, 6, 4, 2, 1]),
            true
        );
        assert_eq!(
            check_report_safety_with_problem_dampener(&vec![1, 2, 7, 8, 9]),
            false
        );
        assert_eq!(
            check_report_safety_with_problem_dampener(&vec![9, 7, 6, 2, 1]),
            false
        );
        assert_eq!(
            check_report_safety_with_problem_dampener(&vec![1, 3, 2, 4, 5]),
            true
        );
        assert_eq!(
            check_report_safety_with_problem_dampener(&vec![8, 6, 4, 4, 1]),
            true
        );
        assert_eq!(
            check_report_safety_with_problem_dampener(&vec![1, 3, 6, 7, 9]),
            true
        );
    }
}
