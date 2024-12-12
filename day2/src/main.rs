use std::fs;

// Read each line
// Check
//  - isIncreasing ?
//  || isDecreasing ?
//  || abs diff between consecutive elemts is within range [1,3]
//  get the count of check condition matching rows
fn main() {
    let input = fs::read_to_string("input.txt");
    let mut count = 0;
    for line in input.unwrap().lines() {
        let condition = line
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .map(|entry| {
                let is_increasing = entry.windows(2).all(|w| (w[0] > w[1]));
                let is_decreasing = entry.windows(2).all(|w| (w[0] < w[1]));
                let is_within_range = entry
                    .windows(2)
                    .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);

                (is_increasing || is_decreasing) && is_within_range
            })
            .unwrap_or(false);
        count += if condition { 1 } else { 0 };
    }
    println!("Safe reports {}", count);
}
