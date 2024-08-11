use num_bigint::BigUint;
use num_traits::One;


fn main() {
    let mult = BigUint::from(2u8);
    let mut res = BigUint::one();
    for _ in 0..1_000 {
        res = res * &mult;
    }

    let mut sum = 0;
    let res_str = res.to_string();
    for digit in res_str.chars().into_iter() {
        sum += digit.to_digit(10).unwrap();
    }

    println!("Sum of digits of 2 ^ 1_000 is {sum}.");
}
