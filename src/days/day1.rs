#[derive(Clone)]
pub struct Elf(Vec<i32>);

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|s| {
            Elf(s
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect())
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Elf]) -> i32 {
    input.iter().map(|e| e.0.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[Elf]) -> i32 {
    let mut totals = input
        .iter()
        .map(|e| e.0.iter().sum::<i32>())
        .collect::<Vec<_>>();
    totals.sort_unstable();
    totals.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        assert_eq!(part1(&parse(input)), 24000);
    }

    #[test]
    fn part2_example() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        assert_eq!(part2(&parse(input)), 45000);
    }
}
