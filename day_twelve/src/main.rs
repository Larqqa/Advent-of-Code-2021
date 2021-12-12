use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
struct Cave {
    name: String,
    small: bool,
    visited: bool,
    connections: Vec<String>,
}

fn insert_cave(map: &mut HashMap<String, Cave>, cave: String, conn: String) {
    if map.contains_key(&cave) {
        map.get_mut(&cave).unwrap().connections.push(conn);
    } else {
        map.insert(
            cave.clone(),
            Cave {
                name: cave.to_string(),
                small: cave.chars().all(|b| b.is_lowercase()) || cave == "start",
                visited: false,
                connections: vec![conn],
            },
        );
    }
}

fn get_input() -> HashMap<String, Cave> {
    let mut map: HashMap<String, Cave> = HashMap::new();
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let pairs: Vec<Vec<&str>> = input.iter().map(|s| s.split("-").collect()).collect();

    for pair in pairs {
        let cave_f = pair[0].to_string();
        let cave_e = pair[1].to_string();
        insert_cave(&mut map, cave_f.clone(), cave_e.clone());
        insert_cave(&mut map, cave_e, cave_f);
    }

    map
}

fn get_paths(map: &mut HashMap<String, Cave>, cave: String, mut small_visited: bool) -> u32 {
    if cave == String::from("end") {
        return 1;
    }

    let start = map.entry(cave).or_default();
    if start.small && start.visited {
        if start.name == "start" || small_visited {
            return 0;
        }
        small_visited = true;
    }

    start.visited = true;
    start.connections.clone().iter().fold(0, |acc, cave| {
        acc + get_paths(&mut map.clone(), cave.to_owned(), small_visited)
    })
}

fn main() {
    let mut input = get_input();
    println!(
        "part one: {}\npart two: {}",
        get_paths(&mut input.clone(), String::from("start"), true),
        get_paths(&mut input, String::from("start"), false)
    );
}
