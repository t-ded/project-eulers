use std::cmp::max;

const DESIRED_MIN_NUM_DIVISORS: u32 = 500;

fn count_divisors(num: u32) -> u32 {
    
    let mut num_divisors = 0;
    let mut max_divisor = 1;
    
    while max_divisor < num / max_divisor {
        if num % max_divisor == 0 {
            if num / max_divisor == max_divisor {
                num_divisors += 1;
            } else {
                num_divisors += 2;
            }
        }
        max_divisor += 1;
    }
    
    num_divisors
}

fn main() {
    
    let mut max_num_divisors = 0;
    let mut triangle = 0;
    let mut last_natural = 1;

    while max_num_divisors < DESIRED_MIN_NUM_DIVISORS {
        triangle += last_natural;
        last_natural += 1;
        max_num_divisors = max(count_divisors(triangle), max_num_divisors);
    }

    println!("The first triangle number having over 500 divisors is {triangle}.")
}
