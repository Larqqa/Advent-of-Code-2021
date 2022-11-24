#[derive(Debug, Clone)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn make_range(range: &str) -> Range {
        let c: Vec<&str> = range.split("=").collect();
        let d: Vec<i32> = c[1].split("..").map(|k| k.parse().unwrap()).collect();
        Range {
            min: d[0],
            max: d[1],
        }
    }
}

#[derive(Debug, Clone)]
struct Target {
    x: Range,
    y: Range,
}

use std::collections::HashMap;
fn find_y(from: i32, to: i32, target_min: i32, target_max: i32) -> (HashMap<i32, Vec<i32>>, i32) {
    let mut steps: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut all_max = 0;
    for y in from..to {
        let mut step = 0;
        let mut y_step = 0;
        let mut y_vel = y;

        while y_step >= 0 || y_step >= target_min {
            if y_step >= target_min && y_step <= target_max {
                steps.entry(step).or_insert(Vec::new()).push(y);
            }
            if y_step > all_max {
                all_max = y_step;
            }
            step += 1;
            y_step += y_vel;
            y_vel -= 1;
        }
    }

    (steps, all_max)
}

fn find_x(from: i32, to: i32, target_min: i32, target_max: i32) -> HashMap<i32, Vec<i32>> {
    let mut steps: HashMap<i32, Vec<i32>> = HashMap::new();

    for x in from..to {
        let mut step = 0;
        let mut x_step = 0;
        let mut x_vel = x;

        while x_vel >= 0 {
            if x_step >= target_min && x_step <= target_max {
                if x_vel == 0 {
                    steps.entry(i32::MAX).or_insert(Vec::new()).push(x);
                } else {
                    steps.entry(step).or_insert(Vec::new()).push(x);
                }
            }

            step += 1;
            x_step += x_vel;
            x_vel -= 1;
        }
    }

    steps
}

fn part_two(y_map: &HashMap<i32, Vec<i32>>, x_map: &HashMap<i32, Vec<i32>>) -> usize {
    let mut pairs: Vec<(i32, i32)> = Vec::new();
    for (k, v) in y_map {
        let mut m: Vec<i32> = Vec::new();

        if x_map.get(&k) != None {
            for x in x_map.get(&k).unwrap() {
                m.push(*x);
            }
        } else {
            if x_map.get(&i32::MAX) != None {
                for x in x_map.get(&i32::MAX).unwrap() {
                    m.push(*x);
                }
            }
        }

        for y in v {
            for x in &m {
                pairs.push((*x, *y));
            }
        }
    }

    pairs.sort();
    pairs.dedup();
    pairs.len()
}

fn main() {
    let input = include_str!("input.txt").replace("target area: ", "");
    let targets: Vec<&str> = input.split(", ").collect();
    let target = Target {
        x: Range::make_range(targets[0]),
        y: Range::make_range(targets[1]),
    };
    let (y_map, max) = find_y(target.y.min, target.y.min * -1, target.y.min, target.y.max);
    let x_map = find_x(0, target.x.max + 1, target.x.min, target.x.max);

    println!("part one: {}", max);
    println!("part two: {}", part_two(&y_map, &x_map));
}
