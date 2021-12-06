use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.txt")?);
    let mut lines = reader.lines();

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    lines.next();

    let mut boards = Vec::new();
    while let Some(Ok(first_line)) = lines.next() {
        let mut board = Board { board: Vec::new() };
        board.parse_line(&first_line);
        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }
            board.parse_line(&line);
        }
        boards.push(board);
    }

    let (number, board_index) = (|| {
        for number in numbers {
            for (i, board) in boards.iter_mut().enumerate() {
                if board.select_number(number) {
                    return (number, i);
                }
            }
        }
        unreachable!()
    })();

    let sum: u32 = boards[board_index]
        .board
        .iter()
        .flat_map(|row| {
            row.iter()
                .filter(|cell| !cell.selected)
                .map(|cell| cell.value)
        })
        .sum();

    println!("num: {}", number);
    println!("sum: {}", sum);
    println!("ans: {}", sum * number);

    Ok(())
}

struct Board {
    board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn parse_line(&mut self, line: &str) {
        let line = line
            .split_whitespace()
            .map(|num| Cell {
                value: num.parse().unwrap(),
                selected: false,
            })
            .collect();
        self.board.push(line);
    }

    pub fn select_number(&mut self, number: u32) -> bool {
        let mut cols = Vec::new();
        for _ in 0..self.board.first().unwrap().len() {
            cols.push(true);
        }

        let mut win = false;
        for row in &mut self.board {
            let mut row_win = true;
            for (i, cell) in row.iter_mut().enumerate() {
                if cell.value == number {
                    cell.selected = true;
                }
                if !cell.selected {
                    cols[i] = false;
                    row_win = false;
                }
            }
            if row_win {
                win = true;
            }
        }

        win || cols.into_iter().any(|win| win)
    }
}

struct Cell {
    value: u32,
    selected: bool,
}
