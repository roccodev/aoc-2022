use fxhash::FxHashSet;

#[derive(Clone)]
pub struct Move {
    count: usize,
    coords: (i32, i32),
}

#[derive(Default, Clone)]
struct Rope {
    parts: Vec<(i32, i32)>,
}

impl Rope {
    fn new(part_count: usize) -> Self {
        Self {
            parts: vec![(0, 0); part_count],
        }
    }

    fn do_move(&mut self, (x, y): (i32, i32), visited: &mut FxHashSet<(i32, i32)>) {
        self.parts[0].0 += x;
        self.parts[0].1 += y;
        self.check_tail(visited);
    }

    fn check_tail(&mut self, visited: &mut FxHashSet<(i32, i32)>) {
        self.for_each_window_mut(2, |parts, is_tail| {
            let (head, tail) = (parts[0], parts[1]);
            let dx = head.0 - tail.0;
            let dy = head.1 - tail.1;
            if dx.abs() > 1 || dy.abs() > 1 {
                parts[1].0 += dx.signum();
                parts[1].1 += dy.signum();
                if is_tail {
                    visited.insert(parts[1]);
                }
            }
        });
    }

    // There is no `windows_mut` in [`std::slice`] because e.g. `collect`ing the returned
    // iterator would give you copies of overlapping mutable references.
    fn for_each_window_mut(
        &mut self,
        window_size: usize,
        mut callback: impl FnMut(&mut [(i32, i32)], bool),
    ) {
        let (mut a, mut b) = (0, window_size);
        let last = self.parts.len();
        while b <= self.parts.len() {
            callback(&mut self.parts[a..b], b == last);
            a += 1;
            b += 1;
        }
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let dir = words.next().unwrap();
            let count = words.next().unwrap().parse().unwrap();
            let coords = match dir {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                d => panic!("invalid direction {d}"),
            };
            Move { coords, count }
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Move]) -> usize {
    let mut rope = Rope::new(2);
    let mut visited = FxHashSet::default();
    visited.insert((0, 0));
    for mov in input {
        for _ in 0..mov.count {
            rope.do_move(mov.coords, &mut visited);
        }
    }
    visited.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Move]) -> usize {
    let mut rope = Rope::new(10);
    let mut visited = FxHashSet::default();
    visited.insert((0, 0));
    for mov in input {
        for _ in 0..mov.count {
            rope.do_move(mov.coords, &mut visited);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        assert_eq!(part1(&parse(input)), 13);
    }

    #[test]
    fn part2_example() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        assert_eq!(part2(&parse(input)), 1);
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        assert_eq!(part2(&parse(input)), 36);
    }
}
