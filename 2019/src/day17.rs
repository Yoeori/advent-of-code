use std::fs;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::cmp;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up, Right, Down, Left
}

impl Direction {

    fn diff(&self) -> Pos {
        match self {
            Direction::Up => Pos { x: 0, y: -1},
            Direction::Right => Pos { x: 1, y: 0},
            Direction::Down => Pos { x: 0, y: 1},
            Direction::Left => Pos { x: -1, y: 0},

        }
    }

    fn apply_instruction(&self, instr: Instr) -> Direction {
        match instr {
            Instr::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            Instr::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            Instr::Forward(_) => self.clone()
        }
    }

}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Instr {
    Left, Right, Forward(i32)
}

impl Instr {
    fn to_char_list(&self) -> Vec<char> {
        match self {
            Instr::Left => vec!['L'],
            Instr::Right => vec!['R'],
            Instr::Forward(num) => num.to_string().chars().collect()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
struct Pos {
    x: i64,
    y: i64
}

impl Pos {
    fn add(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Pos) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            x => x
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Void, Scaffold, Droid(Direction)
}

impl Tile {

    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '.' => Ok(Tile::Void),
            '#' => Ok(Tile::Scaffold),
            '>' => Ok(Tile::Droid(Direction::Right)),
            '^' => Ok(Tile::Droid(Direction::Up)),
            '<' => Ok(Tile::Droid(Direction::Left)),
            'v' => Ok(Tile::Droid(Direction::Down)),
            _ => Err(format!("Invalid char to cast to Tile: {} and as number: {}", c, c as u8))
        }
    }
}

impl From<&Tile> for char {
    fn from(t: &Tile) -> Self {
        match t {
            Tile::Void => '.',
            Tile::Droid(Direction::Right) => '>',
            Tile::Droid(Direction::Up) => '^',
            Tile::Droid(Direction::Left) => '<',
            Tile::Droid(Direction::Down) => 'v',
            Tile::Scaffold => '#'
        }
    }
}

#[derive(Debug)]
struct VacuumRobot {
    program: Vec<i64>,
    map: BTreeMap<Pos, Tile>,
    next_pos: Pos,
    current_route: Option<HashMap<char, VecDeque<char>>>,
    interactive: bool
}

impl VacuumRobot {
    fn run(&mut self) {
        super::intcode_computer::simulate_computer_with_read_write(self.program.clone(), self);
    }

    #[allow(dead_code)]
    fn to_screen(&self) -> String {
        let mut output = String::new();
        let mut last_y = -1;

        for (pos, tile) in &self.map {
            if pos.y != last_y {
                last_y = pos.y;
                output.push('\n');
            }

            output.push(char::from(tile));
        }

        return output;
    }

    fn alignment_parameter(&self) -> i64 {
        self.map.iter().filter(|&(pos, tile)| 
            tile == &Tile::Scaffold &&
            self.map.get(&pos.add(&Pos {x: 1, y: 0})).unwrap_or(&Tile::Void) == &Tile::Scaffold &&
            self.map.get(&pos.add(&Pos {x: -1,y: 0})).unwrap_or(&Tile::Void) == &Tile::Scaffold &&
            self.map.get(&pos.add(&Pos {x: 0, y: 1})).unwrap_or(&Tile::Void) == &Tile::Scaffold &&
            self.map.get(&pos.add(&Pos {x: 0, y:-1})).unwrap_or(&Tile::Void) == &Tile::Scaffold
        ).map(|(pos, _)| pos.x * pos.y).sum()
    }

    fn find_path(&self) -> Vec<Instr> {

        // Helper function for recursive defenition
        fn path_helper(map: &BTreeMap<Pos, Tile>, pos: &Pos, dir: &Direction) -> Vec<Instr> {

            // println!("Called path helper: {:?}, {:?}", pos, dir);

            fn get_len(map: &BTreeMap<Pos, Tile>, pos: &Pos, dir: &Direction) -> (i32, Pos) {
                let new_pos = pos.add(&dir.diff());

                if let Some(Tile::Scaffold) = map.get(&new_pos) {
                    let (total, pos) = get_len(&map, &new_pos, &dir);
                    (total + 1, pos)
                } else {
                    (1, pos.clone())
                }
            }
            
            // We can either turn left or right
            if let Some((pos, Tile::Scaffold)) = map.get_key_value(&pos.add(&dir.apply_instruction(Instr::Left).diff())) {
                
                let dir = dir.apply_instruction(Instr::Left);
                let (length, pos) = get_len(&map, &pos, &dir);

                let mut res = path_helper(&map, &pos, &dir);
                res.push(Instr::Forward(length));
                res.push(Instr::Left);
                res

            } else if let Some((pos, Tile::Scaffold)) = map.get_key_value(&pos.add(&dir.apply_instruction(Instr::Right).diff())) {

                let dir = dir.apply_instruction(Instr::Right);
                let (length, pos) = get_len(&map, &pos, &dir);

                let mut res = path_helper(&map, &pos, &dir);
                res.push(Instr::Forward(length));
                res.push(Instr::Right);
                res

            } else {
                // End of path return empty vec.
                return vec![];
            }

        }

        // First we get our start position
        let start = self.map.iter().find(|(_, tile)| if let Tile::Droid(_) = tile { true } else { false });

        if let Some((pos, Tile::Droid(direction))) = start {
            let mut res = path_helper(&self.map, pos, direction);
            res.reverse();
            return res;
        } else {
            // No path
            return vec![];
        }
    }

    fn path_to_movement_functions(&self, path: Vec<Instr>) -> HashMap<char, VecDeque<char>> {
        
        #[derive(Debug, PartialEq, Eq, Clone, Hash)]
        enum PackNum {
            MAIN, A, B, C
        }

        impl PackNum {
            fn next(&self) -> Option<Self> {
                match self {
                    PackNum::MAIN => None,
                    PackNum::A => Some(PackNum::B),
                    PackNum::B => Some(PackNum::C),
                    PackNum::C => Some(PackNum::MAIN)
                }
            }
        }

        impl From<PackNum> for char {
            fn from(p: PackNum) -> Self {
                match p {
                    PackNum::MAIN => 'M',
                    PackNum::A => 'A',
                    PackNum::B => 'B',
                    PackNum::C => 'C'
                }
            }
        }

        #[derive(Debug, PartialEq, Eq, Clone, Hash)]
        enum Pack {
            Packed(PackNum), Char(char)
        }

        impl From<Pack> for char {
            fn from(p: Pack) -> Self {
                match p {
                    Pack::Packed(num) => char::from(num),
                    Pack::Char(c) => c
                }
            }
        }

        

        // Find first packable Char containing list and return packed list
        fn packing(list: Vec<Pack>, cur_packnum: PackNum) -> Option<HashMap<PackNum, Vec<Pack>>> {

            fn pack(list: &[Pack], to_pack: Vec<Pack>, with: &PackNum) -> Vec<Pack> {
                let mut res = vec![];
                let mut i = 0;
                while i < to_pack.len() {
                    if i+list.len() > to_pack.len() {
                        res.push(to_pack[i].clone());
                    } else if &to_pack[i..(i+list.len())] == list {
                        res.push(Pack::Packed(with.clone()));
                        i += list.len()-1;
                    } else {
                        res.push(to_pack[i].clone());
                    }

                    i += 1;
                }

                return res;
            }

            if cur_packnum == PackNum::MAIN {
                return if list.len() <= 20 {
                    let mut result = HashMap::new();
                    result.insert(PackNum::MAIN, list);
                    Some(result)
                } else {
                    None
                }
            }

            let start = list.iter().enumerate().skip_while(|(_, pack)| matches!(pack, Pack::Packed(_) | Pack::Char(','))).map(|(i, _)| i).next();

            if start.is_none() {
                return Some(HashMap::new());
            }

            let start = start.unwrap();
            let mut result: Option<HashMap<PackNum, Vec<Pack>>> = None;

            for i in start..cmp::min(list.len(), start + 20) {

                if let Some(Pack::Packed(_)) = list.get(i) {
                    break;
                } else if let Some(Pack::Char(',')) = list.get(i) {
                    continue;
                } else if let Some(Pack::Char(',')) = list.get(i+1) {

                    if i - start < 2 {
                        continue;
                    }

                    let sublist = &list[start..=i];
                    let res = pack(sublist, list.clone(), &cur_packnum);
                    let rest = packing(res, cur_packnum.next().unwrap());

                    if let Some(mut rest) = rest {
                        rest.insert(cur_packnum.clone(), list.clone().into_iter().skip(start).take(sublist.len()).collect());

                        if let None = result {
                            result = Some(rest);
                        } else if let Some(act_res) = &result {
                            if rest.iter().map(|(_, v)| v.len()).sum::<usize>() < act_res.iter().map(|(_, v)| v.len()).sum() {
                                result = Some(rest);
                            }
                        }

                    } else {
                        continue;
                    }

                }
            }

            result
        }

        // Convert path to char array
        let mut path = path.iter().map(|instr| {
            let mut res = instr.to_char_list();
            res.push(',');
            res
        }).flatten().map(|c| Pack::Char(c)).collect::<Vec<Pack>>();
        path.pop();

        
        return packing(path.clone(), PackNum::A).unwrap().into_iter()
               .map(|(k, mut v)| { v.push(Pack::Char('\n')); (k, v) })
               .map(|(k, v)| (char::from(k), v.into_iter()
                                              .map(|c| char::from(c))
                                              .collect::<VecDeque<char>>())
               ).collect::<HashMap<char, VecDeque<char>>>();

    }

}

impl Default for VacuumRobot {
    fn default() -> Self {
        VacuumRobot {
            program: vec![],
            map: BTreeMap::new(),
            next_pos: Pos { x: 0, y:0 },
            current_route: None,
            interactive: false
        }
    }   
}

impl super::intcode_computer::ReadWrite for VacuumRobot {
    fn input(&mut self) -> i64 {
        
        // Check if current route:
        if self.current_route.is_none() {
            self.current_route = Some(self.path_to_movement_functions(self.find_path()));
            self.current_route.as_mut().map(|r| { let mut l = VecDeque::new(); 
                                                  l.push_back('n');
                                                  l.push_back('\n');
                                                  r.insert('F', l); 
                                                  r 
                                                });
        }

        if let Some(route) = &mut self.current_route {
            
            let chars = &['M', 'A', 'B', 'C', 'F'];

            for c in chars {
                if let Some(v) = route.get_mut(&c) {
                    if let Some(c) = v.pop_front() {
                        print!("{}", c);
                        return c as u8 as i64;
                    }
                }
            }

            return 0;

        } else {
            panic!();
        };

    }

    fn output(&mut self, output: i64) {
        let c = output as u8 as char;
        let tile = Tile::from_char(c);

        if !self.interactive {
            if let Ok(tile) = tile {
                self.map.insert(self.next_pos.clone(), tile);
                self.next_pos = self.next_pos.add(&Pos {x: 1, y: 0})
            } else if c == '\n' {
                    self.next_pos = Pos { x: 0, y: self.next_pos.y + 1 };
            } else {
                print!("{}", c);
            }
        } else {
            if output > 256 {
                println!("Output: {}", output);
            } else {
                print!("{}", c);
            }
        }
        
    }
}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/17.txt").unwrap();

    let mut robot = VacuumRobot {
        program: file_contents.split(",").map(|x| x.parse().unwrap()).collect(),
        ..Default::default()
    };

    robot.run();
    println!("Answer to exercise 1: {}", robot.alignment_parameter());


    // Second exercise
    robot.program[0] = 2;
    robot.interactive = true;
    robot.run();
}