use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.txt")?);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    let oxygen = &{
        let mut index = 0;
        let mut lines = lines.clone();
        while lines.len() > 1 {
            let bits = count_bits(&lines, index);
            let filter = if bits[0] > bits[1] { '0' } else { '1' };
            lines.retain(|line| line.chars().nth(index).unwrap() == filter);
            index += 1;
        }
        lines
    }[0];
    let oxygen = i32::from_str_radix(oxygen, 2).unwrap();

    let scrubber = &{
        let mut index = 0;
        let mut lines = lines.clone();
        while lines.len() > 1 {
            let bits = count_bits(&lines, index);
            let filter = if bits[0] <= bits[1] { '0' } else { '1' };
            lines.retain(|line| line.chars().nth(index).unwrap() == filter);
            index += 1;
        }
        lines
    }[0];
    let scrubber = i32::from_str_radix(scrubber, 2).unwrap();

    println!("oxygen generator rating: {}", oxygen);
    println!("CO2 scrubber rating: {}", scrubber);
    println!("lif support rating: {}", oxygen * scrubber);

    Ok(())
}

fn count_bits<'a>(lines: impl IntoIterator<Item = &'a String>, index: usize) -> [usize; 2] {
    let mut bits = [0; 2];
    for line in lines {
        match line.chars().nth(index).unwrap() {
            '0' => bits[0] += 1,
            '1' => bits[1] += 1,
            _ => (),
        }
    }
    bits
}
