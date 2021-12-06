use std::io::BufRead;
use std::ops::Sub;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{}", count_incs(&input));
}

fn count_incs(input: &Vec<i32>) -> usize {
    let pairs = input.iter().zip(input.iter().skip(1));
    let deltas = pairs.map(|(a, b)| b.sub(a));
    deltas.filter(|x| x.is_positive()).count()
}
