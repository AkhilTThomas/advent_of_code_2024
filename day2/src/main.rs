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
        let single_line_entry: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        if is_safe(&single_line_entry) {
            count += 1;
        }
    }

    let input_part_two = fs::read_to_string("input.txt");
    println!("Safe reports {}", count);
    let mut safe_report_count: u32 = 0;
    for line in input_part_two.unwrap().lines() {
        let single_line_entry: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let is_safe_with_damper = (0..single_line_entry.len()).any(|skip_index| {
            let modified_line_entry: Vec<u32> = single_line_entry
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != skip_index)
                .map(|(_, &x)| x)
                .collect();

            is_safe(&modified_line_entry)
        });

        if is_safe_with_damper {
            safe_report_count += 1;
        }
    }

    println!("Safe reports after dampner {}", safe_report_count);
}

fn is_safe(entry: &[u32]) -> bool {
    let is_increasing = entry.windows(2).all(|w| (w[0] > w[1]));
    let is_decreasing = entry.windows(2).all(|w| (w[0] < w[1]));
    let is_within_range = entry
        .windows(2)
        .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);

    (is_increasing || is_decreasing) && is_within_range
}
