fn get_dn(n: usize) -> usize {
    let mut sum = 1;
    let upper_bound = (n as f32).sqrt() as usize;
    for divisor in 2..upper_bound {
        let leftover = n % divisor;
        let quotient = n / divisor;
        if leftover == 0 {
            sum += divisor;
            sum += quotient
        }
    }
    if n % upper_bound == 0 { sum += upper_bound }
    sum
}

fn main() {
    let mut amicable_pairs = vec![0; 10_000];
    let mut sum = 0;
    for i in 1..10_000 {
        if amicable_pairs[i] != 0 { continue }
        let dn = get_dn(i);
        if dn < 10_000 && dn != i {
            if get_dn(dn) == i {
                amicable_pairs[i] = dn;
                amicable_pairs[dn] = i;
                println!("Amicable pair found: {i}, {dn}");
                sum += dn + i;
            }
        }
    }
    println!("The sum of all amicable pairs below 10 000 is {sum}");
}
