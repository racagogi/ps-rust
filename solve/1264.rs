use std::error::Error;
use std::fmt::Write;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut result = String::new();
    for l in buf.lines().map(|x| x.chars()) {
        if l.as_str() == "#" {
            break;
        }
        writeln!(
            result,
            "{:?}",
            l.filter(|&x| x == 'a'
                || x == 'e'
                || x == 'i'
                || x == 'o'
                || x == 'u'
                || x == 'A'
                || x == 'E'
                || x == 'I'
                || x == 'O'
                || x == 'U')
                .collect::<Vec<char>>()
                .len()
        )?;
    }
    println!("{result}");
    Ok(())
}
