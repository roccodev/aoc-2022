use regex::Regex;

pub struct Space {
    sensors: Vec<Sensor>,
    x_bounds: (isize, isize),
}

struct Sensor {
    coords: (isize, isize),
    closest_beacon: (isize, isize),
    closest_dist: usize,
}

impl Sensor {
    #[inline(always)]
    fn dist_to(&self, point: (isize, isize)) -> usize {
        self.coords.0.abs_diff(point.0) + self.coords.1.abs_diff(point.1)
    }

    /// Checks sensor borders for holes. There are probably better ways,
    /// but good enough for now.
    fn any_border(
        &self,
        predicate: impl Fn((isize, isize)) -> bool,
        bound: isize,
    ) -> Option<(isize, isize)> {
        let mut x1 = self.coords.0;
        let mut x2 = self.coords.0;
        for dy in (0..=self.closest_dist as isize).rev() {
            let y1 = self.coords.1 - 1 - dy;
            let y2 = self.coords.1 + 1 + dy;

            if x1 >= 0 && x1 <= bound {
                if y1 >= 0 && y1 <= bound && predicate((x1, y1)) {
                    return Some((x1, y1));
                }
                if y2 >= 0 && y2 <= bound && predicate((x1, y2)) {
                    return Some((x1, y2));
                }
            }
            if x2 >= 0 && x2 <= bound {
                if y1 >= 0 && y1 <= bound && predicate((x2, y1)) {
                    return Some((x2, y1));
                }
                if y2 >= 0 && y2 <= bound && predicate((x2, y2)) {
                    return Some((x2, y2));
                }
            }

            x1 -= 1;
            x2 += 1;
        }
        None
    }
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Space {
    let regex =
        Regex::new(r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#)
            .unwrap();
    let mut bounds = (isize::MAX, isize::MIN);
    let sensors = input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let mut sensor = Sensor {
                coords: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                closest_beacon: (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
                closest_dist: 0,
            };
            sensor.closest_dist = sensor.dist_to(sensor.closest_beacon);
            bounds.0 = bounds.0.min(sensor.coords.0).min(sensor.closest_beacon.0);
            bounds.1 = bounds.1.max(sensor.coords.0).max(sensor.closest_beacon.0);
            sensor
        })
        .collect();
    Space {
        sensors,
        x_bounds: bounds,
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &Space) -> usize {
    let y = if input.x_bounds.0.abs() < 100 {
        10
    } else {
        2_000_000
    }; // lmao
    let max_dist = input
        .sensors
        .iter()
        .map(|s| s.dist_to(s.closest_beacon))
        .max()
        .unwrap() as isize;
    (input.x_bounds.0 - max_dist..=input.x_bounds.1 + max_dist)
        .filter(|x| {
            input.sensors.iter().any(|sensor| {
                let point = (*x, y);
                point != sensor.closest_beacon && sensor.closest_dist >= sensor.dist_to(point)
            })
        })
        .count()
}

#[aoc(day15, part2)]
pub fn part2(input: &Space) -> isize {
    let bound = if input.x_bounds.0.abs() < 100 {
        20
    } else {
        4_000_000
    }; // lmao
    let (x, y) = input
        .sensors
        .iter()
        .find_map(|sensor| {
            sensor.any_border(
                |point| {
                    input
                        .sensors
                        .iter()
                        .all(|other| other.closest_dist < other.dist_to(point))
                },
                bound,
            )
        })
        .unwrap();
    x * 4_000_000 + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
        assert_eq!(part1(&parse(input)), 26);
    }

    #[test]
    fn part2_example() {
        let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
        assert_eq!(part2(&parse(input)), 56000011);
    }
}
