use link_ex_5::Polynomial;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("inputfile.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let pa_terms = parse_terms(lines.next().transpose()?.unwrap_or_default().as_str());
    let pb_terms = parse_terms(lines.next().transpose()?.unwrap_or_default().as_str());

    let pa = Polynomial::from_terms(&pa_terms);
    let pb = Polynomial::from_terms(&pb_terms);
    let sum = pa.add(&pb);

    println!("PA = {}", pa.format());
    println!("PB = {}", pb.format());
    println!("和多项式 = {}", sum.format());

    Ok(())
}

fn parse_terms(line: &str) -> Vec<(i32, f32)> {
    let mut nums = line.split_whitespace();
    let n = nums
        .next()
        .and_then(|item| item.parse::<usize>().ok())
        .unwrap_or(0);
    let mut terms = Vec::new();

    for _ in 0..n {
        let exp = nums.next().and_then(|item| item.parse::<i32>().ok());
        let coef = nums.next().and_then(|item| item.parse::<f32>().ok());
        if let (Some(exp), Some(coef)) = (exp, coef) {
            terms.push((exp, coef));
        }
    }

    terms
}
