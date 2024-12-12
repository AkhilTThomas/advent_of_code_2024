use std::fs::read_to_string;

// for each element in left
// find count of it in right
// multiply count into left
// sum all elements in left
fn main() {
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
