use fxhash::FxHashSet;

#[derive(Clone, Copy)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Clone)]
pub struct Grid {
    // Columns first
    inner: Vec<Vec<u8>>,
    lines: Vec<Line>,
    blocks: FxHashSet<(isize, isize)>,
    x_bounds: (usize, usize),
    y_bounds: (usize, usize),
}

#[derive(PartialEq, Eq)]
enum SandResult {
    Placed,
    Void,
}

impl Grid {
    fn set(&mut self, coords: (usize, usize), val: u8) {
        let x = coords.0 - self.x_bounds.0;
        self.inner[x][coords.1] = val;
    }

    fn get_fixed(&self, coords: (usize, usize)) -> u8 {
        if coords.0 >= self.inner.len() || coords.1 >= self.inner[0].len() {
            u8::MAX
        } else {
            self.inner[coords.0][coords.1]
        }
    }

    fn spawn_sand(&mut self, coords: (usize, usize)) -> SandResult {
        let Some((x, y)) = self.fix_coords(coords) else { return SandResult::Void };
        let column = &mut self.inner[x];
        let height = column.len();
        if let Some(obstacle) = column.iter().skip(y).position(|&v| v != 0) {
            let mut y = y + obstacle - 1;
            let mut x = x;
            loop {
                if y >= height || x == 0 || x >= self.inner.len() {
                    return SandResult::Void;
                }
                if !self.inner[x].iter().skip(y).any(|&v| v != 0) {
                    return SandResult::Void;
                }
                if self.get_fixed((x, y + 1)) == 0 {
                    y += 1;
                    continue;
                }
                if self.get_fixed((x - 1, y + 1)) == 0 {
                    x -= 1;
                    y += 1;
                    continue;
                }
                if self.get_fixed((x + 1, y + 1)) == 0 {
                    x += 1;
                    y += 1;
                    continue;
                }
                self.inner[x][y] = 1;
                return SandResult::Placed;
            }
        } else {
            SandResult::Void
        }
    }

    fn test_block(&self, coords: (isize, isize)) -> bool {
        if coords.1 == self.y_bounds.1 as isize + 2 {
            true
        } else {
            self.blocks.contains(&coords)
        }
    }

    fn spawn_sand_til_top(&mut self, coords: (isize, isize)) -> SandResult {
        let (mut x, mut y) = coords;
        if self.blocks.contains(&coords) {
            return SandResult::Void;
        }
        loop {
            if !self.test_block((x, y + 1)) {
                y += 1;
                continue;
            }
            if !self.test_block((x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }
            if !self.test_block((x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }
            self.blocks.insert((x, y));
            return SandResult::Placed;
        }
    }

    fn fix_coords(&self, coords: (usize, usize)) -> Option<(usize, usize)> {
        if coords.0 < self.x_bounds.0 || coords.0 > self.x_bounds.1 {
            return None;
        }
        Some((coords.0 - self.x_bounds.0, coords.1))
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Grid {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (usize::MAX, 0, usize::MAX, 0);
    let lines = input
        .lines()
        .flat_map(|line| {
            let points = line
                .split(" -> ")
                .map(|coords| {
                    let mut split = coords.split(',').map(|n| n.parse().unwrap());
                    let x = split.next().unwrap();
                    (x, split.next().unwrap())
                })
                .collect::<Vec<_>>();
            points
                .windows(2)
                .map(|points| {
                    let (x1, y1) = points[0];
                    let (x2, y2) = points[1];
                    min_x = min_x.min(x1).min(x2);
                    max_x = max_x.max(x1).max(x2);
                    min_y = min_y.min(y1).min(y2);
                    max_y = max_y.max(y1).max(y2);
                    Line {
                        start: (x1.min(x2), y1.min(y2)),
                        end: (x1.max(x2), y1.max(y2)),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let grid = vec![vec![0u8; max_y + 1]; max_x - min_x + 1];
    Grid {
        inner: grid,
        lines,
        blocks: FxHashSet::default(),
        x_bounds: (min_x, max_x),
        y_bounds: (min_y, max_y),
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &Grid) -> i32 {
    let mut grid = input.clone();
    for line in &input.lines {
        for x in line.start.0..=line.end.0 {
            for y in line.start.1..=line.end.1 {
                grid.set((x, y), 1);
            }
        }
    }
    let mut count = 0;
    while grid.spawn_sand((500, 0)) != SandResult::Void {
        count += 1;
    }
    count
}

#[aoc(day14, part2)]
pub fn part2(input: &Grid) -> i32 {
    let mut grid = input.clone();
    for line in &input.lines {
        for x in line.start.0..=line.end.0 {
            for y in line.start.1..=line.end.1 {
                grid.blocks.insert((x as isize, y as isize));
            }
        }
    }

    let mut count = 0;
    while grid.spawn_sand_til_top((500, 0)) != SandResult::Void {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        assert_eq!(part1(&parse(input)), 24);
    }

    #[test]
    fn part2_example() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        assert_eq!(part2(&parse(input)), 93);
    }
}
