use std::ops::RangeInclusive;

use regex::Regex;

#[derive(Clone)]
pub struct ElfPair(RangeInclusive<u32>, RangeInclusive<u32>);

trait RangeContains {
    fn is_superset(&self, other: &Self) -> bool;
    fn is_overlapping(&self, other: &Self) -> bool;
}

impl<N: Ord> RangeContains for RangeInclusive<N> {
    #[inline]
    fn is_superset(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    #[inline]
    fn is_overlapping(&self, other: &Self) -> bool {
        let (slf_a, slf_b, oth_a, oth_b) = (self.start(), self.end(), other.start(), other.end());

        if slf_b == oth_a || oth_b == slf_a {
            return true;
        }

        if slf_a < oth_a {
            return slf_b >= oth_a;
        }

        oth_b >= slf_a
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<ElfPair> {
    let regex = Regex::new(r#"(\d+)-(\d+),(\d+)-(\d+)"#).unwrap();
    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let (a1, a2, b1, b2) = (&captures[1], &captures[2], &captures[3], &captures[4]);
            ElfPair(
                a1.parse().unwrap()..=a2.parse().unwrap(),
                b1.parse().unwrap()..=b2.parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[ElfPair]) -> usize {
    input
        .iter()
        .filter(|ElfPair(a, b)| a.is_superset(b) || b.is_superset(a))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[ElfPair]) -> usize {
    #[cfg(debug_assertions)]
    let overlap_naive = input
        .iter()
        .cloned()
        .filter(|ElfPair(a, b)| {
            b.clone().any(|x| a.contains(&x)) || a.clone().any(|x| b.contains(&x))
        })
        .count();

    let with_range = input
        .iter()
        .filter(|ElfPair(a, b)| a.is_overlapping(b))
        .count();

    #[cfg(debug_assertions)]
    assert_eq!(overlap_naive, with_range);

    with_range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(part1(&parse(input)), 2);
    }

    #[test]
    fn part2_example() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
1-4,3-7
7-9,1-8
7-8,1-9
6-7,1-8
28-79,27-27"#;
        assert_eq!(part2(&parse(input)), 8);
    }
}
