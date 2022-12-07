use std::{collections::HashMap, iter::Peekable, convert::identity};

#[derive(Debug, Default)]
struct Folder<'a> {
    size: Option<usize>,
    folders: HashMap<&'a str, Folder<'a>>,
    files: HashMap<&'a str, usize>,
    root: bool
}

impl<'a> Folder<'a> {
    fn size(&mut self) -> usize {
        *self.size.get_or_insert_with(|| self.folders.values_mut().map(|x| x.size()).sum::<usize>() + self.files.values().sum::<usize>())
    }

    fn find_small_dirs(&mut self, list: &mut Vec<usize>) {
        self.folders.values_mut().for_each(|f| f.find_small_dirs(list));

        if self.size() <= 100_000 {
            list.push(self.size());
        }
    }

    fn minmax_dirs(&mut self, min_size: usize) -> Option<usize> {
        self.folders.values_mut().map(|f| f.minmax_dirs(min_size)).filter_map(identity).min().or_else(|| {
            if self.size() >= min_size {
                Some(self.size())
            } else {
                None
            }
        })  
    }

    fn read_from_lines<I>(&mut self, lines: &mut Peekable<I>) where I: Iterator<Item=&'a str> {
        'outerloop: while let Some(peek) = lines.peek() {
            if peek == &"$ cd /" {
                if self.root {
                    lines.next();
                    continue;
                } else {
                    return;
                }
            }

            let line = lines.next().unwrap();
            if line.starts_with("$ cd") {
                let folder = line.split(' ').last().unwrap();
                if folder == ".." {
                    return;
                } else {
                    self.folders.get_mut(folder).unwrap().read_from_lines(lines);
                }
            } else if line == "$ ls" {
                while let Some(peek) = lines.peek() {
                    if peek.starts_with("$") {
                        continue 'outerloop;
                    } else {
                        let (f, name) = lines.next().unwrap().split_once(' ').unwrap();
                        if f == "dir" {
                            self.folders.insert(name, Folder::default());
                        } else {
                            self.files.insert(name, f.parse::<usize>().unwrap());
                        }
                    }
                }
            }
        }
    }
}

pub fn main() {
    let mut lines = include_str!("../puzzles/7.txt").lines().peekable();

    let mut root = Folder {
        root: true,
        ..Default::default()
    };

    root.read_from_lines(&mut lines);

    let mut small_files = vec![];
    root.find_small_dirs(&mut small_files);
    println!("Exercise 1: {}", small_files.iter().sum::<usize>());

    let root_size = root.size();
    println!("Exercise 2: {}", root.minmax_dirs(30_000_000 - (70_000_000 - root_size)).unwrap());
}