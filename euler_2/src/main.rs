const MAX_NUMBER_TO_CONSIDER: u64 = 4_000_000;

fn main() {

    let mut first = 1;
    let mut second = 2;
    let mut new_term;
    let mut sum = 0;

    while second <= MAX_NUMBER_TO_CONSIDER {

        if second % 2 == 0 {
            sum += second;
        }

        new_term = first + second;
        first = second;
        second = new_term;

    }
    
    println!("Sum of all even-valued Fibonacci numbers below {MAX_NUMBER_TO_CONSIDER} is {sum}");
}
