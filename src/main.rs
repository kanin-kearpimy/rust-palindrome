fn main() {
    println!("Palindrome.");

    let string = "anna";

    let is_palindrome: bool = check_palindrom(string);

    println!("is {} Palindrome? : {}", string, is_palindrome);
}

fn check_palindrom(string: &str) -> bool {
    let mut is_palindrome: bool = true;
    for (c1, c2) in string.chars().zip(string.chars().rev()) {
        if c1 != c2 {
            is_palindrome = false;
        }
    }
    return is_palindrome;
}