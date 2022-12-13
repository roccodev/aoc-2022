use std::cmp::Ordering;
use std::io::Cursor;
use std::io::Read;

#[derive(Clone, Debug)]
pub enum Item {
    Number(i32),
    List(Vec<Item>),
}

impl Item {
    pub fn into_list(self) -> Self {
        match self {
            v @ Self::List(_) => v,
            n @ Self::Number(_) => Self::List(vec![n]),
        }
    }

    pub fn compare_pair(&self, other: &Item) -> Option<bool> {
        match (self, other) {
            (Self::Number(i1), Self::Number(i2)) => (i1 != i2).then_some(i1 < i2),
            (Self::List(v1), Self::List(v2)) => {
                let mut i = 0;
                loop {
                    match (v1.get(i), v2.get(i)) {
                        (None, Some(_)) => return Some(true),
                        (Some(_), None) => return Some(false),
                        (Some(first), Some(second)) => {
                            if let Some(decision) = first.compare_pair(second) {
                                return Some(decision);
                            }
                        }
                        (None, None) => return None,
                    }
                    i += 1;
                }
            }
            (i @ Self::Number(_), v @ Self::List(_)) => i.clone().into_list().compare_pair(v),
            (v @ Self::List(_), i @ Self::Number(_)) => v.compare_pair(&i.clone().into_list()),
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.compare_pair(other) {
            Some(true) => Some(Ordering::Less),
            Some(false) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Item {}

fn parse_items(cursor: &mut Cursor<&[u8]>, level_list: &mut Vec<Item>) {
    let mut buf = [0u8];
    let mut num = vec![];
    while let Ok(1) = cursor.read(&mut buf) {
        match buf[0] {
            b'[' => {
                let mut new_list = vec![];
                parse_items(cursor, &mut new_list);
                level_list.push(Item::List(new_list));
            }
            c @ b',' | c @ b']' => {
                if !num.is_empty() {
                    level_list.push(Item::Number(
                        num.iter()
                            .map(|&b| b as char)
                            .collect::<String>()
                            .parse()
                            .unwrap(),
                    ));
                }
                num.clear();
                if c == b']' {
                    return;
                }
            }
            n => num.push(n),
        }
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<(Item, Item)> {
    input
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines().map(|line| {
                let mut cursor = Cursor::new(line.as_bytes());
                let mut list = vec![];
                parse_items(&mut cursor, &mut list);
                list[0].clone()
            });
            let first = lines.next().unwrap();
            (first, lines.next().unwrap())
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[(Item, Item)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.0.compare_pair(&pair.1).unwrap())
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[(Item, Item)]) -> usize {
    let div_a = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let div_b = Item::List(vec![Item::List(vec![Item::Number(6)])]);
    let mut items = input
        .iter()
        .cloned()
        .flat_map(|(a, b)| [a, b])
        .collect::<Vec<_>>();
    items.push(div_a.clone());
    items.push(div_b.clone());

    items.sort_unstable();
    (items.iter().position(|i| i == &div_a).unwrap() + 1)
        * (items.iter().position(|i| i == &div_b).unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
        assert_eq!(part1(&parse(input)), 13);
    }

    #[test]
    fn part2_example() {
        let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
        assert_eq!(part2(&parse(input)), 140);
    }
}
