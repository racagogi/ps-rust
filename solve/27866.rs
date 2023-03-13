use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut buf = buf.lines();
    let mut input = || buf.next().unwrap();
    let mut s = input().chars();
    let i = input().parse::<usize>()?;
    println!("{}", s.nth(i-1).unwrap());
    Ok(())
}
