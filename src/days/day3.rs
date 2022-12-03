use std::ops::BitAnd;

struct BitSet(u64);

impl BitSet {
    pub fn reduce_sum(&self) -> u32 {
        (0u32..64)
            .map(|i| i * ((self.0 & (1 << i)) != 0) as u8 as u32)
            .sum()
    }
}

impl FromIterator<u32> for BitSet {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut bits = 0;
        for byte in iter {
            debug_assert!(byte < 64, "{byte} does not fit in 64-bit set");
            bits |= 1 << byte as usize;
        }
        Self(bits)
    }
}

impl BitAnd for BitSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(p1, p2)| {
            (
                p1.chars().map(char_to_priority).collect::<BitSet>(),
                p2.chars().map(char_to_priority).collect::<BitSet>(),
            )
        })
        .map(|(s1, s2)| (s1 & s2).reduce_sum())
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .chunks(3)
        .map(|lines| {
            (
                lines[0].chars().map(char_to_priority).collect::<BitSet>(),
                lines[1].chars().map(char_to_priority).collect::<BitSet>(),
                lines[2].chars().map(char_to_priority).collect::<BitSet>(),
            )
        })
        .map(|(s1, s2, s3)| (s1 & s2 & s3).reduce_sum())
        .sum()
}

fn char_to_priority(c: char) -> u32 {
    let code = match c {
        c if ('a'..='z').contains(&c) => 1 + (c as u8) - b'a',
        c if ('A'..='Z').contains(&c) => 27 + (c as u8) - b'A',
        c => panic!("invalid char {c}"),
    };
    code as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(part1(input), 157);
    }

    #[test]
    fn part2_example() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(part2(input), 70);
    }
}
