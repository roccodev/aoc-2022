#[aoc_generator(day20)]
fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day20, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut buffer = input
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect::<Vec<_>>();
    let copy = buffer.clone();

    let len = buffer.len() as i32;
    for (num, idx) in copy {
        if num == 0 {
            continue;
        }
        let pos = buffer.iter().position(|&n| n == (num, idx)).unwrap() as i32;
        let dir = num.signum();
        let mut idx = pos;
        for _ in 0..(num.abs().rem_euclid(len - 1)) {
            let next_idx = (idx + dir).rem_euclid(len);
            buffer.swap(idx as usize, next_idx as usize);
            idx = next_idx;
        }
    }
    let zero = buffer.iter().position(|&n| n.0 == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|m| buffer[(zero + m) % buffer.len()].0)
        .sum()
}

#[aoc(day20, part2)]
pub fn part2(input: &[i32]) -> i64 {
    let mut buffer = input
        .iter()
        .enumerate()
        .map(|(i, &v)| (v as i64 * 811589153i64, i))
        .collect::<Vec<_>>();
    let copy = buffer.clone();

    let len = buffer.len() as i64;
    for _ in 0..10 {
        for &(num, idx) in &copy {
            if num == 0 {
                continue;
            }
            let pos = buffer.iter().position(|&n| n == (num, idx)).unwrap() as i64;
            let dir = num.signum();
            let mut idx = pos;
            for _ in 0..(num.abs().rem_euclid(len - 1)) {
                let next_idx = (idx + dir).rem_euclid(len);
                buffer.swap(idx as usize, next_idx as usize);
                idx = next_idx;
            }
        }
    }
    let zero = buffer.iter().position(|&n| n.0 == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|m| buffer[(zero + m) % buffer.len()].0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"1
2
-3
3
-2
0
4"#;
        assert_eq!(part1(&parse(input)), 10605);
    }

    #[test]
    fn part2_example() {
        let input = r#"1
2
-3
3
-2
0
4"#;
        assert_eq!(part2(&parse(input)), 0);
    }
}
