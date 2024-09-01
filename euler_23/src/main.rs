use std::collections::HashSet;

fn get_proper_divisor_sum(n: usize) -> usize {
    let mut sum = 1;
    let upper_bound = (n as f32).sqrt() as usize;
    for divisor in 2..=upper_bound {
        let leftover = n % divisor;
        let quotient = n / divisor;
        if leftover == 0 {
            sum += divisor;
            if quotient != divisor { sum += quotient }
        }
    }
    sum
}

fn main() {
    let mut num_sum = 0;
    let all_abundant: HashSet<_> = (1..=28_123).filter(|&n| get_proper_divisor_sum(n as usize) > n as usize).collect();
    'outer: for num in 1..=28_123 {
        for abundant in all_abundant.iter() {
            if all_abundant.contains(&(num - abundant)) {
                continue 'outer
            }
        }
        num_sum += num;
    }
    println!("Proper divisor sum of 28 is {:?}", get_proper_divisor_sum(28));
    println!("Proper divisor sum of 12 is {:?}", get_proper_divisor_sum(12));
    println!("Sum of numbers which cannot be created as sum of two abundant numbers is {num_sum}.");
    println!("Hello, world!");
}
