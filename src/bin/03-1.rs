use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.txt")?);

    let mut bits = [[0, 0]; 12]; // Hardcode cause I'm lazy
    for line in reader.lines() {
        let line = line?;
        for (i, digit) in line.chars().enumerate() {
            let digit = match digit {
                '0' => 0,
                '1' => 1,
                _ => continue,
            };
            bits[i][digit] += 1;
        }
    }

    let mut exp = 1;
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for digit in bits.into_iter().rev() {
        let gamma = if digit[0] > digit[1] { 0 } else { 1 };
        gamma_rate += gamma * exp;
        epsilon_rate += (1 - gamma) * exp;
        exp *= 2;
    }

    println!("gamma rate: {}", gamma_rate);
    println!("epsilon rate: {}", epsilon_rate);
    println!("answer: {}", gamma_rate * epsilon_rate);

    Ok(())
}
