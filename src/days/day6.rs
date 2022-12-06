use crate::util::BitSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    input.as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, v)| v.len() as u32 == v.iter()
              .map(|&b| b - b'a')
              .collect::<BitSet>()
              .len())
        .unwrap().0 + 4
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    input.as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, v)| v.len() as u32 == v.iter()
              .map(|&b| b - b'a')
              .collect::<BitSet>()
              .len())
        .unwrap().0 + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn part2_example() {
        let input = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;
        assert_eq!(part2(input), 19);
    }
}
