#[derive(Debug)]
struct Command {
    command: &'static str,
    amount: i32,
}

fn get_input() -> Vec<Command> {
    return include_str!("input.txt")
        .split('\n')
        .map(|s| {
            let x: Vec<&str> = s.split(' ').collect();
            return Command {
                command: x[0],
                amount: x[1].parse().unwrap(),
            };
        })
        .collect();
}

fn part_one() -> i32 {
    let input = get_input();
    let mut horizontal = 0;
    let mut depth = 0;

    for i in input {
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
    let input = get_input();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for i in input {
        match i.command {
            "forward" => {
                horizontal += i.amount;
                depth += aim * i.amount;
            }
            "up" => {
                aim -= i.amount;
            }
            "down" => {
                aim += i.amount;
            }
            _ => (),
        }
    }

    horizontal * depth
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
