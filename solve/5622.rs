use std::io::stdin;

fn main() {
    let mut word = String::new();
    stdin().read_line(&mut word).unwrap();
    let words = word.trim().chars();
    let mut result = 0;
    for c in words {
        match c {
            'A' | 'B' | 'C' => result += 3,
            'D' | 'E' | 'F' => result += 4,
            'G' | 'H' | 'I' => result += 5,
            'J' | 'K' | 'L' => result += 6,
            'M' | 'N' | 'O' => result += 7,
            'P' | 'Q' | 'R' | 'S' => result += 8,
            'T' | 'U' | 'V' => result += 9,
            'W' | 'X' | 'Y' | 'Z' => result += 10,
            _ => (),
        }
    }
    println!("{result}");
}
