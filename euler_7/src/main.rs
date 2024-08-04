const NUM_PRIMES: usize = 10_001;

fn sieve_of_eratosthenes(max_num: usize) -> Vec<usize> {

    let mut is_prime = vec![true; max_num + 1];
    let mut num = 2;
    while num * num <= max_num {
        if is_prime[num] {
            for multiple in ((num * num)..(max_num + 1)).step_by(num) {
                is_prime[multiple] = false;
            }
        }
        num += 1;
    }

    let mut primes = Vec::new();
    for number in 2..(max_num + 1){
        if is_prime[number] {
            primes.push(number);
        }
    }
    return primes

}


fn main() {

    let mut current_max = 500;

    loop {
        let primes = sieve_of_eratosthenes(current_max);
        if primes.len() < NUM_PRIMES {
            current_max *= 2;
        } else {
            if let Some(last) = primes[0..NUM_PRIMES].last() {
                println!("The 10_001st prime number is {}", last);
            }
            break;
        }
    }
}
