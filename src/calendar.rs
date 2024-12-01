use std::fs::read_to_string;

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
}
