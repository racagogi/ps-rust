use std::fmt::Write;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp)?;
    let n = temp.trim().parse::<u32>().unwrap();
    let heart = vec![
        " @@@   @@@ ",
        "@   @ @   @",
        "@    @    @",
        "@         @",
        " @       @ ",
        "  @     @  ",
        "   @   @   ",
        "    @ @    ",
        "     @     ",
    ];
    let mut sol = String::new();
    for i in 0..9 {
        for j in 0..n {
            write!(sol, "{}", heart[i]).unwrap();
            if j < n - 1 {
                write!(sol, " ").unwrap();
            }
        }
        writeln!(sol, "").unwrap();
    }
    println!("{sol}");
    Ok(())
}
