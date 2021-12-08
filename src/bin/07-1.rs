use std::io::BufRead;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().lock().read_line(&mut buffer).unwrap();
    let input = buffer
        .split(",")
        .map(|num| num.trim().parse().unwrap())
        .collect();
    println!("{:?}", solve(input));
}

#[test]
fn test_example() {
    assert_eq!(solve(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37);
}

fn solve(input: Vec<i32>) -> i32 {
    let mut cost_min = i32::MAX;
    let pos_min = *input.iter().min().unwrap();
    let pos_max = *input.iter().max().unwrap();
    for pos in pos_min..=pos_max {
        let cost = cost(&input, pos);
        cost_min = cost_min.min(cost);
    }
    cost_min
}

fn cost(input: &[i32], target: i32) -> i32 {
    input.iter().map(|&pos| (pos - target).abs()).sum()
}
