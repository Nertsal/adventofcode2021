use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let input = parse(stdin.lock().lines().map(|line| line.unwrap()));
    let answer = solve_1(input.clone());
    println!("1) {:?}", answer);
    let answer = solve_2(input);
    println!("2) {:?}", answer);
}

#[test]
fn test_example() {
    let input = parse(
        "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526"
            .lines(),
    );
    assert_eq!(solve_1(input.clone()), 1656);
    assert_eq!(solve_2(input), 195);
}

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

type Map<T> = Vec<Vec<T>>;

fn parse(input: impl IntoIterator<Item = impl AsRef<str>>) -> Map<u32> {
    let map: Map<_> = input
        .into_iter()
        .map(|row| {
            let row: Vec<_> = row
                .as_ref()
                .trim()
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect();
            assert_eq!(row.len(), WIDTH);
            row
        })
        .collect();
    assert_eq!(map.len(), HEIGHT);
    map
}

fn solve_1(mut input: Map<u32>) -> usize {
    (0..100).map(|_| step(&mut input)).sum()
}

fn solve_2(mut input: Map<u32>) -> usize {
    (1..)
        .map(|i| (i, step(&mut input)))
        .find(|(_, flashes)| *flashes == WIDTH * HEIGHT)
        .map(|(i, _)| i)
        .unwrap()
}

fn step(map: &mut Map<u32>) -> usize {
    let mut flashed = vec![vec![false; WIDTH]; HEIGHT];

    let mut flash_queue: Vec<_> = charge(
        map,
        (0..WIDTH).flat_map(|x| (0..HEIGHT).map(move |y| (x, y))),
    )
    .collect();

    while let Some((x, y)) = flash_queue.pop() {
        if flashed[y][x] {
            continue;
        }

        flashed[y][x] = true;
        map[y][x] = 0;
        let neighbours: Vec<_> = neighbours(x, y, map)
            .map(|(x, y, _)| (x, y))
            .filter(|&(x, y)| !flashed[y][x])
            .collect();
        flash_queue.extend(charge(map, neighbours));
    }

    flashed
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|flashed| *flashed)
        .count()
}

fn charge<'a>(
    map: &'a mut Map<u32>,
    charge: impl IntoIterator<Item = (usize, usize)> + 'a,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    charge
        .into_iter()
        .map(|(x, y)| {
            map[y][x] += 1;
            (x, y, map[y][x])
        })
        .filter(|(_, _, charge)| *charge > 9)
        .map(|(x, y, _)| (x, y))
}

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn neighbours<'a, T>(
    x: usize,
    y: usize,
    map: &'a Map<T>,
) -> impl Iterator<Item = (usize, usize, &'a T)> + 'a {
    NEIGHBOURS.iter().filter_map(move |&(dx, dy)| {
        let (x, y) = (x as isize + dx, y as isize + dy);
        if x < 0 || y < 0 {
            None
        } else {
            let (x, y) = (x as usize, y as usize);
            map.get(y)
                .and_then(|row| row.get(x))
                .map(|height| (x, y, height))
        }
    })
}
