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
    assert_eq!(answer, 1134);
}

type HeightMap<T> = Vec<Vec<T>>;

fn parse(input: impl IntoIterator<Item = impl AsRef<str>>) -> HeightMap<u32> {
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

fn solve(input: HeightMap<u32>) -> u32 {
    let mut basins = vec![vec![None; input[0].len()]; input.len()];
    let mut basin_count = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if fill_basin(x, y, basin_count, &input, &mut basins) {
                basin_count += 1;
            }
        }
    }

    let mut basin_sizes = vec![0; basin_count];
    for basin_number in basins
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter_map(|x| x)
    {
        basin_sizes[basin_number] += 1;
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes.into_iter().take(3).product()
}

fn fill_basin(
    x: usize,
    y: usize,
    basin_number: usize,
    map: &HeightMap<u32>,
    basins: &mut HeightMap<Option<usize>>,
) -> bool {
    let height = map[y][x];
    if height == 9 || basins[y][x].is_some() {
        return false;
    }

    basins[y][x] = Some(basin_number);
    for (nx, ny, _) in neighbours(x, y, map)
        .filter(|&(x, y, &height)| height < 9 && basins[y][x].is_none())
        .collect::<Vec<_>>()
    {
        fill_basin(nx, ny, basin_number, map, basins);
    }

    true
}

const NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn neighbours<'a, T>(
    x: usize,
    y: usize,
    map: &'a HeightMap<T>,
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
