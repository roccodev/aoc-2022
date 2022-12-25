use std::{convert::Infallible, fmt::Display, str::FromStr};

struct Number(i64);

impl FromStr for Number {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .rev()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '-' => -1,
                    '=' => -2,
                    c => panic!("invalid char {c}"),
                })
                .fold((0, 1), |(sum, pow), next| (sum + next * pow, pow * 5))
                .0,
        ))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let highest_pow = (self.0 as f64).log(5.0).floor() as u32;
        let mut digits = Vec::with_capacity(highest_pow as usize + 1);
        let mut highest_pow = 5i64.pow(highest_pow);
        let mut n = self.0;
        while highest_pow >= 1 {
            digits.push(n / highest_pow);
            n %= highest_pow;
            highest_pow /= 5;
        }
        for i in (0..digits.len()).rev() {
            if digits[i] > 2 {
                digits[i] -= 5;
                digits[i - 1] += 1;
            }
        }
        digits.reverse();
        write!(
            f,
            "{}",
            digits
                .into_iter()
                .rev()
                .map(|d| match d {
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    -1 => '-',
                    -2 => '=',
                    d => panic!("invalid digit {d}"),
                })
                .collect::<String>()
        )
    }
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(ToString::to_string).collect()
}

#[aoc(day25, part1)]
pub fn part1(input: &[String]) -> String {
    let sum = Number(
        input
            .iter()
            .map(|s| Number::from_str(s).unwrap())
            .map(|n| n.0)
            .sum(),
    );
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"#;
        assert_eq!(part1(&parse(input)), "3");
    }
}
