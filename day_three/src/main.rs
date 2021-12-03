fn get_averaged_decimal(input: &Vec<f32>, length: f32, rounding: f32) -> isize {
    return isize::from_str_radix(
        input
            .iter()
            .map(|&s| {
                (rounding - (s / length))
                    .round()
                    .to_string()
                    .replace('-', "") // signed to unsigned, lol
            })
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
        2,
    )
    .unwrap();
}

fn filter_columns(input: &Vec<&str>, index: usize, flip: bool) -> isize {
    if input.len() == 1 {
        return isize::from_str_radix(input[0], 2).unwrap();
    }

    let mut filter = (input.iter().fold(0.0, |mut acc, byte| {
        let bits: Vec<f32> = byte
            .chars()
            .map(|c| c.to_digit(10).unwrap() as f32)
            .collect();
        acc += bits[index];
        acc
    }) / input.len() as f32)
        .round();
    if flip {
        filter = if filter == 1.0 { 0.0 } else { 1.0 };
    }

    filter_columns(
        &input
            .iter()
            .filter(|j| {
                let p: Vec<char> = j.chars().collect();
                p.get(index).unwrap().to_digit(10).unwrap() as f32 == filter
            })
            .cloned()
            .collect(),
        index + 1,
        flip,
    )
}

fn part_one() -> isize {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let averages: Vec<f32> = input.iter().fold(Vec::new(), |mut acc, byte| {
        let bits: Vec<f32> = byte
            .chars()
            .map(|c| c.to_digit(10).unwrap() as f32)
            .collect();
        for i in 0..bits.len() {
            if acc.get(i) == None {
                acc.push(bits[i])
            } else {
                acc[i] += bits[i];
            }
        }

        acc
    });

    let gamma = get_averaged_decimal(&averages, input.len() as f32, 0.0);
    let epsilon = get_averaged_decimal(&averages, input.len() as f32, 1.0);

    gamma * epsilon
}

fn part_two() -> isize {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let oxygen = filter_columns(&input, 0, false);
    let co_two = filter_columns(&input, 0, true);
    oxygen * co_two
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
