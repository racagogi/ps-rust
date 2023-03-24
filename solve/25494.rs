use std::io;

fn main() {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
    let n = temp.trim().parse::<i32>().unwrap();
    for _ in 0..n {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).unwrap();
        let input = temp
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut result = 0;
        for i in 1..=input[0] {
            for j in 1..=input[1] {
                for k in 1..=input[2] {
                    if i % j == j % k && j % k == k % i {
                        result += 1;
                    }
                }
            }
        }
        println!("{result}");
    }
}
