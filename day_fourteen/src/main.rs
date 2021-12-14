use std::collections::HashMap;

fn get_input() -> (Vec<String>, HashMap<String, String>) {
    let input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();
    let template = input[0].chars().map(|s| s.to_string()).collect();
    let mut insertions: HashMap<String, String> = HashMap::new();

    for line in input[1].lines() {
        let s: Vec<&str> = line.split(" -> ").collect();
        insertions.insert(s[0].to_string(), s[1].to_string());
    }

    (template, insertions)
}

#[rustfmt::skip]
fn count_elements(pairs: HashMap<String, u64>) -> HashMap<String, u64> {
    let mut column_counts: HashMap<String, (u64, u64)> = HashMap::new();

    for (pair, amount) in pairs {
        let elements: Vec<char> = pair.chars().collect();
        column_counts.entry(elements[0].to_string()).or_insert((0, 0)).0 += amount;
        column_counts.entry(elements[1].to_string()).or_insert((0, 0)).1 += amount;
    }

    // Filter out smaller column as overlapping
    column_counts
        .iter()
        .map(|(k, v)| {
            if v.0 > v.1 {
                (k.to_owned(), v.0)
            } else {
                (k.to_owned(), v.1)
            }
        })
        .collect()
}

#[rustfmt::skip]
fn get_min_max(map: HashMap<String, u64>) -> (u64, u64) {
    let min = map.iter().reduce(|a, b| if a.1 < b.1 { a } else { b }).unwrap();
    let max = map.iter().reduce(|a, b| if a.1 > b.1 { a } else { b }).unwrap();
    (*min.1, *max.1)
}

#[rustfmt::skip]
fn make_polymer(iterations: usize) -> u64 {
    let (template, insertions) = get_input();
    let mut pairs: HashMap<String, u64> = HashMap::new();

    for i in template.windows(2) {
        *pairs.entry(i.join("")).or_insert(0) += 1;
    }

    for _ in 0..iterations {
        for (pair, amount) in pairs.clone() {
            if insertions.contains_key(&pair) {
                let insert = insertions.get(&pair).unwrap();
                let elements: Vec<char> = pair.chars().collect();

                *pairs.entry(format!("{}{}", elements[0], insert     )).or_insert(0) += amount;
                *pairs.entry(format!("{}{}", insert,      elements[1])).or_insert(0) += amount;
                *pairs.entry(pair).or_insert(0) -= amount;
            }
        }
    }

    let counts = count_elements(pairs);
    let (min, max) = get_min_max(counts);

    max - min
}

fn main() {
    println!(
        "part one: {}\npart two: {}",
        make_polymer(10),
        make_polymer(40)
    );
}
