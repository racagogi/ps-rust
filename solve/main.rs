use std::{io, println};

fn main() {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
    let t = temp.trim().parse::<i32>().unwrap();
    for _ in 0..t {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).unwrap();
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).unwrap();
        let n = temp
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
        let mut minv = 1000000;
        let mut maxv = -1000000;
        for i in n {
            if i > maxv {
                maxv = i;
            }
            if i < minv {
                minv = i;
            }
        }
        println!("{minv} {maxv}");
    }
}
