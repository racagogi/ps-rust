use std::error::Error;
use std::fmt::Write;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut buf = buf.split_ascii_whitespace();
    let mut input = || buf.next().unwrap();
    let n = input().chars().map(|x| String::from(x).parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut result = String::new();
    for i in 0..n.len() {
        if i == 0 {
            write!(result,"{:b}",n[i]).unwrap();
        }else {
            write!(result,"{:03b}",n[i]).unwrap();
        }
    }
    println!("{}",result);
    Ok(())
}
