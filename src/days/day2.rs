use std::{cmp::Ordering, convert::Infallible, ops::Not, str::FromStr};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

pub struct Turn(Move, Move);

impl Move {
    pub fn score(self) -> i32 {
        self as i32 + 1
    }
}

// cursed stuff
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
        Some(match (self, other) {
            (a, b) if *a == !*b => Greater,
            (a, b) if *b == !*a => Less,
            _ => Equal,
        })
    }
}

// even more cursed
impl Not for Move {
    type Output = Move;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

impl FromStr for Move {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            m => panic!("invalid move {m}"),
        })
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Turn> {
    let moves = input.split_whitespace().collect::<Vec<_>>();
    moves
        .chunks(2)
        .map(|ms| {
            Turn(
                Move::from_str(ms[0]).unwrap(),
                Move::from_str(ms[1]).unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Turn]) -> i32 {
    input
        .iter()
        .map(|Turn(opp, slf)| {
            slf.score()
                + match slf.partial_cmp(opp).unwrap() {
                    Ordering::Less => 0,
                    Ordering::Equal => 3,
                    Ordering::Greater => 6,
                }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Turn]) -> i32 {
    input
        .iter()
        .map(|Turn(opp, slf)| {
            let (slf, score) = match slf {
                Move::Rock => (!!*opp, 0),    // need to lose
                Move::Scissors => (!*opp, 6), // need to win
                Move::Paper => (*opp, 3),     // need to draw
            };
            slf.score() + score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"A Y
B X
C Z
"#;
        assert_eq!(part1(&parse(input)), 15);
    }

    #[test]
    fn part2_example() {
        let input = r#"A Y
B X
C Z
"#;
        assert_eq!(part2(&parse(input)), 12);
    }
}
