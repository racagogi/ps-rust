use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut buf = buf.lines();
    let mut input = || buf.next().unwrap();
    let a = input().parse::<usize>()?;
    let b = input().parse::<usize>()?;
    let c = input().parse::<usize>()?;
    let mut digit = [0; 10];
    for c in (a * b * c).to_string().chars() {
        let i = c.to_string().parse::<usize>()?;
        digit[i] += 1;
    }
    for i in digit {
        println!("{i}");
    }
    Ok(())
}
