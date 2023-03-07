use std::io::stdin;

fn main() {
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    let char_vec: Vec<char> = temp.trim().chars().collect();
    if char_vec.len() == 1
        || temp.trim() == "c="
        || temp.trim() == "c-"
        || temp.trim() == "d-"
        || temp.trim() == "lj"
        || temp.trim() == "nj"
        || temp.trim() == "s="
        || temp.trim() == "z="
    {
        println!("1");
    } else if char_vec.len() == 2 {
        println!("2");
    } else {
        println!("{}", parse_croatia(char_vec, 0));
    }
}

fn parse_croatia(word: Vec<char>, acc: i32) -> i32 {
    if word.len() == 0 {
        acc
    } else if word.len() == 1 {
        acc + 1
    } else {
        let two_chars: String = word[0..2].into_iter().collect();
        match two_chars.as_ref() {
            "c=" | "c-" | "d-" | "lj" | "nj" | "s=" | "z=" => {
                if word.len() > 2 {
                    parse_croatia(word[2..].to_vec(), acc + 1)
                } else {
                    acc + 1
                }
            }
            "dz" => {
                if word[2] == '=' {
                    if word.len() > 3 {
                        parse_croatia(word[3..].to_vec(), acc + 1)
                    } else {
                        acc + 1
                    }
                } else {
                    parse_croatia(word[1..].to_vec(), acc + 1)
                }
            }
            _ => parse_croatia(word[1..].to_vec(), acc + 1),
        }
    }
}
