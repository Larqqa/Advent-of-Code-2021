fn get_input() -> Vec<i32> {
    return include_str!("input.txt")
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();
}

use std::collections::HashMap;
fn part_one() -> i32 {
    let input = get_input();
    let sum = input.iter().max().unwrap();

    let mut r = HashMap::new();
    for n in 0..*sum {
        *r.entry(n).or_insert(0) += input
            .iter()
            .fold(0, |a, b| a + if *b < n { n - b } else { b - n });
    }

    let mut min = (0, *r.get(&0).unwrap());
    for (k, v) in r {
        if v < min.1 {
            min = (k, v);
        }
    }

    min.1
}

fn part_two() -> i32 {
    let input = get_input();
    let sum = input.iter().max().unwrap();

    let mut r = HashMap::new();
    for n in 0..*sum {
        *r.entry(n).or_insert(0) += input.iter().fold(0, |a, b| {
            let c = if *b < n { n - b } else { b - n };
            let mut d = 0;
            let mut p = 1;
            for n in 0..c {
                d += p;
                p += 1;
            }

            a + d
        });
    }

    let mut min = (0, *r.get(&0).unwrap());
    for (k, v) in r {
        if v < min.1 {
            min = (k, v);
        }
    }

    min.1
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
