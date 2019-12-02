use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("The result of part one is {}.", part_one(&input));

    println!("The result of part one is {}.", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|slice| slice.parse::<i32>().unwrap())
        .map(|number| compute(number))
        .sum()
}

fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|slice| slice.parse::<i32>().unwrap())
        .map(|number| compute_iteratively(number))
        .sum()
}

fn compute(number: i32) -> i32 {
    number / 3 - 2
}

fn compute_iteratively(number: i32) -> i32 {
    let mut sum = 0;
    let mut current = number;
    loop {
        current = compute(current);
        if current <= 0 {
            return sum;
        }
        sum += current;
    }
}
