use std::error::Error;
use std::fmt::Write;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut buf = buf.split_ascii_whitespace();
    let mut input = || buf.next().unwrap();
    let n = input().parse::<usize>().unwrap();
    let m = input().parse::<usize>().unwrap();
    let mut result = String::new();
    let numv: Vec<usize> = (1..=n).collect();
    dfs(&numv, m, 0, &mut Vec::new(), &mut result);
    println!("{}", result);
    Ok(())
}

fn dfs(numv: &Vec<usize>, m: usize, index: usize, temp: &mut Vec<usize>, result: &mut String) {
    if m == 0 {
        writeln!(
            result,
            "{}",
            temp.iter()
                .map(|&x| x.to_string() + " ")
                .collect::<String>()
        )
        .unwrap();
        return;
    }
    for (i, &n) in numv.iter().enumerate().skip(index).take(numv.len() - m + 1) {
        if temp.contains(&n) {
            continue;
        }
        temp.push(n);
        dfs(numv, m - 1, i + 1, temp, result);
        temp.pop();
    }
}
