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
    assert_eq!(solve(vec![3, 4, 3, 1, 2]), 5934);
}

#[test]
fn test() {
    solve(vec![0]);
}

fn solve(mut input: Vec<usize>) -> usize {
    for _ in 1..=80 {
        simulate_day(&mut input);
    }
    input.len()
}

fn simulate_day(state: &mut Vec<usize>) {
    let mut new_fishes = 0;
    for num in state.iter_mut() {
        if *num == 0 {
            *num = 6;
            new_fishes += 1;
        } else {
            *num -= 1;
        }
    }
    state.reserve(new_fishes);
    for _ in 0..new_fishes {
        state.push(8);
    }
}
