use rayon::prelude::*;
use regex::Regex;
use std::{
    fs::read_to_string,
    sync::atomic::{AtomicU64, Ordering},
};

fn find_enabled_memory(memory: &String) -> String {
    let memory = format!("{}{}", "do()", memory.clone());
    let re_do = Regex::new(r"do\(\)").expect("Failed to create regex");
    let re_dont = Regex::new(r"don't\(\)").expect("Failed to create regex");
    let do_offsets = re_do
        .find_iter(&memory)
        .map(|m| m.end())
        .collect::<Vec<usize>>();
    let dont_offsets = re_dont
        .find_iter(&memory)
        .map(|m| m.start())
        .collect::<Vec<usize>>();
    let mut enabled_memory = String::new();
    let mut last_offset = 0;
    for do_offset in do_offsets {
        if do_offset > last_offset {
            if let Some(&dont_offset) = dont_offsets.iter().find(|&&x| x > do_offset) {
                enabled_memory.push_str(&memory[do_offset..dont_offset]);
                last_offset = dont_offset;
            } else {
                enabled_memory.push_str(&memory[do_offset..]);
            }
        }
    }
    enabled_memory
}

fn mulling_it_over(memory: &String) -> u64 {
    let product = AtomicU64::new(0);
    let re = Regex::new(r"mul\(\d+,\d+\)").expect("Failed to create regex");
    re.find_iter(&memory).par_bridge().for_each(|_match| {
        let [n1, n2]: [u64; 2] = _match
            .as_str()
            .replace("mul(", "")
            .replace(")", "")
            .split(",")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|s| s.parse::<u64>().expect("Unable to parse number"))
            .collect::<Vec<u64>>()
            .try_into()
            .expect("Unable to convert vector");
        product.fetch_add(n1 * n2, Ordering::Relaxed);
    });
    product.load(Ordering::Relaxed)
}

pub fn day03() {
    println!("Day 3:");
    let file = read_to_string("src/data/day03.txt").expect("Unable to open file");

    let _conditional_product = AtomicU64::new(0);
    let memory = file.lines().collect::<Vec<_>>().concat();
    let product = mulling_it_over(&memory);
    println!("Full multiplication result: {:?}", product);
    let enabled_memory = find_enabled_memory(&memory);
    let product = mulling_it_over(&enabled_memory);
    println!("Only enabled memory multiplication result: {:?}", product);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mulling_it_over() {
        let memory =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        assert_eq!(mulling_it_over(&memory), 161);
    }

    #[test]
    fn test_mulling_it_over_enabled() {
        let memory =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        let enabled_memory = find_enabled_memory(&memory);
        assert_eq!(mulling_it_over(&enabled_memory), 48);
    }
}
