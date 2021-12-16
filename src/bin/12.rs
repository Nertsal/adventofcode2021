use std::{collections::HashMap, io::BufRead};

fn main() {
    let stdin = std::io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap());
    let input = parse(input);
    println!("part 1: {:?}", solve(input.clone(), false));
    println!("part 2: {:?}", solve(input, true));
}

#[test]
fn test_example1() {
    let input = parse(
        "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            .lines(),
    );
    assert_eq!(solve(input.clone(), false), 10);
    assert_eq!(solve(input, true), 36);
}

#[test]
fn test_example2() {
    let input = parse(
        "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc"
            .lines(),
    );
    assert_eq!(solve(input.clone(), false), 19);
    assert_eq!(solve(input, true), 103);
}

#[test]
fn test_example3() {
    let input = parse(
        "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW"
            .lines(),
    );
    assert_eq!(solve(input.clone(), false), 226);
    assert_eq!(solve(input, true), 3509);
}

fn parse(input: impl IntoIterator<Item = impl AsRef<str>>) -> (Caves, Connections) {
    let mut caves = Caves::new();
    let mut connections = Connections::new();
    for line in input {
        let mut words = line.as_ref().trim().split("-");
        let from = words.next().unwrap().to_owned();
        let cave = Cave::from_name(&from);
        caves.insert(from.clone(), cave);
        let to = words.next().unwrap().to_owned();
        let cave = Cave::from_name(&to);
        caves.insert(to.clone(), cave);
        connections.push((from, to));
    }
    (caves, connections)
}

#[derive(Clone)]
enum Cave {
    Big,
    Small,
    Start,
}

impl Cave {
    pub fn from_name(name: &str) -> Self {
        match name {
            "start" => Self::Start,
            name => {
                if name.chars().nth(0).unwrap().is_lowercase() {
                    Self::Small
                } else {
                    Self::Big
                }
            }
        }
    }
}

type CaveId = String;
type Caves = HashMap<CaveId, Cave>;
type Connections = Vec<(CaveId, CaveId)>;

fn solve(input: (Caves, Connections), part2: bool) -> usize {
    paths(
        &"start".to_owned(),
        &"end".to_owned(),
        &input.0,
        &input.1,
        &mut vec!["start".to_owned()],
        part2,
    )
}

fn paths(
    from: &CaveId,
    to: &CaveId,
    caves: &Caves,
    connections: &Connections,
    visited: &mut Vec<CaveId>,
    can_visit_twice: bool,
) -> usize {
    let mut path_count = 0;

    for connected in connections.iter().filter_map(|(a, b)| match from {
        from if from == a => Some(b),
        from if from == b => Some(a),
        _ => None,
    }) {
        let visited_small_twice = match caves.get(connected).unwrap() {
            Cave::Start => continue,
            Cave::Small if visited.contains(connected) => {
                if !can_visit_twice {
                    continue;
                }
                true
            }
            _ => false,
        };
        visited.push(connected.to_owned());

        if connected == to {
            path_count += 1;
        } else {
            path_count += paths(
                connected,
                to,
                caves,
                connections,
                visited,
                can_visit_twice && !visited_small_twice,
            );
        }

        visited.pop();
    }

    path_count
}
