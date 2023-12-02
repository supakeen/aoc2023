use std::{collections::HashMap, io, io::Read, str};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut b = Vec::new();
    stdin.read_to_end(&mut b)?;

    let sum: u32 = as_numbers(str::from_utf8(&b).unwrap(), parse)
        .into_iter()
        .map(|p| format!("{}{}", p.0, p.1).parse::<u32>().unwrap())
        .sum();

    println!("{}", sum);

    Ok(())
}

fn as_numbers<P>(data: &str, parser: P) -> Vec<(u32, u32)>
where
    P: Fn(&str) -> (u32, u32),
{
    data.lines().map(parser).collect()
}

fn parse(line: &str) -> (u32, u32) {
    let replacements: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut numbers: Vec<u32> = vec![];

    for i in 0..line.len() {
        let r = str::from_utf8(&line.as_bytes()[i..]).unwrap();

        for (k, v) in &replacements {
            if r.starts_with(k) {
                numbers.push(*v);
            }
        }

        let c = line.chars().nth(i).unwrap();

        if ('0'..='9').contains(&c) {
            numbers.push(c.to_digit(10).unwrap());
        }
    }

    (
        numbers.first().copied().unwrap(),
        numbers.last().copied().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &'static str = concat!(
        "two1nine\n",
        "eightwothree\n",
        "abcone2threexyz\n",
        "xtwone3four\n",
        "4nineeightseven2\n",
        "zoneight234\n",
        "7pqrstsixteen\n",
    );

    #[test]
    fn test_as_numbers() {
        assert_eq!(as_numbers(EXAMPLE, parse).len(), 7);
        assert_eq!(
            as_numbers(EXAMPLE, parse),
            vec![(2, 9), (8, 3), (1, 3), (2, 4), (4, 2), (1, 4), (7, 6),]
        );
    }
}
