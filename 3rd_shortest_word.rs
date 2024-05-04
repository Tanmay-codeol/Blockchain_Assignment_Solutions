fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let input = "The quick brown fox jumps over the lazy dog";
    if let Some(shortest) = shortest_word(input) {
        println!("The shortest word is: {}", shortest);
    } else {
        println!("No words found in the input string.");
    }
}
