use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap());
    let input = parse(input);
    let answer = solve(input);
    println!("{:?}", answer);
}

#[test]
fn test_example() {
    let answer = solve(parse(
        "2199943210
    3987894921
    9856789892
    8767896789
    9899965678"
            .lines(),
    ));
    assert_eq!(answer, 15);
}

type HeightMap = Vec<Vec<u32>>;

fn parse(input: impl IntoIterator<Item = impl AsRef<str>>) -> HeightMap {
    input
        .into_iter()
        .map(|line| {
            line.as_ref()
                .trim()
                .chars()
                .map(|height| height.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn solve(input: HeightMap) -> u32 {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &height)| (x, y, height))
        })
        .filter(|&(x, y, height)| {
            NEIGHBOURS
                .iter()
                .filter_map(|&(dx, dy)| {
                    let (x, y) = (x as isize + dx, y as isize + dy);
                    if x < 0 || y < 0 {
                        None
                    } else {
                        let (x, y) = (x as usize, y as usize);
                        input.get(y).and_then(|row| row.get(x)).copied()
                    }
                })
                .all(|neighbour| height < neighbour)
        })
        .map(|(_, _, height)| height + 1)
        .sum()
}

const NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
