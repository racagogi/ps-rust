use std::io;

fn main() {
    let temp = io::read_to_string(io::stdin()).unwrap();
    let mut input = temp.split_ascii_whitespace();
    let mut input_iter = || input.next().unwrap();

    let (n, e) = (
        input_iter().parse::<usize>().unwrap(),
        input_iter().parse::<usize>().unwrap(),
    );

    let graph = (0..e).fold(vec![Vec::new(); n + 1], |mut link, _| {
        let (x, y) = (
            input_iter().parse::<usize>().unwrap(),
            input_iter().parse::<usize>().unwrap(),
        );
        link[x].push(y);
        link[y].push(x);
        link
    });
    println!("{}", dfs(1, &graph));
}

fn dfs(start: usize, graph: &Vec<Vec<usize>>) -> i32 {
    let mut stack = Vec::new();
    let mut visited = vec![false; graph.len()];
    visited[start] = true;
    stack.push(start);
    let mut count = 0;
    while !stack.is_empty() {
        let s = stack.pop().unwrap();
        for &n in graph[s].iter() {
            match visited[n] {
                false => {
                    visited[n] = true;
                    stack.push(n);
                    count += 1;
                }
                true => continue,
            }
        }
    }
    count
}
