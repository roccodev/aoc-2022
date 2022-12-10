#[derive(Clone, Copy)]
pub enum Inst {
    Nop,
    Addx(i32),
}

impl From<Inst> for i32 {
    fn from(i: Inst) -> Self {
        match i {
            Inst::Nop => 0,
            Inst::Addx(i) => i,
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Inst> {
    input
        .lines()
        .flat_map(|line| {
            let mut words = line.split_whitespace();
            let inst = words.next().unwrap();
            let arg = words.next();

            match (inst, arg) {
                ("noop", _) => vec![Inst::Nop],
                ("addx", Some(s)) => vec![Inst::Nop, Inst::Addx(s.parse().unwrap())],
                (i, a) => panic!("invalid instruction {i} {a:?}"),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Inst]) -> i32 {
    let mut x = 1;
    let mut sum = 0;
    for (i, inst) in input.iter().enumerate() {
        x += i32::from(*inst);
        if [20, 60, 100, 140, 180, 220].contains(&(i + 2)) {
            sum += (i + 2) as i32 * x;
        }
    }
    sum
}

#[aoc(day10, part2)]
pub fn part2(input: &[Inst]) -> String {
    let mut x = 1;
    let mut buf = vec![];
    for (ci, inst) in input.iter().enumerate() {
        let ci = ci as i32;
        buf.push(if (x - 1..=x + 1).contains(&(ci % 40)) {
            '#'
        } else {
            ' '
        });
        x += i32::from(*inst);
    }
    buf.chunks(40).fold(String::from("\n"), |s, line| {
        s + &line.iter().collect::<String>() + "\n"
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
        assert_eq!(part1(&parse(input)), 13140);
    }
}
