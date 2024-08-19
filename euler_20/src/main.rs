use num_bigint::BigUint;
use num_traits::One;


fn factorial(n: u128) -> BigUint {
    let mut fact = BigUint::one();
    for i in 2..=n {
        fact *= i;
    }
    fact
}

fn sum_digits(x: impl ToString) -> u128 {
    let x_str = x.to_string();
    let mut sum = 0;
    for digit in x_str.chars() {
        sum += digit.to_digit(10).unwrap() as u128;
    }
    sum
}

fn main() {
    println!("Sum of digits of 100! is {}.", sum_digits(factorial(99)));
}
