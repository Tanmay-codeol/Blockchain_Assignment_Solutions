// similar to the 4th code but here we are checking without using the function


fn main() {
    let test_cases = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    
    for &num in &test_cases {
        let mut is_prime = true;
        if num <= 1 {
            is_prime = false;
        } else if num <= 3 {
            is_prime = true;
        } else if num % 2 == 0 || num % 3 == 0 {
            is_prime = false;
        } else {
            let mut i = 5;
            while i * i <= num {
                if num % i == 0 || num % (i + 2) == 0 {
                    is_prime = false;
                    break;
                }
                i += 6;
            }
        }
        println!("{} is prime: {}", num, is_prime);
    }
}
