use std::io;

fn main() -> Result<(), io::Error> {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp)?;
    let n = temp.trim().parse::<u32>().unwrap();
    let c = char::from_u32('가' as u32 + n - 1).unwrap();
    println!("{c}");
    Ok(())
}
