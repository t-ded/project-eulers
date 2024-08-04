const SUM: u32 = 1_000;

fn main() {
    for a in 1..=(SUM / 3 - 1) {
        for b in (a + 1)..=(SUM / 2 - a / 2 - 1) {
            let c = SUM - b - a;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                println!("The triplet is {a}, {b}, {c} with their product being {}.", a * b * c);
                return;
            }
        }
    }
}
