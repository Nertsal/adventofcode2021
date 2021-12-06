use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.txt")?);

    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line?;
        let mut words = line.split(" ");
        let command = words.next().unwrap();
        let value: i32 = words.next().unwrap().parse().unwrap();
        match command {
            "forward" => {
                position += value;
                depth += value * aim;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => (),
        }
    }

    println!("position: {}", position);
    println!("depth: {}", depth);
    println!("area: {}", position * depth);

    Ok(())
}
