use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct SpaceObject<'a> {
    name: &'a str,
    orbits: Option<&'a str>
}

pub fn main() {
    let puzzle_contents = fs::read_to_string("puzzles/06.txt").unwrap();
    let lines: Vec<&str> = puzzle_contents.split("\n").collect();

    let mut space_objects: HashMap<&str, SpaceObject> = HashMap::new();
    space_objects.insert("COM", SpaceObject {
        name: "COM",
        orbits: None
    });

    for line in lines {
        let split_line: Vec<&str> = line.split(")").collect();

        space_objects.insert(split_line[1], SpaceObject {
            name: split_line[1],
            orbits: Some(split_line[0])
        });
    }

    let total: u32 = space_objects.iter().map(|(_, space_object)| {
        count_orbits(space_object, &space_objects, &space_objects.get("COM").unwrap())
    }).sum();

    println!("Solution to exercise 1: {}", total);

    let path_to_you = path_to_com(space_objects.get("YOU").unwrap(), &space_objects);
    let path_to_santa = path_to_com(space_objects.get("SAN").unwrap(), &space_objects);

    let mut res = None;
    for element in path_to_you {
        if path_to_santa.contains(&element) {
            res = Some(element);
            break;
        }
    }

    let path_length = count_orbits(&space_objects.get("YOU").unwrap(), &space_objects, res.unwrap()) + 
                      count_orbits(&space_objects.get("SAN").unwrap(), &space_objects, res.unwrap()) - 2;

    println!("Solution to exercise 2: {}", path_length);
}


fn count_orbits(space_object: &SpaceObject, space_objects: &HashMap<&str, SpaceObject>, to: &SpaceObject) -> u32 {
    if space_object == to {
        return 0
    } else {
        return 1 + count_orbits(space_objects.get(space_object.orbits.unwrap()).unwrap(), space_objects, to)
    }
}

fn path_to_com<'a>(space_object: &SpaceObject, space_objects: &'a HashMap<&str, SpaceObject>) -> Vec<&'a SpaceObject<'a>> {
    if space_object.orbits.is_none() {
        return vec![space_objects.get(space_object.name).unwrap()];
    } else {
        let mut objects = vec![space_objects.get(space_object.name).unwrap()];
        objects.extend(path_to_com(space_objects.get(space_object.orbits.unwrap()).unwrap(), space_objects));
        return objects;
    }
}