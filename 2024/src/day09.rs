use std::collections::VecDeque;

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Blob {
    File(usize, usize, usize), // pos, id, size
    Free(usize, usize),        // pos, size
}

impl Blob {
    fn pos(&self) -> usize {
        match self {
            Self::File(pos, _, _) => *pos,
            Self::Free(pos, _) => *pos,
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::File(_, _, size) => *size,
            Self::Free(_, size) => *size,
        }
    }

    fn checksum(&self) -> usize {
        match self {
            Self::File(_, id, _) => id * (self.pos()..(self.pos() + self.size())).sum::<usize>(),
            Self::Free(_, _) => 0,
        }
    }
}

struct SegmentTree {
    tree: Vec<(usize, usize)>, // Start position + length
}

impl SegmentTree {
    fn merge(left: (usize, usize), right: (usize, usize)) -> (usize, usize) {
        (left.0.min(right.0), left.1.max(right.1))
    }

    fn new(data: &[(usize, usize)]) -> SegmentTree {
        let tree_len = 2usize.pow((data.len() as f64).log2().ceil() as u32 + 1);
        let mut tree = vec![(usize::MAX, 0); tree_len];

        // Copy original data to end of list
        tree[(tree_len / 2)..(data.len() + tree_len / 2)].clone_from_slice(data);

        // Build tree
        for idx in (1..(tree.len() / 2)).rev() {
            tree[idx] = Self::merge(tree[idx * 2], tree[idx * 2 + 1]);
        }

        SegmentTree { tree }
    }

    fn set(&mut self, mut idx: usize, v: (usize, usize)) {
        self.tree[idx] = v;
        idx >>= 1;
        while idx > 0 {
            self.tree[idx] = Self::merge(self.tree[idx * 2], self.tree[idx * 2 + 1]);
            idx >>= 1;
        }
    }

    // We find the left most possible node where: start position < max_start_pos, and size > min_size
    fn find(&self, limits: (usize, usize), idx: usize) -> Option<usize> {
        let (position, size) = self.tree[idx];

        if position > limits.0 || size < limits.1 {
            return None;
        }

        if idx >= (self.tree.len() / 2) {
            return Some(idx);
        }

        self.find(limits, idx * 2)
            .or_else(|| self.find(limits, idx * 2 + 1))
    }
}

pub(crate) struct Day09;
impl Puzzle for Day09 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        // Create filesystem

        let mut pos = 0;
        let initial_filesystem = inp
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let size = c.to_string().parse::<usize>().unwrap();
                let blob = if i % 2 == 0 {
                    Blob::File(pos, i >> 1, size)
                } else {
                    Blob::Free(pos, size)
                };

                pos += size;
                blob
            })
            .filter(|v| !matches!(v, Blob::Free(_, 0)))
            .collect::<VecDeque<Blob>>();

        let defragged = defrag(initial_filesystem.clone());
        let defragged_size = defragged.iter().map(|blob| blob.checksum()).sum::<usize>();

        let empty_space: Vec<(usize, usize)> = initial_filesystem
            .iter()
            .filter_map(|blob| match *blob {
                Blob::File(_, _, _) => None,
                Blob::Free(pos, size) => Some((pos, size)),
            })
            .collect();

        let mut tree = SegmentTree::new(&empty_space);

        let mut files: Vec<(usize, usize, usize)> = initial_filesystem
            .iter()
            .filter_map(|blob| match *blob {
                Blob::File(pos, id, size) => Some((pos, id, size)),
                Blob::Free(_, _) => None,
            })
            .collect();

        for file in files.iter_mut().rev() {
            // We find the best position in the tree
            let idx = tree.find((file.0, file.2), 1);

            if let Some(idx) = idx {
                let (pos, size) = tree.tree[idx];
                tree.set(idx, (pos + file.2, size - file.2));
                *file = (pos, file.1, file.2)
            }
        }

        let limited_defrag_size = files
            .iter()
            .map(|&file| Blob::File(file.0, file.1, file.2).checksum())
            .sum::<usize>();

        (defragged_size, limited_defrag_size)
    }
}

fn defrag(mut filesystem: VecDeque<Blob>) -> Vec<Blob> {
    let mut output: Vec<Blob> = vec![];

    while filesystem.len() >= 2 {
        let left = filesystem.pop_front().unwrap();

        match left {
            Blob::File(_, _, _) => {
                output.push(left);
            }
            Blob::Free(pos, free_size) => {
                let right = filesystem.pop_back().unwrap();

                match right {
                    Blob::File(_, id, size) if size == free_size => {
                        output.push(Blob::File(pos, id, size));
                    }
                    Blob::File(file_pos, id, size) if (size > free_size) => {
                        output.push(Blob::File(pos, id, free_size));
                        filesystem.push_back(Blob::File(file_pos, id, size - free_size));
                    }
                    Blob::File(_, id, size) if (size < free_size) => {
                        output.push(Blob::File(pos, id, size));
                        filesystem.push_front(Blob::Free(pos + size, free_size - size));
                    }
                    Blob::File(_, _, _) => {}
                    Blob::Free(_, _) => {
                        filesystem.push_front(left);
                    }
                };
            }
        }
    }

    if !filesystem.is_empty() {
        output.push(filesystem.pop_front().unwrap());
    }

    output
}
