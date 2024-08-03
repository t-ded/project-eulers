const NEED_TO_DIVIDE: [u32; 10] = [20, 19, 18, 17, 16, 15, 14, 13, 12, 11];

fn is_divisible(x: u32) -> bool {
    for num in NEED_TO_DIVIDE {
        if x % num != 0 {
            return false;
        }
    }
    return true;
}

fn main() {

    let mut num = 40;

    loop {
        if is_divisible(num) {
            println!("Smallest number divisible by numbers 1-20 is {num}");
            break;
        }
        num += 20;
    }
}
