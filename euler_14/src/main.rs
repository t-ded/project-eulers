use std::cmp::max;

const MAX_STARTING_NUM: u32 = 1_000_000;

struct Collatz {
    num: u128,
    n_th_in_chain: u32
}

impl Collatz {
    fn move_to_next(&mut self) -> () {
        if self.num % 2 == 0 {
            self.num /= 2;
        } else {
            self.num = 3 * self.num + 1;
        }
        self.n_th_in_chain += 1;
    }

    fn is_last(&self) -> bool {
        self.num == 1
    }

    fn new(num: u128) -> Collatz {
        Collatz {
            num,
            n_th_in_chain: 1
        }
    }
}


fn main() {

    let mut longest_chain = 0;
    let mut start_number_with_longest_chain= 1;

    for i in 1..MAX_STARTING_NUM {
        let mut col =  Collatz::new(i as u128);
        while !col.is_last() {
            col.move_to_next();
        }
        if col.n_th_in_chain > longest_chain {
            longest_chain = col.n_th_in_chain;
            start_number_with_longest_chain = i;
        }
        longest_chain = max(longest_chain, col.n_th_in_chain);
    }

    println!("Longest chain produced was of length {longest_chain} from starting number {start_number_with_longest_chain}.");
}
