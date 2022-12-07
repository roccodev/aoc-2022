use fxhash::FxHashMap;
use std::{collections::VecDeque, rc::Rc};

pub enum Line {
    CdFwd(Name),
    CdBck,
    CdRoot,
    Ls,
    Dir(Name),
    File(Name, usize),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Name(Rc<String>);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct FilePath {
    parts: VecDeque<Name>,
}

#[derive(Default, Debug)]
struct Directory {
    children: FxHashMap<Name, DirEntry>,
}

#[derive(Debug)]
enum DirEntry {
    Directory(Directory),
    File(Name, usize),
}

#[derive(Default)]
struct Terminal {
    current_dir: FilePath,
    visited_dirs: FxHashMap<FilePath, Directory>,
}

impl FilePath {
    pub fn go_back(&mut self) {
        if self.parts.len() > 1 {
            self.parts.pop_back();
        }
    }

    pub fn go_forward(&mut self, child: Name) {
        self.parts.push_back(child);
    }

    pub fn to_forward(&self, child: Name) -> Self {
        let mut new = self.clone();
        new.go_forward(child);
        new
    }
}

impl Terminal {
    pub fn run_cmd(&mut self, line: &Line) {
        match line {
            Line::CdFwd(dir) => self.current_dir.go_forward(dir.clone()),
            Line::CdBck => self.current_dir.go_back(),
            Line::CdRoot => self.current_dir = FilePath::default(),
            Line::Ls => {}
            Line::Dir(name) => {
                self.visited_dirs
                    .entry(self.current_dir.clone())
                    .or_default()
                    .children
                    .insert(name.clone(), DirEntry::Directory(Directory::default()));
            }
            Line::File(name, size) => {
                self.visited_dirs
                    .entry(self.current_dir.clone())
                    .or_default()
                    .children
                    .insert(name.clone(), DirEntry::File(name.clone(), *size));
            }
        }
    }

    pub fn get_size(&self, file_path: &FilePath) -> usize {
        if let Some(dir) = self.visited_dirs.get(file_path) {
            let mut size = 0;
            for (name, child) in &dir.children {
                match child {
                    DirEntry::Directory(_) => {
                        size += self.get_size(&file_path.to_forward(name.clone()))
                    }
                    DirEntry::File(_, sz) => size += *sz,
                }
            }
            return size;
        }
        0
    }
}

impl Default for FilePath {
    fn default() -> Self {
        Self {
            parts: std::iter::once("/".into()).collect(),
        }
    }
}

impl<S: ToString> From<S> for Name {
    fn from(s: S) -> Self {
        Self(Rc::new(s.to_string()))
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            match (words[0], words[1]) {
                ("$", "cd") => match words[2] {
                    ".." => Line::CdBck,
                    "/" => Line::CdRoot,
                    w => Line::CdFwd(w.into()),
                },
                ("$", "ls") => Line::Ls,
                ("dir", name) => Line::Dir(name.into()),
                (size, name) => Line::File(name.into(), size.parse().unwrap()),
            }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[Line]) -> usize {
    let mut terminal = Terminal::default();
    for line in input {
        terminal.run_cmd(line);
    }
    terminal
        .visited_dirs
        .keys()
        .map(|dir| terminal.get_size(dir))
        .filter(|&s| s < 100_000)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[Line]) -> usize {
    let mut terminal = Terminal::default();
    for line in input {
        terminal.run_cmd(line);
    }
    let root_size = terminal.get_size(&FilePath::default());
    let required = root_size - 40_000_000;
    terminal
        .visited_dirs
        .keys()
        .map(|dir| terminal.get_size(dir))
        .filter(|&s| s >= required)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        assert_eq!(part1(&parse(input)), 95437);
    }

    #[test]
    fn part2_example() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        assert_eq!(part2(&parse(input)), 24933642);
    }
}
