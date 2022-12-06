use crate::util::BitSet;

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
