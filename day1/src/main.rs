use std::fs::read_to_string;

// for each element in left
// find count of it in right
// multiply count into left
// sum all elements in left
fn part2() {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let file = "input.txt";
    for line in read_to_string(file).unwrap().lines() {
        if let Some((first, second)) = line.split_once("   ") {
            match first.trim().parse::<u32>() {
                Ok(value) => left.push(value),
                Err(_) => println!("Converion failed!"),
            }
            match second.trim().parse::<u32>() {
                Ok(value) => right.push(value),
                Err(_) => println!("Converion failed!"),
            }
        }
    }
    let similarity_score: u32 = left
        .iter()
        .map(|entry| {
            let count = right.iter().filter(|elem| *elem == entry).count() as u32;
            entry * count
        })
        .sum::<u32>();

    println!("similarity_score = {}", similarity_score);
}

// Read input.txt
// Split left and right into an array
// sort both arrays
// calculate abs diff of arrays
// sum the diff
fn main() {
    println!("Welcome!");
    let mut left = Vec::new();
    let mut right = Vec::new();
    let file = "input.txt";
    for line in read_to_string(file).unwrap().lines() {
        if let Some((first, second)) = line.split_once("   ") {
            match first.trim().parse::<u32>() {
                Ok(value) => left.push(value),
                Err(_) => println!("Converion failed!"),
            }
            match second.trim().parse::<u32>() {
                Ok(value) => right.push(value),
                Err(_) => println!("Converion failed!"),
            }
        }
    }
    left.sort();
    right.sort();
    let sum: u32 = left.iter().zip(&right).map(|(l, r)| l.abs_diff(*r)).sum();
    println!("Sum is {}", sum);

    part2();
}
