use std::{collections::HashSet, io::stdin};

fn main() {
    let mut temp = String::new();
    stdin().read_line(&mut temp);
    let nm: Vec<i32> = temp
        .trim()
        .split(" ")
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect();
    let mut heard = HashSet::new();
    let mut listen = HashSet::new();
    let mut not_hl: Vec<String> = Vec::new();
    for _ in 0..nm[0] {
        let mut temp = String::new();
        stdin().read_line(&mut temp);
        heard.insert(String::from(temp.trim()));
    }
    for _ in 0..nm[1] {
        let mut temp = String::new();
        stdin().read_line(&mut temp);
        listen.insert(String::from(temp.trim()));
    }
    for i in heard.intersection(&listen) {
        not_hl.push(i.to_string());
    }
    not_hl.sort();
    println!("{}", not_hl.len());

    for i in not_hl {
        println!("{i}");
    }
}
