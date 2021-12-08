use std::io::BufRead;

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
            .split(' ')
            .map(|pattern| pattern.trim().to_owned())
            .collect();
        let output = entry
            .next()
            .unwrap()
            .split(' ')
            .map(|digit| digit.trim().to_owned())
            .collect();
        result.push(Entry { patterns, output });
    }
    result
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
    assert_eq!(solve(input), 26);
}

struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn solve(entries: Vec<Entry>) -> usize {
    entries
        .iter()
        .flat_map(|entry| entry.output.iter().map(|output| output.len()))
        .filter(|&output| matches!(output, 2 | 4 | 3 | 7))
        .count()
}
