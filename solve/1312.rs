use std::io::stdin;

fn main() {
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    let read: Vec<i32> = temp
        .strip_suffix('\n')
        .unwrap()
        .split(" ")
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect();
    let a = read[0];
    let b = read[1];
    let c = read[2];
    if a % b == 0 {
        println!("0");
    } else {
        let mut n = a % b;
        let mut result = n/b;
        for _ in 0..c {
            n = (n % b) * 10;
            result = n/b;
        }
        println!("{}", result);
    }
}
