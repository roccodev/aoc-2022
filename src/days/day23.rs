use fxhash::{FxHashMap, FxHashSet};

#[derive(Clone, Debug)]
pub struct Grid {
    elves: FxHashSet<(isize, isize)>,
    directions: Vec<Direction>,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Grid {
    fn get_elf_moves(
        &self,
        elf: (isize, isize),
        destinations: &mut FxHashMap<(isize, isize), Vec<(isize, isize)>>,
    ) {
        if self
            .directions
            .iter()
            .all(|&dir| !self.check_obstructed(elf, dir))
        {
            return;
        }
        let direction = self
            .directions
            .iter()
            .find(|dir| !self.check_obstructed(elf, **dir));
        if let Some(direction) = direction {
            let pos_mod = direction.get_pos_mod();
            let dest = (elf.0 + pos_mod.0, elf.1 + pos_mod.1);
            destinations.entry(dest).or_default().push(elf);
        }
    }

    fn check_obstructed(&self, source: (isize, isize), direction: Direction) -> bool {
        let pos_mod = direction.get_pos_mod();
        let dest = (source.0 + pos_mod.0, source.1 + pos_mod.1);
        match direction {
            Direction::North | Direction::South => {
                (-1..=1).any(|m| self.elves.contains(&(dest.0 + m, dest.1)))
            }
            Direction::East | Direction::West => {
                (-1..=1).any(|m| self.elves.contains(&(dest.0, dest.1 + m)))
            }
        }
    }

    fn run_turn(&mut self) -> bool {
        let mut destinations = FxHashMap::default();
        for elf in &self.elves {
            self.get_elf_moves(*elf, &mut destinations);
        }
        let mut moved = false;
        for (destination, sources) in destinations.into_iter().filter(|(_, v)| v.len() == 1) {
            self.elves.remove(&sources[0]);
            self.elves.insert(destination);
            moved = true;
        }
        self.directions.rotate_left(1);
        moved
    }
}

impl Direction {
    fn get_pos_mod(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        }
    }
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Grid {
    let elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x as isize, y as isize)))
        })
        .collect();
    Grid {
        elves,
        directions: vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &Grid) -> isize {
    let mut grid = input.clone();
    for _ in 0..10 {
        grid.run_turn();
    }
    let min_x = grid.elves.iter().map(|(x, _)| x).min().unwrap();
    let max_x = grid.elves.iter().map(|(x, _)| x).max().unwrap();
    let min_y = grid.elves.iter().map(|(_, y)| y).min().unwrap();
    let max_y = grid.elves.iter().map(|(_, y)| y).max().unwrap();
    (max_x + 1 - min_x) * (max_y + 1 - min_y) - grid.elves.len() as isize
}

#[aoc(day23, part2)]
pub fn part2(input: &Grid) -> i64 {
    let mut grid = input.clone();
    for i in 1.. {
        if !grid.run_turn() {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;
        assert_eq!(part1(&parse(input)), 3);
    }

    #[test]
    fn part2_example() {
        let input = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;
        assert_eq!(part2(&parse(input)), 1623178306);
    }
}
