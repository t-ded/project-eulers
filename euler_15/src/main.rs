fn comb(n: usize, k: usize) -> usize {
    let k = if k > n - k { n - k } else { k };
    let mut result = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}


fn main() {
    println!("The number of possible paths is {}.", comb(40, 20));
}
