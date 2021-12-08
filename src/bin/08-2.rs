use std::{collections::HashMap, io::BufRead};

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());
    let input = parse(lines);
    println!("{:?}", solve(input));
}

fn parse<'a>(input: impl Iterator<Item = impl AsRef<str>>) -> Vec<Entry> {
    let mut result = Vec::new();
    for line in input {
        let mut entry = line.as_ref().split('|');
        let patterns = entry
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|pattern| {
                pattern
                    .trim()
                    .chars()
                    .map(|char| parse_segment(char).unwrap())
                    .collect()
            })
            .collect();
        let output = entry
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|digit| {
                digit
                    .trim()
                    .chars()
                    .map(|char| parse_segment(char).unwrap())
                    .collect()
            })
            .collect();
        result.push(Entry { patterns, output });
    }
    result
}

fn parse_segment(segment: char) -> Option<Segment> {
    match segment {
        'a' => Some(Segment::A),
        'b' => Some(Segment::B),
        'c' => Some(Segment::C),
        'd' => Some(Segment::D),
        'e' => Some(Segment::E),
        'f' => Some(Segment::F),
        'g' => Some(Segment::G),
        _ => None,
    }
}

#[test]
fn test_example() {
    let input = parse(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            .lines(),
    );
    assert_eq!(solve(input), 61229);
}

#[test]
fn test_simple() {
    let input = parse(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
            .lines(),
    );
    assert_eq!(solve(input), 5353);
}

#[test]
fn test_option() {
    assert_eq!(
        options(vec![vec![1, 2], vec![3, 4]]),
        vec![vec![3, 1], vec![4, 1], vec![3, 2], vec![4, 2]]
    );
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Segment {
    fn one() -> Vec<Self> {
        use Segment::*;
        vec![C, F]
    }

    fn zero() -> Vec<Self> {
        use Segment::*;
        vec![A, B, C, E, F, G]
    }

    fn two() -> Vec<Self> {
        use Segment::*;
        vec![A, C, D, E, G]
    }

    fn three() -> Vec<Self> {
        use Segment::*;
        vec![A, C, D, F, G]
    }

    fn four() -> Vec<Self> {
        use Segment::*;
        vec![B, C, D, F]
    }

    fn five() -> Vec<Self> {
        use Segment::*;
        vec![A, B, D, F, G]
    }

    fn six() -> Vec<Self> {
        use Segment::*;
        vec![A, B, D, E, F, G]
    }

    fn seven() -> Vec<Self> {
        use Segment::*;
        vec![A, C, F]
    }

    fn eight() -> Vec<Self> {
        use Segment::*;
        vec![A, B, C, D, E, F, G]
    }

    fn nine() -> Vec<Self> {
        use Segment::*;
        vec![A, B, C, D, F, G]
    }
}

type Pattern = Vec<Segment>;

struct Entry {
    patterns: Vec<Pattern>,
    output: Vec<Pattern>,
}

fn solve(entries: Vec<Entry>) -> u32 {
    let mut result = 0;
    for entry in entries {
        let mut bindings = Bindings::default();
        let patterns = entry.patterns.iter().chain(entry.output.iter());
        infer(patterns, &mut bindings);

        let mut exp = 1;
        let mut value = 0;
        for output in entry.output.iter().rev() {
            let digit = deduce(output, &bindings);
            value += digit * exp;
            exp *= 10;
        }
        result += value;
    }
    result
}

#[derive(Debug)]
struct Bindings(HashMap<Segment, Vec<Segment>>);

impl Default for Bindings {
    fn default() -> Self {
        let mut binds = HashMap::with_capacity(7);
        for segment in Segment::eight() {
            binds.insert(segment, Segment::eight());
        }
        Self(binds)
    }
}

fn deduce(pattern: &[Segment], bindings: &Bindings) -> u32 {
    match pattern.len() {
        // Digit 1
        2 => 1,
        // Digit 4
        4 => 4,
        // Digit 7
        3 => 7,
        // Digit 8
        7 => 8,
        // Digits 2, 3, 5
        5 => {
            let options = gen_options(pattern, bindings);
            if fits(&options, Segment::two()) {
                2
            } else if fits(&options, Segment::three()) {
                3
            } else if fits(&options, Segment::five()) {
                5
            } else {
                unreachable!()
            }
        }
        // Digits 0, 6, 9
        6 => {
            let options = gen_options(pattern, bindings);
            if fits(&options, Segment::zero()) {
                0
            } else if fits(&options, Segment::six()) {
                6
            } else if fits(&options, Segment::nine()) {
                9
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
}

fn fits(options: &Vec<Pattern>, pattern: Pattern) -> bool {
    options
        .iter()
        .any(|option| pattern.iter().all(|segment| option.contains(segment)))
}

fn gen_options(pattern: &[Segment], bindings: &Bindings) -> Vec<Vec<Segment>> {
    let binds: Vec<_> = pattern
        .iter()
        .map(|segment| bindings.0.get(segment).unwrap().clone())
        .collect();
    options(binds)
}

fn options<T: Clone>(binds: impl IntoIterator<Item = Vec<T>>) -> Vec<Vec<T>> {
    let mut binds = binds.into_iter();
    match binds.next() {
        None => vec![vec![]],
        Some(head) => {
            let tails = options(binds);
            head.into_iter()
                .flat_map(|option| {
                    tails.iter().map(move |tail| {
                        let mut tail = tail.clone();
                        tail.push(option.clone());
                        tail
                    })
                })
                .collect()
        }
    }
}

fn infer<'a>(patterns: impl Iterator<Item = &'a Pattern>, bindings: &mut Bindings) {
    for pattern in patterns {
        match pattern.len() {
            // Digit 1
            2 => constraint(pattern, &Segment::one(), bindings),
            // Digit 4
            4 => constraint(pattern, &Segment::four(), bindings),
            // Digit 7
            3 => constraint(pattern, &Segment::seven(), bindings),
            _ => (),
        }
    }
}

fn constraint(pattern: &[Segment], target: &[Segment], bindings: &mut Bindings) {
    for segment in pattern {
        bindings
            .0
            .get_mut(segment)
            .unwrap()
            .retain(|segment| target.contains(segment))
    }
    for segment in Segment::eight()
        .iter()
        .filter(|segment| !pattern.contains(segment))
    {
        bindings
            .0
            .get_mut(segment)
            .unwrap()
            .retain(|segment| !target.contains(segment))
    }
}
