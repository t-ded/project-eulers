use String;

fn is_palindromic(x: u64) -> bool {
    let x_str = x.to_string();
    x_str == x_str.chars().rev().collect::<String>()
}

fn main() {

    let mut max_palindrome = 0;

    'outer: for left in (100..=999).rev() {
        for right in (100..=left).rev() {
            let prod = left * right;
            if prod < max_palindrome {
                continue 'outer;
            } else if is_palindromic(prod) {
                max_palindrome = prod;
                continue 'outer;
            }
        }
    }

    println!("Largest palindrome product of 3-digit numbers is {max_palindrome}!");
}
