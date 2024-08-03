fn get_largest_prime_factor(mut x: u128) -> u128 {
    
    let mut largest_prime_factor = 1;

    while x % 2 == 0 {
        x = x / 2;
        largest_prime_factor = 2;
    }

    println!("{x}");
    let square_root =  (x as f64).sqrt() as u128;
    println!("{square_root}");
    for i in (3..=square_root).step_by(2) {
        while x % i == 0 {
            x = x / i;
            largest_prime_factor = i;
        }
    }

    if x > 2 {
        largest_prime_factor = x;
    }

    return largest_prime_factor;
}

fn main() {
    let num = 600_851_475_143;
    let max_prime_factor = get_largest_prime_factor(num);
    println!("Largest prime factor of {num} is {max_prime_factor}");
}
