use std::{io, io::Read, str};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut b = Vec::new();
    stdin.read_to_end(&mut b)?;

    let sum: u32 = as_numbers(str::from_utf8(&b).unwrap(), parse)
        .into_iter()
        .map(|p| p.0 * 10 + p.1)
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
    let numbers: Vec<u32> = line
        .chars()
        .filter(|c| ('0'..='9').contains(c))
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    (
        numbers.first().copied().unwrap(),
        numbers.last().copied().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &'static str =
        concat!("1abc2\n", "pqr3stu8vwx\n", "a1b2c3d4e5f\n", "treb7uchet\n",);

    #[test]
    fn test_as_numbers() {
        assert_eq!(as_numbers(EXAMPLE, parse).len(), 4);
        assert_eq!(
            as_numbers(EXAMPLE, parse),
            vec![(1, 2), (3, 8), (1, 5), (7, 7),]
        );
    }
}
