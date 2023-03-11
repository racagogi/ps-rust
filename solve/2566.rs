use std::{collections::HashMap, io};

fn main() {
    let temp = io::read_to_string(io::stdin()).unwrap();
    let mut input = temp.split_ascii_whitespace();
    let mut input_iter = || input.next().unwrap();
    let mut board = HashMap::new();
    let mut max = 0;
    for i in 1..10 {
        for j in 1..10 {
            let n = input_iter().parse::<i32>().unwrap();
            match max.cmp(&n) {
                std::cmp::Ordering::Less => {
                    max = n;
                    board.insert(n, (i, j));
                }
                _ => {
                    board.insert(n, (i, j));
                }
            }
        }
    }
    println!("{max}");
    let (i, j) = board.get(&max).unwrap();
    println!("{} {}", i, j);
}
