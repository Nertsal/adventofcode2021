use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let input: Vec<_> = stdin.lock().lines().map(|line| line.unwrap()).collect();
    let answer = solve(input);
    println!("{:?}", answer);
}

#[test]
fn test_example() {
    let input: Vec<_> = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]"
        .lines()
        .collect();
    assert_eq!(solve(input), 26397);
}

fn solve(input: Vec<impl AsRef<str>>) -> u32 {
    let mut total_score = 0;
    for line in &input {
        let score = check_line(line.as_ref());
        total_score += score;
    }
    total_score
}

fn check_line(line: &str) -> u32 {
    let mut chunk_stack = Vec::new();
    for character in line.chars() {
        match closed_bracket(character) {
            Some(closed) => {
                chunk_stack.push(closed);
            }
            None => match bracket_score(character) {
                Some(score) => match chunk_stack.pop() {
                    Some(open) if open == character => (),
                    _ => return score,
                },
                _ => (),
            },
        }
    }
    0
}

fn closed_bracket(bracket: char) -> Option<char> {
    match bracket {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn bracket_score(bracket: char) -> Option<u32> {
    match bracket {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}
