use std::collections::HashMap;
fn generate_population(days: i32) -> i64 {
    let mut map: HashMap<i64, i64> =
        include_str!("input.txt")
            .split(",")
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c.parse().unwrap()).or_insert(0) += 1;
                acc
            });

    let weeks = days / 7;
    let remainder = days % 7;
    let mut temp: HashMap<i64, i64> = HashMap::new();

    for _ in 0..weeks {
        for (key, value) in &map {
            match key {
                0 => *temp.entry(2).or_insert(0) += value,
                1 => *temp.entry(3).or_insert(0) += value,
                2 => *temp.entry(4).or_insert(0) += value,
                3 => *temp.entry(5).or_insert(0) += value,
                4 => *temp.entry(6).or_insert(0) += value,
                5 => *temp.entry(7).or_insert(0) += value,
                6 => *temp.entry(8).or_insert(0) += value,
                7 => *temp.entry(0).or_insert(0) += value,
                8 => *temp.entry(1).or_insert(0) += value,
                _ => (),
            }
        }

        for (key, value) in &temp {
            if key == &7 || key == &8 {
                *map.entry(*key).or_insert(0) = *value;
            } else {
                *map.entry(*key).or_insert(0) += *value;
            }
        }

        temp.clear();
    }

    let remaining_pop = map
        .iter()
        .filter(|(a, _)| a < &&(remainder as i64))
        .fold(0, |a, (_, b)| a + b);

    let weekly_pop = map.iter().fold(0, |a, (_, b)| a + b);

    weekly_pop + remaining_pop
}

fn main() {
    println!("part one: {}", generate_population(80));
    println!("part two: {}", generate_population(256));

    generate_population(14);
}
