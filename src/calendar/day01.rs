use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
fn calculate_distance(vec1: &Vec<i32>, vec2: &Vec<i32>) -> i32 {
    let mut vec1 = vec1.clone();
    let mut vec2 = vec2.clone();
    vec1.sort();
    vec2.sort();
    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn calculate_similarity(vec1: &Vec<i32>, vec2: &Vec<i32>) -> i32 {
    let unique_vec1: HashSet<i32> = vec1.clone().into_iter().collect();
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    for i in vec2 {
        if unique_vec1.contains(&i) {
            *frequency_map.entry(*i).or_insert(0) += 1;
        }
    }
    let mut running_sum = 0;
    for i in vec1 {
        if frequency_map.contains_key(&i) {
            let key = *i;
            let value = *frequency_map.get(&i).unwrap();
            running_sum += key * value;
        }
    }
    running_sum
}

pub fn day01() {
    print!("       .--'~ ~ ~|        .-' *       \\  /     '-.  1: ");
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

    let distance = calculate_distance(&vec1, &vec2);

    print!("Location pair distance: {}, ", distance);

    let similarity = calculate_similarity(&vec1, &vec2);

    println!("similarity: {}", similarity);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input_vectors() -> (Vec<i32>, Vec<i32>) {
        (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    }

    #[test]
    fn test_calculate_distance() {
        let (vec1, vec2) = get_input_vectors();
        assert_eq!(calculate_distance(&vec1, &vec2), 11);
    }

    #[test]
    fn test_calculate_similarity() {
        let (vec1, vec2) = get_input_vectors();
        assert_eq!(calculate_similarity(&vec1, &vec2), 31);
    }
}
