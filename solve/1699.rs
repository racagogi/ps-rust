use std::{cmp::min, io::stdin};

fn main() {
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    let n = i32::from_str_radix(temp.trim(), 10).unwrap();
    let squrtv: Vec<i32> = (1..317).map(|x| x * x).collect();
    let mut dp: Vec<i32> = vec![4; 100001];
    for i in 0..100001 as usize {
        if squrtv.contains(&(i as i32)) {
            dp[i] = 1;
        } else {
            for x in 0..(i as f64).sqrt() as usize {
                dp[i] = min(dp[i], dp[i - squrtv[x] as usize] + 1);
            }
        }
    }
    println!("{}", dp[n as usize]);
}
