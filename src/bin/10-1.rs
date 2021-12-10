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
    assert_eq!(solve(input), 288957);
}

fn solve(input: Vec<impl AsRef<str>>) -> u64 {
    let mut scores = Vec::new();
    for line in &input {
        match check_line(line.as_ref()).map(|stack| autocomplete(stack)) {
            Ok(completion) => {
                let mut line_score = 0;
                for score in completion
                    .into_iter()
                    .map(|close| bracket_score(close).unwrap())
                {
                    line_score = line_score * 5 + score;
                }
                scores.push(line_score);
            }
            Err(_) => (),
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn autocomplete(stack: Vec<char>) -> Vec<char> {
    stack
        .into_iter()
        .rev()
        .map(|open| closed_bracket(open).unwrap())
        .collect()
}

fn check_line(line: &str) -> Result<Vec<char>, ()> {
    let mut chunk_stack = Vec::new();
    for character in line.chars() {
        match closed_bracket(character) {
            Some(_) => {
                chunk_stack.push(character);
            }
            None => match bracket_score(character) {
                Some(_) => match chunk_stack.pop() {
                    Some(open) if closed_bracket(open).unwrap() == character => (),
                    _ => return Err(()),
                },
                _ => (),
            },
        }
    }
    Ok(chunk_stack)
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

fn bracket_score(bracket: char) -> Option<u64> {
    match bracket {
        ')' => Some(1),
        ']' => Some(2),
        '}' => Some(3),
        '>' => Some(4),
        _ => None,
    }
}
