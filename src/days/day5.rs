use std::collections::VecDeque;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Move {
    from: usize,
    to: usize,
    n: usize,
}

#[derive(Debug, Clone)]
pub struct Storage {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

impl Storage {
    fn do_move(&mut self, mov: &Move, reverse: bool) {
        let Move { from, to, n } = *mov;
        let (from, to) = (from - 1, to - 1);
        let (to, from) = if from < to {
            let (left, right) = self.stacks.split_at_mut(from + 1);
            (&mut right[to - from - 1], &mut left[from])
        } else {
            let (left, right) = self.stacks.split_at_mut(to + 1);
            (&mut left[to], &mut right[from - to - 1])
        };
        if reverse {
            from.drain(0..n).rev().for_each(|c| to.push_front(c));
        } else {
            from.drain(0..n).for_each(|c| to.push_front(c));
        }
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Storage {
    let mut stacks = vec![VecDeque::default(); input.lines().next().unwrap().len() / 3];
    let mut last = 0;
    for (i, line) in input.lines().enumerate() {
        last = i;
        if !line.chars().any(|c| c == '[') {
            break;
        }
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push_back(c);
            }
        }
    }

    let regex = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();
    let moves = input
        .lines()
        .skip(last + 2)
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            Move {
                from: captures[2].parse().unwrap(),
                to: captures[3].parse().unwrap(),
                n: captures[1].parse().unwrap(),
            }
        })
        .collect();
    Storage { stacks, moves }
}

#[aoc(day5, part1)]
pub fn part1(input: &Storage) -> String {
    let mut storage = input.clone();
    for mov in storage.moves.clone() {
        storage.do_move(&mov, false);
    }
    storage
        .stacks
        .into_iter()
        .filter_map(|mut q| q.pop_front())
        .collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &Storage) -> String {
    let mut storage = input.clone();
    for mov in storage.moves.clone() {
        storage.do_move(&mov, true);
    }
    storage
        .stacks
        .into_iter()
        .filter_map(|mut q| q.pop_front())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        assert_eq!(part1(&parse(input)), "CMZ");
    }

    #[test]
    fn part2_example() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        assert_eq!(part2(&parse(input)), "MCD");
    }
}
