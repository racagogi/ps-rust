use std::io::stdin;

fn main() {
    let mut grade = 0.0;
    let mut gpa = 0.0;
    for i in 0..20 {
        let temp = grade_parse();
        grade += temp.0;
        gpa += temp.1;
    }
    println!("{}", gpa / grade);
}

fn grade_parse() -> (f32, f32) {
    let mut temp = String::new();
    stdin().read_line(&mut temp);
    let words: Vec<&str> = temp.trim().split(" ").collect();
    let mut grade: f32 = 0.0;
    let mut score: f32 = 0.0;
    match words[2] {
        "A+" => {
            grade = words[1].parse::<f32>().unwrap();
            score = 4.5;
        }
        "A0" => {
            grade = words[1].parse::<f32>().unwrap();
            score = 4.0;
        }
        "B+" => {
            grade = words[1].parse::<f32>().unwrap();
            score = 3.5;
        }
        "B0" => {
            grade = words[1].parse::<f32>().unwrap();
            score = 3.0;
        }
        "C+" => {
            grade = words[1].parse::<f32>().unwrap();
            score = 2.5;
        }
        "C0" => {
            grade = words[1].parse::<f32>().unwrap();
            score = 2.0;
        }
        "D+" => {
            grade = words[1].parse::<f32>().unwrap();
            score = 1.5;
        }
        "D0" => {
            grade = words[1].parse::<f32>().unwrap();
            score = 1.0;
        }
        "F" => {
            grade = words[1].parse::<f32>().unwrap();
            score = 0.0;
        }
        _ => {
            grade = 0.0;
            score = 4.5;
        }
    };
    (grade, score * grade)
}
