use std::{collections::VecDeque, ops::Mul};

#[derive(Clone, Copy)]
pub enum Operation {
    Add(i32),
    Mul(i32),
    Square,
}

#[derive(Clone, Debug)]
pub enum Number {
    Scalar(i32),
    // [(n % j)], [j], where j is each monkey's test number
    Vector(Vec<i32>, Vec<i32>),
}

impl Number {
    fn vector_from_scalar(&self, mods: Vec<i32>) -> Self {
        let Self::Scalar(scalar) = self else {panic!()};
        Self::Vector(mods.iter().map(|m| scalar % m).collect(), mods)
    }

    fn scalar_add(&mut self, add: i32) {
        match self {
            Self::Scalar(i) => *i += add,
            Self::Vector(nums, mods) => {
                for (i, n) in nums.iter_mut().enumerate() {
                    *n = (*n + add) % mods[i]
                }
            }
        }
    }

    fn scalar_mul(&mut self, mul: i32) {
        match self {
            Self::Scalar(i) => *i *= mul,
            Self::Vector(nums, mods) => {
                for (i, n) in nums.iter_mut().enumerate() {
                    *n = (*n * mul) % mods[i]
                }
            }
        }
    }

    fn div3(&mut self) {
        if let Self::Scalar(i) = self {
            *i /= 3;
        }
    }

    fn square(&mut self) {
        match self {
            Self::Scalar(i) => *i *= *i,
            Self::Vector(nums, mods) => {
                for (i, n) in nums.iter_mut().enumerate() {
                    *n = (*n * *n) % mods[i]
                }
            }
        }
    }

    fn divisible_by_scalar(&self, num: i32) -> bool {
        let Self::Scalar(i) = self else { return false };
        i % num == 0
    }

    fn divisible_by_monkey(&self, monkey_id: usize) -> bool {
        let Self::Vector(nums, _) = self else { return false };
        nums[monkey_id] == 0
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Self::Scalar(n)
    }
}

#[derive(Clone)]
pub struct Monkey {
    items: VecDeque<Number>,
    operation: Operation,
    test_mod: i32,
    test_y: usize,
    test_n: usize,
}

impl Operation {
    fn run(&self, num: &mut Number) {
        match self {
            Self::Add(j) => num.scalar_add(*j),
            Self::Mul(j) => num.scalar_mul(*j),
            Self::Square => num.square(),
        }
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines().skip(1);
            let items = lines
                .next()
                .unwrap()
                .split(": ")
                .last()
                .unwrap()
                .split(", ")
                .map(|s| Number::from(s.parse::<i32>().unwrap()))
                .collect();
            let op = lines.next().unwrap();
            let test_cond = lines.next().unwrap();
            let test_y = lines.next().unwrap();
            let test_n = lines.next().unwrap();

            let operation = {
                let args = op.split_whitespace().rev().take(2).collect::<Vec<_>>();
                match (args[1], args[0]) {
                    ("+", i) => Operation::Add(i.parse().unwrap()),
                    ("*", "old") => Operation::Square,
                    ("*", i) => Operation::Mul(i.parse().unwrap()),
                    (o, a) => panic!("invalid operation {o} {a}"),
                }
            };

            let test_cond = test_cond
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let test_y = test_y.split_whitespace().last().unwrap().parse().unwrap();
            let test_n = test_n.split_whitespace().last().unwrap().parse().unwrap();

            Monkey {
                items,
                operation,
                test_mod: test_cond,
                test_y,
                test_n,
            }
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &[Monkey]) -> i32 {
    let mut monkeys = input.to_vec();
    let mut counts = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let (less_than_i, i_and_gtr) = monkeys.split_at_mut(i);
            let (monkey, gtr_than_i) = i_and_gtr.split_at_mut(1);
            let monkey = &mut monkey[0];
            for mut item in monkey.items.drain(..) {
                counts[i] += 1;
                monkey.operation.run(&mut item);
                item.div3();
                let next_monkey = if item.divisible_by_scalar(monkey.test_mod) {
                    monkey.test_y
                } else {
                    monkey.test_n
                };
                let next_monkey = if next_monkey < i {
                    &mut less_than_i[next_monkey]
                } else {
                    &mut gtr_than_i[next_monkey - i - 1]
                };
                next_monkey.items.push_back(item);
            }
        }
    }
    counts.sort_unstable();
    counts.into_iter().rev().take(2).reduce(Mul::mul).unwrap()
}

#[aoc(day11, part2)]
pub fn part2(input: &[Monkey]) -> i64 {
    let mods = input.iter().map(|m| m.test_mod).collect::<Vec<_>>();
    let mut monkeys = input.to_vec();
    for monkey in &mut monkeys {
        monkey.items = monkey
            .items
            .iter()
            .map(|n| Number::vector_from_scalar(n, mods.clone()))
            .collect();
    }
    let mut counts = vec![0; monkeys.len()];
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let (less_than_i, i_and_gtr) = monkeys.split_at_mut(i);
            let (monkey, gtr_than_i) = i_and_gtr.split_at_mut(1);
            let monkey = &mut monkey[0];
            for mut item in monkey.items.drain(..) {
                counts[i] += 1;
                monkey.operation.run(&mut item);
                let next_monkey = if item.divisible_by_monkey(i) {
                    monkey.test_y
                } else {
                    monkey.test_n
                };
                let next_monkey = if next_monkey < i {
                    &mut less_than_i[next_monkey]
                } else {
                    &mut gtr_than_i[next_monkey - i - 1]
                };
                next_monkey.items.push_back(item);
            }
        }
    }
    counts.sort_unstable();
    counts.into_iter().rev().take(2).reduce(Mul::mul).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;
        assert_eq!(part1(&parse(input)), 10605);
    }

    #[test]
    fn part2_example() {
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;
        assert_eq!(part2(&parse(input)), 10605);
    }
}
