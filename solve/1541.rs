use std::io::stdin;

fn main() {
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    let token = minus_parse(temp.trim());
    let evel: Vec<i32> = token.iter().map(|x| cal_plus(x)).collect();
    let mut result = evel[0];
    for i in 1..evel.len() {
        result -= evel[i];
    }

    println!("{:?}", result);
}

fn cal_plus(e: &str) -> i32 {
    let ev: Vec<i32> = e
        .split("+")
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect();
    let mut acc = 0;
    for i in ev {
        acc += i;
    }
    acc
}

fn minus_parse(e: &str) -> Vec<&str> {
    let ev: Vec<&str> = e.split("-").collect();
    ev
}
