use std::collections::VecDeque;

use fxhash::FxHashSet;

#[derive(Debug)]
pub struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

struct Space<'c> {
    cubes: &'c FxHashSet<(isize, isize, isize)>,
    min: isize,
    max: isize,
}

impl<'a> Space<'a> {
    #[inline]
    fn is_outside(
        &self,
        air: &FxHashSet<(isize, isize, isize)>,
        cube: (isize, isize, isize),
    ) -> bool {
        !self.cubes.contains(&cube) && air.contains(&cube)
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',');
            Cube {
                x: coords.next().unwrap().parse().unwrap(),
                y: coords.next().unwrap().parse().unwrap(),
                z: coords.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Cube]) -> i32 {
    let cubes = input
        .iter()
        .map(|&Cube { x, y, z }| (x, y, z))
        .collect::<FxHashSet<_>>();
    input
        .iter()
        .map(|cube| {
            (!cubes.contains(&(cube.x - 1, cube.y, cube.z)) as u8
                + !cubes.contains(&(cube.x + 1, cube.y, cube.z)) as u8
                + !cubes.contains(&(cube.x, cube.y - 1, cube.z)) as u8
                + !cubes.contains(&(cube.x, cube.y + 1, cube.z)) as u8
                + !cubes.contains(&(cube.x, cube.y, cube.z - 1)) as u8
                + !cubes.contains(&(cube.x, cube.y, cube.z + 1)) as u8) as i32
        })
        .sum()
}

fn try_fill(cube: (isize, isize, isize), space: &Space<'_>) -> FxHashSet<(isize, isize, isize)> {
    let mut visited: FxHashSet<(isize, isize, isize)> = FxHashSet::default();
    let mut queue = VecDeque::new();
    let mut air = FxHashSet::default();
    queue.push_back(cube);
    air.insert(cube);

    while let Some(cube) = queue.pop_front() {
        if !space.cubes.contains(&cube) && !visited.contains(&cube) {
            air.insert(cube);

            push(&mut queue, space, (cube.0 - 1, cube.1, cube.2));
            push(&mut queue, space, (cube.0 + 1, cube.1, cube.2));
            push(&mut queue, space, (cube.0, cube.1 - 1, cube.2));
            push(&mut queue, space, (cube.0, cube.1 + 1, cube.2));
            push(&mut queue, space, (cube.0, cube.1, cube.2 - 1));
            push(&mut queue, space, (cube.0, cube.1, cube.2 + 1));
        }
        visited.insert(cube);
    }
    air
}

fn push(
    queue: &mut VecDeque<(isize, isize, isize)>,
    space: &Space<'_>,
    coords: (isize, isize, isize),
) {
    if coords.0 >= space.min
        && coords.0 <= space.max
        && coords.1 >= space.min
        && coords.1 <= space.max
        && coords.2 >= space.min
        && coords.2 <= space.max
    {
        queue.push_back(coords);
    }
}

#[aoc(day18, part2)]
pub fn part2(input: &[Cube]) -> i32 {
    let cubes = input
        .iter()
        .map(|&Cube { x, y, z }| (x, y, z))
        .collect::<FxHashSet<_>>();
    let max_x = input.iter().map(|c| c.x).max().unwrap();
    let max_y = input.iter().map(|c| c.y).max().unwrap();
    let max_z = input.iter().map(|c| c.z).max().unwrap();
    let min_x = input.iter().map(|c| c.x).min().unwrap();
    let min_y = input.iter().map(|c| c.y).min().unwrap();
    let min_z = input.iter().map(|c| c.z).min().unwrap();

    let max = max_x.max(max_y).max(max_z) + 1;
    let min = min_x.min(min_y).min(min_z) - 1;
    let space = Space {
        cubes: &cubes,
        min,
        max,
    };
    let air = try_fill((min, min, min), &space);

    input
        .iter()
        .map(|cube| {
            (space.is_outside(&air, (cube.x - 1, cube.y, cube.z)) as u8
                + space.is_outside(&air, (cube.x + 1, cube.y, cube.z)) as u8
                + space.is_outside(&air, (cube.x, cube.y - 1, cube.z)) as u8
                + space.is_outside(&air, (cube.x, cube.y + 1, cube.z)) as u8
                + space.is_outside(&air, (cube.x, cube.y, cube.z - 1)) as u8
                + space.is_outside(&air, (cube.x, cube.y, cube.z + 1)) as u8) as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;
        assert_eq!(part1(&parse(input)), 64);
    }

    #[test]
    fn part2_example() {
        let input = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;
        assert_eq!(part2(&parse(input)), 58);
    }
}
