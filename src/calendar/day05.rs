use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::read_to_string;

fn sum_middle_pages(updates: &Vec<Vec<u32>>) -> u64 {
    updates
        .par_iter()
        .map(|update| update[update.len() / 2] as u64)
        .sum()
}

fn split_ordered_updates(
    rules: &Vec<(u32, u32)>,
    updates: &Vec<Vec<u32>>,
) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    updates
        .par_iter()
        .cloned()
        .partition(|update| validate_update_order(&rules, &update))
}

fn validate_update_order(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> bool {
    !rules.par_iter().any(|rule| {
        if update.contains(&rule.0) && update.contains(&rule.1) {
            let rule_1_index = update.iter().position(|&page| page == rule.0).unwrap();
            let rule_2_index = update.iter().position(|&page| page == rule.1).unwrap();
            rule_1_index > rule_2_index
        } else {
            false
        }
    })
}

fn correct_update_order(rules: &Vec<(u32, u32)>, mut update: Vec<u32>) -> Vec<u32> {
    while !validate_update_order(&rules, &update) {
        rules.iter().for_each(|rule| {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                let rule_1_index = update.iter().position(|&page| page == rule.0).unwrap();
                let rule_2_index = update.iter().position(|&page| page == rule.1).unwrap();
                if rule_1_index > rule_2_index {
                    update.swap(rule_1_index, rule_2_index);
                }
            }
        });
    }
    update
}

pub fn day05() {
    print!(r"|_.~._#'.. ~ ~ *|        | _| |_    ..\_\_ ..'* |  5: ");
    let file = read_to_string("src/data/day05.txt").expect("Unable to open file");
    let mut rules: Vec<(u32, u32)> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];
    let mut empty_line_found = false;
    for line in file.lines().into_iter() {
        if line.is_empty() {
            empty_line_found = true
        } else if !empty_line_found {
            let parts: Vec<&str> = line.split("|").collect();
            let page_1: u32 = parts[0].parse().expect("Failed to parse page 1");
            let page_2: u32 = parts[1].parse().expect("Failed to parse page 2");
            rules.push((page_1, page_2))
        } else {
            updates.push(
                line.split(",")
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|page| {
                        page.parse()
                            .expect(&format!("Failed to parse page {} in line {}", page, line))
                    })
                    .collect::<Vec<u32>>(),
            );
        }
    }
    let (ordered_updates, unordered_updates) = split_ordered_updates(&rules, &updates);
    let result_task_1 = sum_middle_pages(&ordered_updates);
    print!("Middle sum of already valid updates 1: {}, ", result_task_1);
    let corrected_updates: Vec<Vec<u32>> = unordered_updates
        .par_iter()
        .map(|update| correct_update_order(&rules, update.to_owned()))
        .collect();
    let result_task_2 = sum_middle_pages(&corrected_updates);
    println!(
        "Middle sum of remaining corrected updates 2: {}",
        result_task_2
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_test_rules() -> Vec<(u32, u32)> {
        vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ]
    }

    fn get_test_updates() -> Vec<Vec<u32>> {
        vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ]
    }

    fn get_test_validities() -> Vec<bool> {
        vec![true, true, true, false, false, false]
    }

    fn get_corrected_updates() -> Vec<Vec<u32>> {
        vec![
            vec![97, 75, 47, 61, 53],
            vec![61, 29, 13],
            vec![97, 75, 47, 29, 13],
        ]
    }

    fn get_invalid_updates() -> Vec<Vec<u32>> {
        vec![
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ]
    }

    #[test]
    fn test_validate_update_order() {
        let rules = get_test_rules();
        let updates = get_test_updates();
        let test_validities = get_test_validities();
        let computed_validites = updates
            .iter()
            .map(|update| validate_update_order(&rules, &update))
            .collect::<Vec<_>>();
        assert_eq!(test_validities, computed_validites)
    }

    #[test]
    fn test_get_ordered_updates() {
        let rules = get_test_rules();
        let updates = get_test_updates();
        let ordered_updates: Vec<Vec<u32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ];
        assert_eq!(ordered_updates, split_ordered_updates(&rules, &updates).0)
    }

    #[test]
    fn test_sum_middle_pages_without_reordering() {
        let rules = get_test_rules();
        let updates = get_test_updates();
        let (ordered_updates, _) = split_ordered_updates(&rules, &updates);
        let sum = sum_middle_pages(&ordered_updates);
        assert_eq!(sum, 143);
    }

    #[test]
    fn test_correct_update_order() {
        let rules = get_test_rules();
        let invalid_updates = get_invalid_updates();
        let test_corrected_updates = get_corrected_updates();
        let corrected_updates = invalid_updates
            .iter()
            .map(|update| correct_update_order(&rules, update.to_vec()))
            .collect::<Vec<_>>();
        assert_eq!(test_corrected_updates, corrected_updates);
    }

    #[test]
    fn test_sum_middle_pages_with_reordering() {
        let corrected_updates = get_corrected_updates();
        let sum = sum_middle_pages(&corrected_updates);
        assert_eq!(sum, 123);
    }
}
