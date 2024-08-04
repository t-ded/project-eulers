const MAX_NUMBER: usize = 2_000_000;

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
    let sum: usize = sieve_of_eratosthenes(MAX_NUMBER).into_iter().sum();
    println!("The sum of all primes below {MAX_NUMBER} is {sum}.");
}
