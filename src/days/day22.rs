use std::ops::RangeInclusive;

use fxhash::{FxHashMap, FxHashSet};
use regex::Regex;

#[derive(Debug)]
pub struct Grid {
    walls: FxHashSet<(isize, isize)>,
    bounds: Vec<(RangeInclusive<isize>, RangeInclusive<isize>)>,
    starting_pos: (isize, isize),
    directions: Vec<Direction>,
    faces: FxHashMap<char, Rect>,
    edges: FxHashMap<(char, Facing), (char, Facing)>,
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    top: (isize, isize),
    bottom: (isize, isize),
}

#[derive(Debug)]
enum Direction {
    Move(usize),
    Turn(bool),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Facing {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}

use Facing::*;

//  AB
//  C
// ED
// F
// (old face, old facing) -> (new face, new facing)
const EDGES: [((char, Facing), (char, Facing)); 14] = [
    (('A', Up), ('F', Right)),
    (('A', Left), ('E', Right)),
    (('B', Up), ('F', Up)),
    (('B', Down), ('C', Left)),
    (('B', Right), ('D', Left)),
    (('C', Left), ('E', Down)),
    (('C', Right), ('B', Up)),
    (('D', Right), ('B', Left)),
    (('D', Down), ('F', Left)),
    (('E', Up), ('C', Right)),
    (('E', Left), ('A', Right)),
    (('F', Down), ('B', Down)),
    (('F', Right), ('D', Up)),
    (('F', Left), ('A', Down)),
];

const RANGES: [(char, Rect); 6] = [
    ('A', Rect::new(50, 0, 100, 50)),
    ('B', Rect::new(100, 0, 150, 50)),
    ('C', Rect::new(50, 50, 100, 100)),
    ('E', Rect::new(0, 100, 50, 150)),
    ('D', Rect::new(50, 100, 100, 150)),
    ('F', Rect::new(0, 150, 50, 200)),
];

const RANGES_EXAMPLE: [(char, Rect); 6] = [
    ('A', Rect::new(4, 0, 8, 4)),
    ('B', Rect::new(8, 0, 12, 4)),
    ('C', Rect::new(4, 4, 8, 8)),
    ('E', Rect::new(0, 8, 4, 12)),
    ('D', Rect::new(4, 8, 8, 12)),
    ('F', Rect::new(0, 12, 4, 16)),
];

impl Grid {
    fn get_next_pos(&self, current_pos: (isize, isize), facing: Facing) -> Option<(isize, isize)> {
        let pos_mod = facing.get_pos_mod();
        let new_pos = (current_pos.0 + pos_mod.0, current_pos.1 + pos_mod.1);
        let mut wrapped_pos = (isize::MIN, isize::MIN);
        let mut found = false;
        for (y_bounds, x_bounds) in &self.bounds {
            match (
                x_bounds.contains(&new_pos.0),
                y_bounds.contains(&new_pos.1),
                y_bounds.contains(&current_pos.1),
            ) {
                (true, true, _) => {
                    wrapped_pos = new_pos;
                    found = true;
                    break;
                }
                (false, _, true) => {
                    wrapped_pos = (wrap(new_pos.0, &[x_bounds]), new_pos.1);
                    found = true;
                    break;
                }
                _ => continue,
            }
        }
        if !found {
            wrapped_pos = (
                new_pos.0,
                match facing {
                    Facing::Down => wrap(
                        new_pos.1,
                        &self
                            .bounds
                            .iter()
                            .rev()
                            .skip_while(|(_, x_b)| !x_b.contains(&current_pos.0))
                            .take_while(|(_, x_b)| x_b.contains(&current_pos.0))
                            .map(|(y_b, _)| y_b)
                            .collect::<Vec<_>>(),
                    ),
                    Facing::Up => wrap(
                        new_pos.1,
                        &self
                            .bounds
                            .iter()
                            .skip_while(|(_, x_b)| !x_b.contains(&current_pos.0))
                            .take_while(|(_, x_b)| x_b.contains(&current_pos.0))
                            .map(|(y_b, _)| y_b)
                            .collect::<Vec<_>>(),
                    ),
                    _ => new_pos.1,
                },
            );
        }
        (!self.walls.contains(&wrapped_pos)).then_some(wrapped_pos)
    }

    fn get_next_cube_pos(
        &self,
        current_pos: (isize, isize),
        facing: Facing,
    ) -> Option<((isize, isize), Facing)> {
        let pos_mod = facing.get_pos_mod();
        let new_pos = (current_pos.0 + pos_mod.0, current_pos.1 + pos_mod.1);
        let mut wrapped_pos = ((isize::MIN, isize::MIN), facing);
        let mut found = false;
        for (y_bounds, x_bounds) in &self.bounds {
            match (
                x_bounds.contains(&new_pos.0),
                y_bounds.contains(&new_pos.1),
                y_bounds.contains(&current_pos.1),
            ) {
                (true, true, _) => {
                    wrapped_pos = (new_pos, facing);
                    found = true;
                    break;
                }
                (false, _, true) => {
                    wrapped_pos = self.wrap_cube(current_pos, facing);
                    found = true;
                    break;
                }
                _ => continue,
            }
        }
        if !found {
            wrapped_pos = self.wrap_cube(current_pos, facing);
        }
        (!self.walls.contains(&wrapped_pos.0)).then_some(wrapped_pos)
    }

    fn wrap_cube(&self, prev_pos: (isize, isize), prev_facing: Facing) -> ((isize, isize), Facing) {
        let prev_cube_id = *self
            .faces
            .iter()
            .find(|(_, rect)| rect.is_inside(prev_pos))
            .unwrap()
            .0;
        let prev_cube = self.faces[&prev_cube_id];
        let (rel_x, rel_y) = (prev_pos.0 - prev_cube.top.0, prev_pos.1 - prev_cube.top.1);
        // Our new pos will use either X or Y to shift along the edge
        let (next_cube_id, next_facing) = self.edges[&(prev_cube_id, prev_facing)];
        let relative_coord = match (prev_facing, next_facing) {
            (Up, Right) | (Up, Up) => rel_x,
            (Up, _) => prev_cube.bottom.0 - prev_pos.0,
            (Down, Left) | (Down, Down) => rel_x,
            (Down, _) => prev_cube.bottom.0 - prev_pos.0,
            (Left, Down) | (Left, Left) => rel_y,
            (Left, _) => prev_cube.bottom.1 - prev_pos.1,
            (Right, Up) | (Right, Right) => rel_y,
            (Right, _) => prev_cube.bottom.1 - prev_pos.1,
        };
        let next_cube = self.faces[&next_cube_id];
        let next_pos = match next_facing {
            Down => {
                // We're looking down, so we must be at the top edge
                (next_cube.top.0 + relative_coord, next_cube.top.1)
            }
            Up => (next_cube.top.0 + relative_coord, next_cube.bottom.1),
            Left => (next_cube.bottom.0, next_cube.top.1 + relative_coord),
            Right => (next_cube.top.0, next_cube.top.1 + relative_coord),
        };
        (next_pos, next_facing)
    }
}

impl Rect {
    const fn new(top_x: isize, top_y: isize, bottom_x: isize, bottom_y: isize) -> Self {
        Self {
            top: (top_x, top_y),
            bottom: (bottom_x - 1, bottom_y - 1),
        }
    }

    fn is_inside(&self, point: (isize, isize)) -> bool {
        point.0 >= self.top.0
            && point.0 <= self.bottom.0
            && point.1 >= self.top.1
            && point.1 <= self.bottom.1
    }
}

fn wrap(coord: isize, ranges: &[&RangeInclusive<isize>]) -> isize {
    let min = ranges.iter().map(|r| r.start()).min().unwrap();
    let max = ranges.iter().map(|r| r.end()).max().unwrap();
    (coord - min).rem_euclid(max + 1 - min) + min
}

impl Facing {
    fn get_pos_mod(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn turn(&self, right: bool) -> Self {
        match (self, right) {
            (Self::Up, true) => Self::Right,
            (Self::Right, true) => Self::Down,
            (Self::Down, true) => Self::Left,
            (Self::Left, true) => Self::Up,
            (Self::Up, false) => Self::Left,
            (Self::Left, false) => Self::Down,
            (Self::Down, false) => Self::Right,
            (Self::Right, false) => Self::Up,
        }
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Grid {
    let mut walls = FxHashSet::default();
    let mut bounds = Vec::new();
    let (mut last_y, mut last_bound) = (isize::MAX, isize::MIN..=isize::MIN);
    let mut start = (isize::MIN, isize::MIN);
    for (y, line) in input
        .lines()
        .enumerate()
        .take_while(|&(_, l)| !l.trim().is_empty())
    {
        let y = y as isize;
        let mut min_x = isize::MAX;
        let mut max_x = 0;
        for (x, c) in line.chars().enumerate().filter(|&(_, c)| c != ' ') {
            let x = x as isize;
            if start.0 == isize::MIN {
                start = (x, y);
            }
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            if c == '#' {
                walls.insert((x, y));
            }
        }
        let x_bound = min_x..=max_x;
        if last_bound != x_bound {
            bounds.push((last_y..=y - 1, last_bound));
            last_bound = x_bound;
            last_y = y;
        }
    }
    bounds.push((last_y..=input.lines().count() as isize - 3, last_bound));
    let regex = Regex::new(r#"(\d+)([RL]?)"#).unwrap();
    let directions = regex
        .captures_iter(input.lines().last().unwrap())
        .flat_map(|caps| {
            [
                Direction::Move(caps[1].parse().unwrap()),
                match &caps[2] {
                    "R" => Direction::Turn(true),
                    "L" => Direction::Turn(false),
                    _ => Direction::Move(0),
                },
            ]
        })
        .collect();
    bounds.remove(0);
    Grid {
        walls,
        bounds,
        starting_pos: start,
        directions,
        edges: EDGES.into_iter().collect(),
        faces: if last_y > 16 { RANGES } else { RANGES_EXAMPLE }
            .into_iter()
            .collect(),
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &Grid) -> isize {
    let mut pos = input.starting_pos;
    let mut facing = Facing::Right;
    for direction in &input.directions {
        match direction {
            Direction::Turn(r) => facing = facing.turn(*r),
            Direction::Move(n) => {
                for _ in 0..*n {
                    match input.get_next_pos(pos, facing) {
                        Some(new) => pos = new,
                        None => break,
                    }
                }
            }
        }
    }

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + facing as isize
}

#[aoc(day22, part2)]
pub fn part2(input: &Grid) -> isize {
    let mut pos = input.starting_pos;
    let mut facing = Facing::Right;
    for direction in &input.directions {
        match direction {
            Direction::Turn(r) => facing = facing.turn(*r),
            Direction::Move(n) => {
                for _ in 0..*n {
                    match input.get_next_cube_pos(pos, facing) {
                        Some((new_pos, new_facing)) => {
                            pos = new_pos;
                            facing = new_facing;
                        }
                        None => break,
                    }
                }
            }
        }
    }

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + facing as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
        assert_eq!(part1(&parse(input)), 6032);
    }

    #[test]
    fn part2_example() {
        // Sample from https://www.reddit.com/r/adventofcode/comments/zsle97/2022_day_22_part_2_help_making_a_sample_input/
        let input = r#"    ...#.#..
    .#......
    #.....#.
    ........
    ...#
    #...
    ....
    ..#.
..#....#
........
.....#..
........
#...
..#.
....
....
 
10R5L5R10L4R5L5"#;
        assert_eq!(part2(&parse(input)), 10006);
    }
}
