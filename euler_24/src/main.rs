fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn perm_unrank(rank: usize, size: usize) -> Vec<usize> {
    let n = size;
    if n == 1 {
        return vec![0]
    }

    let fact = factorial(n - 1);
    let blocks_count = rank / fact;
    let first_elem = blocks_count;
    let mut perm = perm_unrank(rank % fact, n - 1);

    for j in 0..perm.len() {
        if perm[j] >= first_elem {
            perm[j] += 1;
        }
    }

    [vec![first_elem], perm].concat()
}

fn main() {
    println!("The millionth permutation of numbers 0-9 in lexicographical order is {:?}.", perm_unrank(999_999, 10));
}
