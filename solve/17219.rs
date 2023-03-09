use std::fmt::Write;
use std::{collections::HashMap, io::stdin};

fn main() {
    let mut pwmap = HashMap::new();
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    let mut result = String::new();
    let nm: Vec<i32> = temp
        .trim()
        .split(" ")
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect();
    for _ in 0..nm[0] {
        let mut temp = String::new();
        stdin().read_line(&mut temp).unwrap();
        let site_pw: Vec<String> = temp.trim().split(" ").map(|x| x.to_string()).collect();
        pwmap.insert(site_pw[0].clone(), site_pw[1].clone());
    }
    for _ in 0..nm[1] {
        let mut temp = String::new();
        stdin().read_line(&mut temp).unwrap();
        let site = temp.trim().to_string();
        writeln!(result, "{}", pwmap.get(&site).unwrap()).unwrap();
    }
    print!("{}", result);
}
