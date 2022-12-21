use std::{convert::Infallible, fmt::Debug, str::FromStr};

use fxhash::FxHashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Key(u32);

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum Instruction {
    Input,
    Immediate(i64),
    Add(Key, Key),
    Sub(Key, Key),
    Mul(Key, Key),
    Div(Key, Key),
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub struct Entry {
    key: Key,
    instruction: Instruction,
}

#[derive(Default)]
struct Memory {
    variables: FxHashMap<Key, i64>,
}

impl Memory {
    fn run(&mut self, instruction: Entry) {
        if self.variables.contains_key(&instruction.key) {
            return;
        }
        let result = match instruction.instruction {
            Instruction::Immediate(imm) => imm,
            Instruction::Add(x, y) => self.get(x) + self.get(y),
            Instruction::Sub(x, y) => self.get(x) - self.get(y),
            Instruction::Mul(x, y) => self.get(x) * self.get(y),
            Instruction::Div(x, y) => self.get(x) / self.get(y),
            Instruction::Input => return,
        };
        self.variables.insert(instruction.key, result);
    }

    fn get(&self, key: Key) -> i64 {
        self.variables[&key]
    }
}

impl FromStr for Key {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        assert!(s.len() == 4);
        Ok(Key((s[0] as u32) << 24
            | (s[1] as u32) << 16
            | (s[2] as u32) << 8
            | (s[3] as u32)))
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (a, b, c, d) = (self.0 >> 24, self.0 >> 16, self.0 >> 8, self.0 as u8);
        let (a, b, c, d) = (a as u8 as char, b as u8 as char, c as u8 as char, d as char);
        write!(f, "{a}{b}{c}{d}")
    }
}

impl Instruction {
    fn run_dependencies(&self, map: &FxHashMap<Key, Entry>, mem: &mut Memory) -> bool {
        match self {
            Self::Add(x, y) | Self::Sub(x, y) | Self::Mul(x, y) | Self::Div(x, y) => {
                let r1 = map[x].instruction.run_dependencies(map, mem);
                let r2 = map[y].instruction.run_dependencies(map, mem);
                if r1 {
                    mem.run(map[x]);
                }
                if r2 {
                    mem.run(map[y]);
                }
                r1 && r2
            }
            Self::Input => false,
            _ => true,
        }
    }
}

impl Entry {
    fn run(&self, map: &FxHashMap<Key, Entry>, mem: &mut Memory) -> Option<i64> {
        self.instruction.run_dependencies(map, mem).then(|| {
            mem.run(*self);
            mem.get(self.key)
        })
    }

    /// Calculates the immediate value of a branch, if possible.
    /// If a human node is reached, returns None. Otherwise, it returns the new target value
    /// after taking the branch into account.
    fn run_until_human(
        &self,
        map: &FxHashMap<Key, Entry>,
        target_value: i64,
        memory: &mut Memory,
    ) -> Result<i64, i64> {
        match &self.instruction {
            Instruction::Input => Err(target_value),
            Instruction::Immediate(_) => Ok(target_value),
            Instruction::Add(x, y) => match (map[x].run(map, memory), map[y].run(map, memory)) {
                (Some(vx), None) => map[y].run_until_human(map, target_value - vx, memory),
                (None, Some(vy)) => map[x].run_until_human(map, target_value - vy, memory),
                _ => unreachable!(),
            },
            Instruction::Sub(x, y) => match (map[x].run(map, memory), map[y].run(map, memory)) {
                (Some(vx), None) => map[y].run_until_human(map, vx - target_value, memory),
                (None, Some(vy)) => map[x].run_until_human(map, target_value + vy, memory),
                _ => unreachable!(),
            },
            Instruction::Mul(x, y) => match (map[x].run(map, memory), map[y].run(map, memory)) {
                (Some(vx), None) => map[y].run_until_human(map, target_value / vx, memory),
                (None, Some(vy)) => map[x].run_until_human(map, target_value / vy, memory),
                _ => unreachable!(),
            },
            Instruction::Div(x, y) => match (map[x].run(map, memory), map[y].run(map, memory)) {
                (Some(vx), None) => map[y].run_until_human(map, vx / target_value, memory),
                (None, Some(vy)) => map[x].run_until_human(map, target_value * vy, memory),
                _ => unreachable!(),
            },
        }
    }

    fn dependencies(&self) -> (Key, Key) {
        match self.instruction {
            Instruction::Add(x, y)
            | Instruction::Sub(x, y)
            | Instruction::Mul(x, y)
            | Instruction::Div(x, y) => (x, y),
            _ => panic!(),
        }
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let key = words[0][..words[0].len() - 1].parse().unwrap();
            let inst = if words.len() == 2 {
                Instruction::Immediate(words[1].parse().unwrap())
            } else {
                let x = words[1].parse().unwrap();
                let y = words[3].parse().unwrap();
                match words[2] {
                    "+" => Instruction::Add(x, y),
                    "-" => Instruction::Sub(x, y),
                    "*" => Instruction::Mul(x, y),
                    "/" => Instruction::Div(x, y),
                    op => panic!("invalid op {op}"),
                }
            };
            Entry {
                key,
                instruction: inst,
            }
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn part1(input: &[Entry]) -> i64 {
    let mut memory = Memory::default();
    let root: Key = "root".parse().unwrap();
    let entry_map = input
        .iter()
        .cloned()
        .map(|e| (e.key, e))
        .collect::<FxHashMap<_, _>>();
    entry_map[&root]
        .instruction
        .run_dependencies(&entry_map, &mut memory);
    memory.run(entry_map[&root]);
    memory.get(root)
}

#[aoc(day21, part2)]
pub fn part2(input: &[Entry]) -> i64 {
    let mut memory = Memory::default();
    let root: Key = "root".parse().unwrap();
    let mut entry_map = input
        .iter()
        .cloned()
        .map(|e| (e.key, e))
        .collect::<FxHashMap<_, _>>();
    entry_map.insert(
        "humn".parse().unwrap(),
        Entry {
            key: "humn".parse().unwrap(),
            instruction: Instruction::Input,
        },
    );
    let deps = entry_map[&root].dependencies();
    let deps: (Vec<Key>, Vec<Key>) = [deps.0, deps.1]
        .into_iter()
        .partition(|k| entry_map[k].run(&entry_map, &mut memory).is_some());
    let immediate_branch = deps.0[0];
    let human_branch = deps.1[0];

    // From the root, we have two branches, one of which contains the "humn" node.
    // Our target value will start with the value of the "immediate" branch, i.e. the one
    // without the "humn" node.
    memory.run(entry_map[&immediate_branch]);
    let target_value = memory.get(immediate_branch);
    entry_map[&human_branch]
        .run_until_human(&entry_map, target_value, &mut memory)
        .unwrap_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"dbpl: 5
cczh: sllz + lgvd
zczc: 2
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
ptdq: humn - dvpt
root: pppw + sjmn
hmdt: 32"#;
        assert_eq!(part1(&parse(input)), 152);
    }

    #[test]
    fn part2_example() {
        let input = r#"dbpl: 5
cczh: sllz + lgvd
zczc: 2
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
ptdq: humn - dvpt
root: pppw + sjmn
hmdt: 32"#;
        assert_eq!(part2(&parse(input)), 301);
    }
}
