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
    let mut nodes: Vec<usize> = (1..=n).collect();
    let mut cc = 0;
    while !nodes.is_empty() {
        cc += 1;
        let start = nodes.pop().unwrap();
        let visited = dfs(start, &graph);
        for i in visited {
            nodes.retain(|x| i != *x);
        }
    }

    println!("{cc}");
}

fn dfs(start: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut stack = Vec::new();
    let mut visited = vec![false; graph.len()];
    let mut result = Vec::new();
    result.push(start);
    visited[start] = true;
    stack.push(start);
    while !stack.is_empty() {
        let s = stack.pop().unwrap();
        for &n in graph[s].iter() {
            match visited[n] {
                false => {
                    visited[n] = true;
                    result.push(n);
                    stack.push(n);
                }
                true => continue,
            }
        }
    }
    result
}
