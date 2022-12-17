use fxhash::FxHashMap;

struct Grid {
    inner: FxHashMap<(isize, isize), usize>,
    wind_direction: bool, // right: true
}

struct Piece {
    area: &'static [(isize, isize)],
    piece_id: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct MemoState {
    line: Vec<(isize, isize)>,
    dot: Vec<(isize, isize)>,
    wind_index: usize,
}

static PIECE_AREA_LINE: [(isize, isize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
static PIECE_AREA_CROSS: [(isize, isize); 5] = [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
static PIECE_AREA_SEAT: [(isize, isize); 5] = [(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
static PIECE_AREA_COLUMN: [(isize, isize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
static PIECE_AREA_DOT: [(isize, isize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

impl Grid {
    fn new(first_direction: bool) -> Self {
        Self {
            inner: FxHashMap::default(),
            wind_direction: first_direction,
        }
    }

    fn is_obstructed(&self, pos: (isize, isize), not_piece: &Piece) -> bool {
        pos.0 < 0
            || pos.0 > 6
            || pos.1 < 0
            || self
                .inner
                .get(&pos)
                .map(|&p| p != not_piece.piece_id)
                .unwrap_or(false)
    }
}

impl Piece {
    fn new(shape: &'static [(isize, isize)], id: usize) -> Self {
        Self {
            area: shape,
            piece_id: id,
        }
    }

    fn move_all(
        &self,
        grid: &mut Grid,
        top: (isize, isize),
        direction: (isize, isize),
    ) -> (bool, (isize, isize)) {
        let mut moves = [None; 16];
        for &(dx, dy) in self.area {
            let (old_x, old_y) = (top.0 + dx, top.1 - dy);
            let (new_x, new_y) = (old_x + direction.0, old_y + direction.1);
            if grid.is_obstructed((new_x, new_y), self) {
                return (true, top);
            }
            moves[dx as usize + dy as usize * 4] = Some(((old_x, old_y), (new_x, new_y)));
        }
        for (old, _) in moves.iter().flatten() {
            grid.inner.remove(old);
        }
        for (_, new) in moves.iter().flatten().copied() {
            grid.inner.insert(new, self.piece_id);
        }
        (false, (top.0 + direction.0, top.1 + direction.1))
    }

    fn do_move(&self, grid: &mut Grid, top_pos: (isize, isize)) -> (bool, (isize, isize)) {
        let (_, new_pos) =
            self.move_all(grid, top_pos, (if grid.wind_direction { 1 } else { -1 }, 0));
        self.move_all(grid, new_pos, (0, -1))
    }
}

#[inline]
fn set_to_sorted_vec(input: &FxHashMap<(isize, isize), usize>) -> Vec<(isize, isize)> {
    let mut copy = input.keys().copied().collect::<Vec<_>>();
    let min_y = copy
        .iter()
        .map(|(_, y)| y)
        .min()
        .copied()
        .unwrap_or_default();
    for tup in &mut copy {
        tup.1 -= min_y;
    }
    copy.sort();
    copy
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '>').collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &[bool]) -> isize {
    let mut grid = Grid::new(input[0]);
    let mut wind_index = 0;
    let order = [
        // shape, height
        (PIECE_AREA_LINE.as_ref(), 1),
        (&PIECE_AREA_CROSS, 3),
        (&PIECE_AREA_SEAT, 3),
        (&PIECE_AREA_COLUMN, 4),
        (&PIECE_AREA_DOT, 2),
    ];
    let mut rested = 0;
    while rested < 2022 {
        let (shape, height) = order[rested % order.len()];
        let new_piece = Piece::new(shape, rested);
        let max_y = grid
            .inner
            .keys()
            .max_by_key(|pos| pos.1)
            .copied()
            .unwrap_or((0, -1))
            .1;
        let mut pos = (2, max_y + 3 + height);
        while let (false, new_pos) = new_piece.do_move(&mut grid, pos) {
            pos = new_pos;
            wind_index = (wind_index + 1) % input.len();
            grid.wind_direction = input[wind_index];
        }
        wind_index = (wind_index + 1) % input.len();
        grid.wind_direction = input[wind_index];
        rested += 1;
    }
    grid.inner.into_keys().max_by_key(|pos| pos.1).unwrap().1 + 1
}

#[aoc(day17, part2)]
pub fn part2(input: &[bool]) -> isize {
    const TARGET: usize = 1000000000000;

    let mut grid = Grid::new(input[0]);
    let mut visited: FxHashMap<MemoState, (usize, isize)> = FxHashMap::default();
    let mut max_ys: FxHashMap<usize, isize> = FxHashMap::default();
    let mut wind_index = 0;
    let order = [
        // shape, height
        (PIECE_AREA_LINE.as_ref(), 1),
        (&PIECE_AREA_CROSS, 3),
        (&PIECE_AREA_SEAT, 3),
        (&PIECE_AREA_COLUMN, 4),
        (&PIECE_AREA_DOT, 2),
    ];
    let mut rested = 0;
    let mut last_line = vec![];
    while rested < TARGET {
        let piece_index = rested % order.len();
        if piece_index == 0 {
            last_line = set_to_sorted_vec(&grid.inner);
        } else if piece_index == 4 {
            let memo_state = MemoState {
                dot: set_to_sorted_vec(&grid.inner),
                line: last_line.clone(),
                wind_index,
            };
            if let Some((rested_src, max_y_src)) = visited.get(&memo_state) {
                let diff = (rested - rested_src) as isize;
                let y_diff = grid.inner.keys().max_by_key(|pos| pos.1).unwrap().1 + 1 - max_y_src;
                let (factor, real_target) = (TARGET as isize / diff, TARGET as isize % diff);
                let real_target_max_y = max_ys[&(real_target as usize)];
                return real_target_max_y + y_diff * factor;
            }

            visited.insert(
                memo_state,
                (
                    rested,
                    grid.inner
                        .keys()
                        .max_by_key(|pos| pos.1)
                        .unwrap_or(&(0, 0))
                        .1
                        + 1,
                ),
            );
        }

        let (shape, height) = order[piece_index];
        let new_piece = Piece::new(shape, rested);
        let max_y = grid
            .inner
            .keys()
            .max_by_key(|pos| pos.1)
            .copied()
            .unwrap_or((0, -1))
            .1;
        let mut pos = (2, max_y + 3 + height);
        while let (false, new_pos) = new_piece.do_move(&mut grid, pos) {
            pos = new_pos;
            wind_index = (wind_index + 1) % input.len();
            grid.wind_direction = input[wind_index];
        }
        wind_index = (wind_index + 1) % input.len();
        grid.wind_direction = input[wind_index];
        rested += 1;

        // If one row is fully covered, discard everything below it
        let mut counts: FxHashMap<isize, isize> = FxHashMap::default();
        for (_, y) in grid.inner.keys() {
            *counts.entry(*y).or_default() += 1isize;
        }
        for (y, _) in counts.iter().filter(|(_, v)| **v == 7) {
            grid.inner.retain(|(_, y1), _| y1 >= y);
        }
        max_ys.insert(
            rested,
            grid.inner.keys().max_by_key(|pos| pos.1).unwrap().1 + 1,
        );
    }
    max_ys[&rested]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
        assert_eq!(part1(&parse(input)), 3068);
    }
}
