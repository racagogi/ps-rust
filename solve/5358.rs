use std::io::stdin;

fn main() {
    while true {
        let mut temp = String::new();
        let eof = stdin().read_line(&mut temp).unwrap();
        if eof == 0 {
            break;
        }
        println!("{}", &swap_ie(&temp.trim()));
    }
}

fn swap_ie(word: &str) -> String {
    let mut sword = String::new();
    for s in word.chars() {
        match s {
            'i' => sword.push('e'),
            'I' => sword.push('E'),
            'e' => sword.push('i'),
            'E' => sword.push('I'),
            _ => sword.push(s),
        }
    }
    sword
}
