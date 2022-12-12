use std::collections::VecDeque;
use fxhash::FxHashSet;

struct Grid {
    elevations: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize
}

impl Grid {
    fn get_neighbors(&self, (x, y): (usize, usize), reverse: bool) -> Vec<(usize, usize)> {
        let elevation = self.elevations[y][x];
        [(-1, 0), (1, 0), (0, 1), (0, -1)].into_iter().filter_map(|(dx, dy)| {
            let x = x as isize + dx;
            let y = y as isize + dy;
            if y < 0 || x < 0 || y as usize >= self.height || x as usize >= self.width {
                return None;
            }
            let x = x as usize;
            let y = y as usize;
            let new_elev = self.elevations[y][x];
            if !reverse && (new_elev < elevation || new_elev.abs_diff(elevation) < 2) {
                Some((x, y))
            } else if reverse && (elevation < new_elev || elevation.abs_diff(new_elev) < 2) {
                Some((x as usize, y as usize))
            } else {
                None
            }
        }).collect()
    }

    fn is_end(&self, pos: (usize, usize), reverse: bool) -> bool {
        if reverse {
            self.elevations[pos.1][pos.0] == b'a'
        } else {
            pos == self.end
        }
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Grid {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let elevations: Vec<Vec<u8>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                'S' => {
                    start = (x, y); 
                    b'a'
                },
                'E' => {
                    end = (x, y); 
                    b'z'
                },
                c => c as u8,
            }
        }).collect()
    }).collect();
    Grid {
        start, 
        end, 
        width: elevations[0].len(), 
        height: elevations.len(), 
        elevations
    }
}

#[aoc(day12, part1)]
fn part1(input: &Grid) -> usize {
    bfs(input.start, input, false)
}

#[aoc(day12, part2)]
fn part2(input: &Grid) -> usize {
    bfs(input.end, input, true)
}

fn bfs(start: (usize, usize), grid: &Grid, reverse: bool) -> usize {
    let mut explored: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut to_visit: VecDeque<((usize, usize), usize)> = VecDeque::new();

    explored.insert(start);
    to_visit.push_back((start, 0));

    while !to_visit.is_empty() {
        let vertex = to_visit.pop_front().unwrap();
        if grid.is_end(vertex.0, reverse) {
            return vertex.1;
        }
        for neighbor in grid.get_neighbors(vertex.0, reverse) {
            if !explored.contains(&neighbor) {
                explored.insert(neighbor);
                to_visit.push_back((neighbor, vertex.1 + 1));
            }
        }
    }
    panic!("end not reached")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

        assert_eq!(part1(&parse(input)), 31);
    }

    #[test]
    fn part2_example() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

        assert_eq!(part2(&parse(input)), 29);
    }
}
