use std::{collections::VecDeque, fmt::Display};

use enumset::{enum_set, EnumSet, EnumSetType};
use fxhash::FxHashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    inner: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
    Blizzards(EnumSet<Direction>),
}

#[derive(EnumSetType)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    // IDEA: instead of moving blizzards, calculate their position every time based on current time
    fn move_blizzards(&mut self) {
        let mut clone = self.inner.clone();
        for y in 1..1 + self.height {
            for cell in self.inner[y].iter_mut().skip(1).take(self.width) {
                *cell = Cell::Empty;
            }
        }
        for y in 1..1 + self.height {
            for x in 1..1 + self.width {
                let cell = &mut clone[y][x];
                for direction in cell.take_blizzards() {
                    let new_cell = self.blizz_cell_at((x, y), direction);
                    self.inner[new_cell.1][new_cell.0].add_blizzard(direction);
                }
            }
        }
    }

    fn blizz_cell_at(&self, start: (usize, usize), direction: Direction) -> (usize, usize) {
        let (dx, dy) = direction.get_pos_mod();
        (
            (start.0 as isize + dx - 1).rem_euclid(self.width as isize) as usize + 1,
            (start.1 as isize + dy - 1).rem_euclid(self.height as isize) as usize + 1,
        )
    }

    fn is_end(&self, pos: (usize, usize), reverse: bool) -> bool {
        if reverse {
            pos == (1, 0)
        } else {
            pos.1 == self.height + 1 && pos.0 == self.width
        }
    }

    fn get_neighbors(&self, pos: (usize, usize), reverse: bool) -> Vec<(usize, usize)> {
        EnumSet::<Direction>::all()
            .into_iter()
            .filter_map(|dir| {
                let (dx, dy) = dir.get_pos_mod();
                let new_pos = (pos.0 as isize + dx, pos.1 as isize + dy);
                if new_pos.0 < 1
                    || new_pos.1 < 0
                    || new_pos.0 > self.width as isize
                    || new_pos.1 > self.height as isize + 1
                {
                    return None;
                }
                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                (self.inner[new_pos.1][new_pos.0] == Cell::Empty).then_some(new_pos)
            })
            .collect()
    }
}

impl Cell {
    fn take_blizzards(&mut self) -> EnumSet<Direction> {
        match self {
            Self::Empty | Self::Wall => Default::default(),
            Self::Blizzards(set) => {
                let set = *set;
                *self = Self::Empty;
                set
            }
        }
    }

    fn add_blizzard(&mut self, direction: Direction) {
        match self {
            Self::Empty => *self = Self::Blizzards(enum_set!(direction)),
            Self::Blizzards(set) => {
                set.insert(direction);
            }
            _ => {}
        }
    }
}

impl Direction {
    fn get_pos_mod(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Blizzards(set) => {
                if set.len() != 1 {
                    write!(f, "{}", set.len())
                } else {
                    for dir in *set {
                        write!(
                            f,
                            "{}",
                            match dir {
                                Direction::Up => "^",
                                Direction::Down => "v",
                                Direction::Left => "<",
                                Direction::Right => ">",
                            }
                        )?
                    }
                    Ok(())
                }
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height + 2 {
            for x in 0..self.width + 2 {
                write!(f, "{}", self.inner[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn bfs(start: (usize, usize), grid: Grid, reverse: bool) -> (usize, Grid) {
    let mut explored: FxHashSet<((usize, usize), usize)> = FxHashSet::default();
    let mut to_visit: VecDeque<((usize, usize), Grid, usize)> = VecDeque::new();

    explored.insert((start, 0));
    to_visit.push_back((start, grid, 0));

    while let Some((pos, mut grid, steps)) = to_visit.pop_front() {
        if grid.is_end(pos, reverse) {
            return (steps, grid);
        }
        grid.move_blizzards();
        for neighbor in grid.get_neighbors(pos, reverse) {
            if !explored.contains(&(neighbor, steps + 1)) {
                explored.insert((neighbor, steps + 1));
                to_visit.push_back((neighbor, grid.clone(), steps + 1));
            }
        }
        if grid.inner[pos.1][pos.0] == Cell::Empty && !explored.contains(&(pos, steps + 1)) {
            explored.insert((pos, steps + 1));
            to_visit.push_back((pos, grid.clone(), steps + 1));
        }
    }
    panic!("end not reached")
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Grid {
    let cells: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    '>' => Cell::Blizzards(enum_set!(Direction::Right)),
                    '<' => Cell::Blizzards(enum_set!(Direction::Left)),
                    'v' => Cell::Blizzards(enum_set!(Direction::Down)),
                    '^' => Cell::Blizzards(enum_set!(Direction::Up)),
                    c => panic!("invalid char {c}"),
                })
                .collect()
        })
        .collect();
    Grid {
        width: cells[0].len() - 2,
        height: cells.len() - 2,
        inner: cells,
    }
}

#[aoc(day24, part1)]
pub fn part1(input: &Grid) -> usize {
    let grid = input.clone();
    bfs((1, 0), grid, false).0
}

#[aoc(day24, part2)]
pub fn part2(input: &Grid) -> usize {
    let grid = input.clone();
    let (steps_a, grid) = bfs((1, 0), grid, false);
    let (steps_b, grid) = bfs((grid.width, grid.height + 1), grid, true);
    let (steps_c, _) = bfs((1, 0), grid, false);
    steps_a + steps_b + steps_c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;
        assert_eq!(part1(&parse(input)), 18);
    }

    #[test]
    fn part2_example() {
        let input = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;
        assert_eq!(part2(&parse(input)), 54);
    }
}
