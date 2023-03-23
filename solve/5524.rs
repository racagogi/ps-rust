use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut buf = buf.split_ascii_whitespace();
    let mut input = || buf.next().unwrap();
    let n = input().parse::<i32>().unwrap();
    for _ in 0..n {
        let name = input().to_lowercase();
        println!("{}",name);
    }

}
