use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.txt")?);

    let mut distance = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let line = line?;
        let mut words = line.split(" ");
        let command = words.next().unwrap();
        let dist: i32 = words.next().unwrap().parse().unwrap();
        match command {
            "forward" => distance += dist,
            "down" => depth += dist,
            "up" => depth -= dist,
            _ => (),
        }
    }

    println!("distance: {}", distance);
    println!("depth: {}", depth);
    println!("area: {}", distance * depth);

    Ok(())
}
