fn parse_input(input_path: &str) -> Vec<String> {
    let input = std::fs::read_to_string(&input_path).expect("Unable to open the path");
    input
        .split(",")
        .map(|name| name.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string())
        .collect()
}

fn word_value(word: &str) -> u8 {
    word.chars().map(|letter| letter as u8 - 64).sum()
}


fn main() {
    let mut score = 0;
    let mut names = parse_input("names.txt");
    names.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for (i, name) in names.iter().enumerate() {
        score += (i + 1) * word_value(name) as usize;
    }
    println!("The total score is {score}.");
}
