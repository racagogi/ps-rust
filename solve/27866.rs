use std::io::stdin;

fn main() {
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    let chars:Vec<char> = temp.chars().collect();
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    let n = temp.trim().parse::<usize>().unwrap();
    println!("{}",chars[n-1]);
}
