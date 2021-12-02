#[derive(Debug)]
struct Command {
    command: &'static str,
    amount: i32,
}

fn get_input() -> Vec<Command> {
    return include_str!("input.txt")
        .lines()
        .map(|s| {
            let mut s2 = s.split_whitespace();
            return Command {
                command: s2.next().unwrap(),
                amount: s2.next().unwrap().parse().unwrap(),
            };
        })
        .collect();
}

fn part_one() -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    get_input().iter();

    for i in get_input() {
        match i.command {
            "forward" => horizontal += i.amount,
            "up" => depth -= i.amount,
            "down" => depth += i.amount,
            _ => (),
        }
    }

    horizontal * depth
}

fn part_two() -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for i in get_input() {
        match i.command {
            "forward" => {
                horizontal += i.amount;
                depth += aim * i.amount;
            }
            "up" => aim -= i.amount,
            "down" => aim += i.amount,
            _ => (),
        }
    }

    horizontal * depth
}

fn p1(mut j: std::slice::Iter<Command>, mut x: i32, mut y: i32) -> i32 {
    let i = j.next().unwrap();
    match i.command {
        "forward" => x += i.amount,
        "up" => y -= i.amount,
        "down" => y += i.amount,
        _ => (),
    }

    if j.len() > 0 {
        p1(j, x, y)
    } else {
        return x * y;
    }
}

fn p2(mut j: std::slice::Iter<Command>, mut x: i32, mut y: i32, mut z: i32) -> i32 {
    let i = j.next().unwrap();
    match i.command {
        "forward" => {
            x += i.amount;
            y += z * i.amount;
        }
        "up" => z -= i.amount,
        "down" => z += i.amount,
        _ => (),
    }
    if j.len() > 0 {
        p2(j, x, y, z)
    } else {
        return x * y;
    }
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
    println!("part one recursive: {}", p1(get_input().iter(), 0, 0));
    println!("part two recursive: {}", p2(get_input().iter(), 0, 0, 0));

    // Optimized methods
    let foo = include_str!("input.txt").lines().map(|s| {
        let mut s2 = s.split_whitespace();
        return (s2.next().unwrap(), s2.next().unwrap().parse().unwrap());
    });

    let i = foo.clone().fold((0, 0), |mut a, b: (&str, i32)| {
        match b.0 {
            "forward" => a.0 += b.1,
            "up" => a.1 -= b.1,
            _ => a.1 += b.1,
        }
        a
    });

    let j = foo.fold((0, 0, 0), |mut a, b: (&str, i32)| {
        match b.0 {
            "forward" => {
                a.0 += b.1;
                a.1 += a.2 * b.1;
            }
            "up" => a.2 -= b.1,
            _ => a.2 += b.1,
        }
        a
    });

    println!("p1: {}\np2: {}", i.0 * i.1, j.0 * j.1);
}
