use std::io::BufRead;
use std::ops::{Add, Sub};

fn main() {
    let input: Vec<i32> = input()
        .unwrap()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let input = triple_sums(&input).collect();
    println!("{}", count_incs(&input));
}

fn input() -> std::io::Result<impl Iterator<Item = std::io::Result<String>>> {
    let input_file = std::fs::File::open("input.txt")?;
    let reader = std::io::BufReader::new(input_file);
    Ok(reader.lines())
}

fn triple_sums<'a>(input: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map(|((a, b), c)| a.add(b).add(c))
}

fn count_incs(input: &Vec<i32>) -> usize {
    let pairs = input.iter().zip(input.iter().skip(1));
    let deltas = pairs.map(|(a, b)| b.sub(a));
    deltas.filter(|x| x.is_positive()).count()
}
