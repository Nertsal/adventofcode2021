use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.txt")?);

    let mut ocean_floor = OceanFloor::default();

    for line in reader.lines() {
        let line = line?;
        let mut points = line.split("->").map(|point| {
            let mut coords = point.split(",").map(|coord| coord.trim().parse().unwrap());
            (coords.next().unwrap(), coords.next().unwrap())
        });
        let start = points.next().unwrap();
        let end = points.next().unwrap();
        assert!(points.next().is_none());

        ocean_floor.mark_line(start, end);
    }

    // println!("-----");
    // for row in &ocean_floor.map {
    //     let mut line = String::new();
    //     for &mark in row {
    //         if mark == 0 {
    //             line.push_str(".");
    //         } else {
    //             line.push_str(&mark.to_string());
    //         }
    //     }
    //     println!("{}", line);
    // }
    // println!("-----");

    let overlaps = ocean_floor
        .map
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&x| x > 1)
        .count();
    println!("answer: {}", overlaps);

    Ok(())
}

#[derive(Debug, Default)]
struct OceanFloor {
    map: Vec<Vec<usize>>,
}

impl OceanFloor {
    pub fn mark_line(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;
        let delta = dx.abs().max(dy.abs());
        let dx = dx.signum();
        let dy = dy.signum();

        for d in 0..=delta {
            let x = (x1 as isize + d * dx) as usize;
            let y = (y1 as isize + d * dy) as usize;
            self.mark(x, y);
        }
    }

    fn mark(&mut self, x: usize, y: usize) {
        // Add new lines if neccessary
        for _ in self.map.len()..=y {
            self.map.push(Vec::new());
        }

        let row = &mut self.map[y];
        // Add new columns if neccessary
        for _ in row.len()..=x {
            row.push(0);
        }

        // Mark position
        row[x] += 1;
    }
}
