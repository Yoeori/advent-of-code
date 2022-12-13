use std::collections::VecDeque;

pub fn main() {
    let mut start = (0, 0);
    let mut finish = (0, 0);

    let height_map: Vec<Vec<u8>> = include_str!("../puzzles/12.txt").lines().enumerate().map(|(y, l)| l.chars().enumerate().map(|(x, c)| {
        if c == 'S' {
            start = (y, x);
            0
        } else if c == 'E' {
            finish = (y, x);
            25
        } else {
            c as u8 - 97
        }
    }).collect()).collect();

    println!("Exercise 1: {}", bfs(&height_map, finish, Some(start)).unwrap());
    println!("Exercise 2: {}", bfs(&height_map, finish, None).unwrap());
}

fn bfs(height_map: &Vec<Vec<u8>>, from: (usize, usize), to: Option<(usize, usize)>) -> Option<usize> {
    let mut dist = vec![vec![usize::MAX; height_map[0].len()]; height_map.len()];
    dist[from.0][from.1] = 0;

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(from);

    while let Some(pos) = queue.pop_front() {
        if (to.is_some() && to.unwrap() == pos) || 
           (to.is_none() && height_map[pos.0][pos.1] == 0) {
            return Some(dist[pos.0][pos.1]);
        }

        'innerloop:
        for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let np = (pos.0 as isize + dx, pos.1 as isize + dy);

            if !(np.0 >= 0 && np.1 >= 0 && np.0 < height_map.len() as isize && np.1 < height_map[0].len() as isize) {
                continue 'innerloop;
            }

            let np = (np.0 as usize, np.1 as usize);

            if height_map[np.0][np.1] + 1 >= height_map[pos.0][pos.1] && dist[np.0][np.1] == usize::MAX {
                dist[np.0][np.1] = dist[pos.0][pos.1] + 1;
                queue.push_back(np);
            }
        }
    }

    None
}