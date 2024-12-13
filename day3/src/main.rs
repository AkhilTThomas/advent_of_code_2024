use regex::Regex;
use std::fs;

// match for mul(num,num) pattern
// evaluate the result and sum it.
fn main() {
    let input = "input.txt";
    let content = fs::read_to_string(input).unwrap();

    let re = Regex::new(r"\bmul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();

    let mut sum: u32 = 0;
    for line in content.lines() {
        for capture in re.captures_iter(line) {
            let first = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let second = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
            println!("{} * {}", first, second);
            sum += first * second;
        }
    }

    println!("Sum is {}", sum);
}
