use std::io;

fn main() {
    loop {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).unwrap();
        if let Ok(i) = temp.trim().parse::<f64>() {
            if i == 0.0 {
                break;
            } else {
                println!("{:.2}", lves(i));
            }
        }
    }
}

fn lves(n: f64) -> f64 {
    1.0 + n + n * n + n * n * n + n * n * n * n
}
