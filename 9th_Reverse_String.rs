fn reverse_string(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

fn main() {
    let input = "Hello, world!";
    let reversed = reverse_string(input);
    println!("Original: {}", input);
    println!("Reversed: {}", reversed);
}
