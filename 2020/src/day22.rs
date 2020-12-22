use std::{collections::{HashSet, VecDeque, hash_map::DefaultHasher}, hash::Hasher, hash::Hash};
// use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Round {
    p1: VecDeque<usize>,
    p2: VecDeque<usize>
}

impl Round {

    // Better encoder but not really needed and way slower...
    // fn encode(&self) -> String {
    //     format!("{}|{}", 
    //         self.p1.iter().map(|x| x.to_string()).intersperse(",".to_string()).collect::<String>(), 
    //         self.p2.iter().map(|x| x.to_string()).intersperse(",".to_string()).collect::<String>())
    // }

    fn encode(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        return hasher.finish();
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Winner {
    P1, P2
}

pub fn main() {
    let mut file = include_str!("../puzzles/22.txt").split("\n\n");

    let initial = Round {
        p1: file.next().unwrap().split('\n').skip(1).map(|x| x.parse().unwrap()).collect(),
        p2: file.next().unwrap().split('\n').skip(1).map(|x| x.parse().unwrap()).collect()
    };

    println!("Solution to exercise 1: {}", game(initial.clone(), false).1.iter().rev().enumerate().map(|(n, c)| (n + 1) * c).sum::<usize>());
    println!("Solution to exercise 2: {}", game(initial.clone(), true).1.iter().rev().enumerate().map(|(n, c)| (n + 1) * c).sum::<usize>());
}

fn game(mut round: Round, parttwo: bool) -> (Winner, VecDeque<usize>) {
    let mut seen = HashSet::with_capacity(1792);

    while !round.p1.is_empty() && !round.p2.is_empty() {

        // Cannot insert Round itself because 'memory' usage
        if parttwo && !seen.insert(round.encode()) {
            return (Winner::P1, round.p1);
        }

        let c1 = round.p1.pop_front().unwrap();
        let c2 = round.p2.pop_front().unwrap();

        let winner = if parttwo && c1 <= round.p1.len() && c2 <= round.p2.len() {

            // If you add the following line the algorithm breaks. Don't know why though...
            // round.p2.make_contiguous();

            game(Round {
                p1: round.p1.iter().take(c1).map(|x| *x).collect(),
                p2: round.p2.iter().take(c2).map(|x| *x).collect(),
            }, parttwo).0
        } else if c1 > c2 {
            Winner::P1
        } else {
            Winner::P2
        };

        if winner == Winner::P1 {
            round.p1.push_back(c1);
            round.p1.push_back(c2);
        } else {
            round.p2.push_back(c2);
            round.p2.push_back(c1);
        }
    }

    if round.p2.is_empty() {
        (Winner::P1, round.p1)
    } else {
        (Winner::P2, round.p2)
    }
}