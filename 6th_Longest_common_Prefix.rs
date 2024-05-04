fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new(); 
    }
    
    let first_str = &strs[0]; 
    let mut prefix = String::new();
    
    'outer: for (i, ch) in first_str.chars().enumerate() {
        for other_str in strs.iter().skip(1) {
            if i >= other_str.len() || other_str.chars().nth(i) != Some(ch) {
                break 'outer; 
            }
        }
        prefix.push(ch); 
    }
    
    prefix
}

fn main() {
    let input1 = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let input2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    
    println!("Longest common prefix of input1: {}", longest_common_prefix(&input1));
    println!("Longest common prefix of input2: {}", longest_common_prefix(&input2));
}
