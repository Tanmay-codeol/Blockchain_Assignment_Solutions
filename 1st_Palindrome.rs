fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase(); // Convert input to lowercase for case-insensitive comparison
    let input_chars: Vec<char> = input.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed_chars: Vec<char> = input_chars.iter().rev().cloned().collect();
    
    input_chars == reversed_chars
}

fn main() {
    let test_cases = vec![
        "A man, a plan, a canal, Panama",
        "Was it a car or a cat I saw?",
        "No 'x' in Nixon",
        "Racecar",
        "Hello, world"
    ];

    for &test_case in &test_cases {
        println!("\"{}\" is a palindrome: {}", test_case, is_palindrome(test_case));
    }
}
