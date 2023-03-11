use std::fmt::Write;
use std::{collections::HashMap, io};

fn main() {
    let temp = io::read_to_string(io::stdin()).unwrap();
    let mut input = temp.split_ascii_whitespace();
    let mut input_iter = || input.next().unwrap();
    let mut result = String::new();
    let (n, m) = (
        input_iter().parse::<i32>().unwrap(),
        input_iter().parse::<i32>().unwrap(),
    );
    let mut name = HashMap::new();
    let mut index = Vec::new();

    for (i, n) in (1..=n).map(|x| (x, input_iter())) {
        name.insert(n, i);
        index.push(n);
    }

    for q in (0..m).map(|_| input_iter()) {
        (match q.parse::<usize>() {
            Ok(i) => writeln!(result, "{}", index[i-1]),
            Err(_) => writeln!(result, "{}", name.get(q).unwrap()),
        })
        .unwrap();
    }
    println!("{result}");
}
