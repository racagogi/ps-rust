use std::io::stdin;

fn main() {
    for i in 0..read_i32()[0] {
        cal_over_mean_radio(&read_f32());
    }
}

fn read_i32() -> Vec<i32> {
    let mut temp = String::new();
    stdin().read_line(&mut temp);
    temp.trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn read_f32() -> Vec<f32> {
    let mut temp = String::new();
    stdin().read_line(&mut temp);
    temp.trim()
        .split(" ")
        .map(|x| x.parse::<f32>().unwrap())
        .collect()
}

fn cal_over_mean_radio(l: &Vec<f32>) -> () {
    let mut sum = 0.0;
    let mut mean = 0.0;
    let mut over_mean = 0.0;
    for i in l[1..].iter() {
        sum += i;
    }
    mean = sum as f32 / l[0];
    for i in l[1..].iter() {
        if i > &mean {
            over_mean += 1.0;
        }
    }
    println!("{:.3}%", over_mean / l[0] * 100.0)
}
