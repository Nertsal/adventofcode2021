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
    assert_eq!(solve(vec![3, 4, 3, 1, 2]), 26984457539);
}

fn solve(input: Vec<usize>) -> u64 {
    let mut state = [0; 7];
    let mut delayed = [0; 7];
    let mut fishes = input.len() as u64;
    input
        .into_iter()
        .for_each(|fish| state[(fish + 1) % 7] += u64::from(1u64));

    for day in 1..=256 {
        let new = state[day % 7];
        fishes += new;
        let rem = (day + 2) % 7;
        state[rem] += std::mem::take(&mut delayed[rem]);
        delayed[rem] += new;
    }

    fishes
}
