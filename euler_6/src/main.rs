const MAX_NUMBER: usize = 100;

fn sum_of_squares(numbers: &Vec<usize>) -> usize {
    let mut sum = 0;
    for num in numbers {
        sum += num.pow(2);
    }
    return sum;
}

fn square_of_sum(numbers: &Vec<usize>) -> usize {
    let sum: usize = numbers.iter().sum();
    return sum.pow(2);
}

fn main() {

    let numbers = (1..=MAX_NUMBER).collect();
    println!("Sum of squares: {}", sum_of_squares(&numbers));
    println!("Square of sum: {}", square_of_sum(&numbers));

    println!(
        "Difference between square of sums and sum of squares of the first {MAX_NUMBER} natural numbers is {}",
        square_of_sum(&numbers) - sum_of_squares(&numbers)
    );

}
