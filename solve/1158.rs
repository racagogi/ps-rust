use std::error::Error;
use std::fmt::Write;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut buf = buf.split_ascii_whitespace();
    let mut input = || buf.next().unwrap();
    let n = input().parse::<usize>().unwrap();
    let m = input().parse::<usize>().unwrap();

    let mut queue: Vec<usize> = (1..=n).collect();
    let mut result = "<".to_string();
    let mut i = 0;
    while !queue.is_empty() {
        i = (i + m - 1) % queue.len();
        if queue.len() == 1 {
            write!(result, "{}>", queue.remove(i)).unwrap();
        } else {
            write!(result, "{}, ", queue.remove(i)).unwrap();
        }
    }
    println!("{result}");
    Ok(())
}
