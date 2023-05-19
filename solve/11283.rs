use std::io;

fn main() -> Result<(), io::Error> {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp)?;
    let c = temp.trim().parse::<char>().unwrap();
    let n = c as u32 - '가' as u32 + 1;
    println!("{n}");
    Ok(())
}
