use std::io::{self, stdin};

fn main() {
    let temp = io::read_to_string(stdin()).unwrap();
    let mut temp = temp.split_ascii_whitespace();
    let mut iter = || temp.next().unwrap().parse::<u32>().unwrap();
    let n = iter();
    let mut acc = 0;
    let mut last = 0;
    for _ in 0..n {
        let now = iter();
        if now >= last {
            acc += 1;
        }
        last = now;
    }
    println!("{acc}");
}
