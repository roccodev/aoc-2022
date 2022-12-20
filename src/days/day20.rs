use std::collections::VecDeque;

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
        .collect::<VecDeque<_>>();
    let copy = buffer.clone();

    let len = buffer.len() as i32;
    for (num, idx) in copy {
        if num == 0 {
            continue;
        }
        let pos = buffer.iter().position(|&n| n == (num, idx)).unwrap() as i32;
        buffer.remove(pos as usize);
        // State repeats every (len - 1) swaps. For example,
        // 0321 -> 0231 -> 0213 -> 0321
        if num > 0 {
            buffer.rotate_left((num % (len - 1)) as usize);
        } else {
            buffer.rotate_right((-num % (len - 1)) as usize);
        }
        buffer.insert(pos as usize, (num, idx));
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
        .collect::<VecDeque<_>>();
    let copy = buffer.clone();

    let len = buffer.len() as i64;
    for _ in 0..10 {
        for &(num, idx) in &copy {
            if num == 0 {
                continue;
            }
            let pos = buffer.iter().position(|&n| n == (num, idx)).unwrap() as i32;
            buffer.remove(pos as usize);
            if num > 0 {
                buffer.rotate_left((num % (len - 1)) as usize);
            } else {
                buffer.rotate_right((-num % (len - 1)) as usize);
            }
            buffer.insert(pos as usize, (num, idx));
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
        assert_eq!(part1(&parse(input)), 3);
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
        assert_eq!(part2(&parse(input)), 1623178306);
    }
}
