use std::io::stdin;

fn main() {
    let mut word = String::new();
    stdin().read_line(&mut word);
    let n: i32 = word.trim().parse().unwrap();
    for _ in 0..n {
        repeat_string();
    }
}

fn repeat_string() -> () {
    let mut string = String::new();
    stdin().read_line(&mut string).unwrap();
    let parse = string.trim().split(" ").collect::<Vec<&str>>();
    let mut result = String::new();
    let n = parse[0].parse::<i32>().unwrap();
    for c in parse[1].chars() {
        for _ in 0..n {
            result.push(c)
        }
    }
    println!("{result}");
}
