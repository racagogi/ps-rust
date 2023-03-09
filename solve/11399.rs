use std::io::stdin;

fn main() {
    let mut temp = String::new();
    let mut ig = String::new();
    stdin().read_line(&mut ig).unwrap();
    stdin().read_line(&mut temp).unwrap();
    let mut queue: Vec<i32> = temp
        .trim()
        .split(" ")
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect();
    queue.sort();
    let mut result = 0;
    for i in 0..queue.len()+1 {
        result += queue[0..i].iter().sum::<i32>();
    }
    println!("{}",result);
}
