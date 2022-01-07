use std::io::stdin;

fn main() {
    println!("Palindrome.");
    println!("If you want to exit. please type ==> :q\n");
    loop {
        let mut pre_string = String::new();
        println!("Please input your thing ==>");
        stdin().read_line(&mut pre_string).expect("Please put correct string.");
        
        let string: &str = &pre_string.trim().replace(" ", "");
        if string == ":q" {
            println!("==========");
            println!("THANK YOU.");
            println!("==========");
            break;
        }
        if string == "" {
            println!("Please put correct string!\n");
            continue;
        }

        let is_palindrome: bool = check_palindrom(string);

        println!("is {} Palindrome? : {}\n", string, match is_palindrome { true => "YES", false => "NO" });
    }
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