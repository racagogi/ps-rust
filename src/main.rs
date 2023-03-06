use std::io::stdin;

fn main() {
    println!("{}",sign(read_num(),0));
    println!("{}",sign(read_num(),0));
    println!("{}",sign(read_num(),0));
}

fn read_num() -> i128 {
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    temp.strip_suffix("\n").unwrap().parse::<i128>().unwrap()
}

fn sign(count: i128, acc: i128) -> String {
    if count > 0 {
        sign(count - 1, acc + read_num())
    } else {
        match acc.cmp(&0) {
            std::cmp::Ordering::Less => String::from("-"),
            std::cmp::Ordering::Equal => String::from("0"),
            std::cmp::Ordering::Greater => String::from("+"),
        }
    }
}
