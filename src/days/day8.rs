pub struct Grid {
    inner: Vec<Vec<i8>>,
    width: isize,
    height: isize,
}

impl Grid {
    pub fn new(inner: Vec<Vec<i8>>) -> Self {
        debug_assert!(!inner.is_empty());
        let width = inner[0].len() as isize;
        let height = inner.len() as isize;
        Self {
            inner,
            width,
            height,
        }
    }

    #[inline]
    pub fn get(&self, x: isize, y: isize) -> Option<i8> {
        self.inner
            .get(y as usize)
            .and_then(|col| col.get(x as usize))
            .copied()
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Grid {
    Grid::new(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
            .collect(),
    )
}

#[aoc(day8, part1)]
pub fn part1(input: &Grid) -> usize {
    (0..input.width)
        .flat_map(|y| (0..input.height).map(move |x| (x, y)))
        .filter(|(x, y)| {
            let val = input.get(*x, *y).unwrap();
            'dirs: for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let (mut x, mut y) = (x + dir.0, y + dir.1);
                while x >= 0 && x < input.width && y >= 0 && y <= input.height {
                    if matches!(input.get(x, y), Some(adj) if adj >= val) {
                        continue 'dirs;
                    }
                    x += dir.0;
                    y += dir.1;
                }
                return true;
            }
            false
        })
        .count()
}

#[aoc(day8, part2)]
pub fn part2(input: &Grid) -> i32 {
    (0..input.width)
        .flat_map(|y| (0..input.height).map(move |x| (x, y)))
        .map(|(x, y)| {
            let val = input.get(x, y).unwrap();
            let mut score = 1;
            for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let (mut x, mut y) = (x + dir.0, y + dir.1);
                let mut dir_score = 0;
                while x >= 0 && x < input.width && y >= 0 && y <= input.height {
                    match input.get(x, y) {
                        Some(adj) if adj < val => dir_score += 1,
                        None => break,
                        _ => {
                            dir_score += 1;
                            break;
                        }
                    }
                    x += dir.0;
                    y += dir.1;
                }
                score *= dir_score;
            }
            score
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"30373
25512
65332
33549
35390"#;
        assert_eq!(part1(&parse(input)), 21);
    }

    #[test]
    fn part2_example() {
        let input = r#"30373
25512
65332
33549
35390"#;
        assert_eq!(part2(&parse(input)), 8);
    }
}
