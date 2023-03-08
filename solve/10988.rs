use std::io::stdin;

fn main() {
    let mut word = String::new();
    stdin().read_line(&mut word);
    if palindrome(&word.trim()) {
        println!("1");
    } else {
        println!("0");
    }
}

fn palindrome(word: &str) -> bool {
    let rev = String::from_iter(word.chars().rev());
    rev == word
}
