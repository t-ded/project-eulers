const MAX_NUM: u128 = 1_000;

fn main() {

    let mut sum_of_numbers: u128 = 0;
    let mut i: u128 = 1;

    loop {

        let multiple_three = 3 * i;
        let multiple_five = 5 * i;

        if multiple_three < MAX_NUM {
            sum_of_numbers += multiple_three;
            if multiple_five < MAX_NUM && multiple_five % 3 != 0 {
                sum_of_numbers += multiple_five;
            }
        } else {
            break
        }

        i += 1;
    }

    println!("Sum of all multiples of 3 and 5 below {MAX_NUM} is {sum_of_numbers}.");
}
