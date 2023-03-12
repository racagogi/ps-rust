use std::fmt::Write;
use std::io;

fn main() {
    let temp = io::read_to_string(io::stdin()).unwrap();
    let mut input = temp.split_ascii_whitespace();
    let mut input_iter = || input.next().unwrap();

    let expr = input_iter().chars();
    let mut result = String::new();
    let mut stack = Vec::new();

    for c in expr {
        match c {
            '(' => {
                stack.push(c);
            }
            ')' => {
                while !stack.is_empty() && stack.last().unwrap() != &'(' {
                    write!(result, "{}", stack.pop().unwrap()).unwrap();
                }
                stack.pop();
            }
            '+' | '-' => {
                while !stack.is_empty() && stack.last().unwrap() != &'(' {
                    write!(result, "{}", stack.pop().unwrap()).unwrap();
                }
                stack.push(c);
            }
            '*' | '/' => {
                while !stack.is_empty()
                    && (stack.last().unwrap() == &'*' || stack.last().unwrap() == &'/')
                {
                    write!(result, "{}", stack.pop().unwrap()).unwrap();
                }
                stack.push(c);
            }
            _ => {
                write!(result, "{}", c).unwrap();
            }
        }
    }
    while !stack.is_empty() {
        write!(result, "{}", stack.pop().unwrap()).unwrap();
    }
    println!("{result}");
}
