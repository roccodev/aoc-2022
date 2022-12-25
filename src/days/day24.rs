use std::collections::VecDeque;

use enumset::{EnumSet, EnumSetType};
use fxhash::FxHashSet;

#[derive(Clone, PartialEq, Eq)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Blizzard(Direction),
}

#[derive(EnumSetType, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Blizzard {
    initial_pos: (usize, usize),
    direction: Direction,
}

impl Grid {
    fn has_blizzard(&self, pos: (usize, usize), time: usize) -> bool {
        let width = self.width as isize;
        let height = self.height as isize;
        let time = time as isize;
        let (x, y) = (pos.0 as isize, pos.1 as isize);
        self.is_blizzard_direction((x, self.wrap(y + time + 1, height)), Direction::Up)
            || self.is_blizzard_direction((x, self.wrap(y - time - 1, height)), Direction::Down)
            || self.is_blizzard_direction((self.wrap(x - time - 1, width), y), Direction::Right)
            || self.is_blizzard_direction((self.wrap(x + time + 1, width), y), Direction::Left)
    }

    #[inline(always)]
    fn wrap(&self, coord: isize, val: isize) -> isize {
        (coord - 1).rem_euclid(val) + 1
    }

    fn is_blizzard_direction(&self, pos: (isize, isize), direction: Direction) -> bool {
        match self.cells[pos.1 as usize][pos.0 as usize] {
            Cell::Blizzard(dir) => dir == direction,
            _ => false,
        }
    }

    fn is_end(&self, pos: (usize, usize), reverse: bool) -> bool {
        if reverse {
            pos == (1, 0)
        } else {
            pos.1 == self.height + 1 && pos.0 == self.width
        }
    }

    fn get_neighbors(
        &self,
        pos: (usize, usize),
        time: usize,
        reverse: bool,
    ) -> Vec<(usize, usize)> {
        EnumSet::<Direction>::all()
            .into_iter()
            .filter_map(|dir| {
                let (dx, dy) = dir.get_pos_mod();
                let new_pos = (pos.0 as isize + dx, pos.1 as isize + dy);
                if self.is_end((new_pos.0 as usize, new_pos.1 as usize), reverse) {
                    return Some((new_pos.0 as usize, new_pos.1 as usize));
                }
                if new_pos.0 < 1
                    || new_pos.1 < 1
                    || new_pos.0 > self.width as isize
                    || new_pos.1 > self.height as isize
                {
                    return None;
                }
                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                (!self.has_blizzard(new_pos, time)).then_some(new_pos)
            })
            .collect()
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

fn bfs(start: (usize, usize), grid: Grid, reverse: bool, step: usize) -> (usize, Grid) {
    let mut explored: FxHashSet<((usize, usize), usize)> = FxHashSet::default();
    let mut to_visit: VecDeque<((usize, usize), usize)> = VecDeque::new();

    explored.insert((start, step));
    to_visit.push_back((start, step));

    while let Some((pos, steps)) = to_visit.pop_front() {
        if grid.is_end(pos, reverse) {
            return (steps, grid);
        }
        // States repeat after (w-2 * h-2) iterations
        let unique_step = (steps + 1) % (grid.width * grid.height);
        for neighbor in grid.get_neighbors(pos, steps, reverse) {
            if !explored.contains(&(neighbor, unique_step)) {
                explored.insert((neighbor, unique_step));
                to_visit.push_back((neighbor, steps + 1));
            }
        }
        if !grid.has_blizzard(pos, steps) && !explored.contains(&(pos, unique_step)) {
            explored.insert((pos, unique_step));
            to_visit.push_back((pos, steps + 1));
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
                    '>' => Cell::Blizzard(Direction::Right),
                    '<' => Cell::Blizzard(Direction::Left),
                    'v' => Cell::Blizzard(Direction::Down),
                    '^' => Cell::Blizzard(Direction::Up),
                    c => panic!("invalid char {c}"),
                })
                .collect()
        })
        .collect();
    Grid {
        width: cells[0].len() - 2,
        height: cells.len() - 2,
        cells,
    }
}

#[aoc(day24, part1)]
pub fn part1(input: &Grid) -> usize {
    let grid = input.clone();
    bfs((1, 0), grid, false, 0).0
}

#[aoc(day24, part2)]
pub fn part2(input: &Grid) -> usize {
    let grid = input.clone();
    let (steps_a, grid) = bfs((1, 0), grid, false, 0);
    let (steps_b, grid) = bfs((grid.width, grid.height + 1), grid, true, steps_a);
    let (steps_c, _) = bfs((1, 0), grid, false, steps_b);
    steps_c
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
