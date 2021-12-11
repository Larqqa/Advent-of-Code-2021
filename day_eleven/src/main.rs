fn get_input() -> (Vec<(u8, bool)>, usize, usize) {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    (
        input
            .iter()
            .map(|s| s.chars().map(|i| (i.to_string().parse().unwrap(), false)))
            .flatten()
            .collect(),
        input[0].len(),
        input.len(),
    )
}

fn flash(input: &mut Vec<(u8, bool)>, index: usize, width: usize, amount: &mut u32) {
    let i = index as i8;
    let w = width as i8;
    let len = (input.len() - 1) as i8;
    let top = i - w;
    let bottom = i + w;
    let left = i - 1;
    let right = i + 1;
    let top_left = i - w - 1;
    let top_right = i - w + 1;
    let bottom_left = i + w - 1;
    let bottom_right = i + w + 1;

    // Check if flashed already
    if input[index].1 == true {
        return;
    } else {
        input[index].1 = true;
        *amount += 1;
    }

    // Increment adjacent to flash
    if top >= 0 {
        input[top as usize].0 += 1;
    }

    if bottom <= len {
        input[bottom as usize].0 += 1;
    }

    if left >= 0 && left % w < w - 1 {
        input[left as usize].0 += 1;
    }

    if right <= len && right % w > 0 {
        input[right as usize].0 += 1;
    }

    if top_left >= 0 && top_left % w < w - 1 {
        input[top_left as usize].0 += 1;
    }

    if top_right >= 0 && top_right % w > 0 {
        input[top_right as usize].0 += 1;
    }

    if bottom_left <= len && bottom_left % w < w - 1 {
        input[bottom_left as usize].0 += 1;
    }

    if bottom_right <= len && bottom_right % w > 0 {
        input[bottom_right as usize].0 += 1;
    }

    // Flash adjacent
    if top >= 0 && input[top as usize].0 > 9 {
        flash(input, top as usize, width, amount);
    }

    if bottom <= len && input[bottom as usize].0 > 9 {
        flash(input, bottom as usize, width, amount);
    }

    if left >= 0 && left % w < w - 1 && input[left as usize].0 > 9 {
        flash(input, left as usize, width, amount);
    }

    if right <= len && right % w > 0 && input[right as usize].0 > 9 {
        flash(input, right as usize, width, amount);
    }

    if top_left >= 0 && top_left % w < w - 1 && input[top_left as usize].0 > 9 {
        flash(input, top_left as usize, width, amount);
    }

    if top_right >= 0 && top_right % w > 0 && input[top_right as usize].0 > 9 {
        flash(input, top_right as usize, width, amount);
    }

    if bottom_left <= len && bottom_left % w < w - 1 && input[bottom_left as usize].0 > 9 {
        flash(input, bottom_left as usize, width, amount);
    }

    if bottom_right <= len && bottom_right % w > 0 && input[bottom_right as usize].0 > 9 {
        flash(input, bottom_right as usize, width, amount);
    }
}

fn step(input: &mut Vec<(u8, bool)>, width: usize) -> u32 {
    let mut flash_amount = 0;

    // increase energy by one
    for i in 0..input.len() {
        input[i].0 += 1;
    }

    // Then flash all over 9 power
    for i in 0..input.len() {
        if input[i].0 > 9 {
            flash(input, i, width, &mut flash_amount);
        }
    }

    // Reset the powerful bois
    for i in 0..input.len() {
        input[i].1 = false;
        if input[i].0 > 9 {
            input[i].0 = 0;
        }
    }

    flash_amount
}

fn print_map(input: &Vec<(u8, bool)>, width: usize) {
    let mut temp = String::new();
    for i in 0..input.len() {
        if i % width == 0 {
            temp += "\n";
        }
        temp += input[i].0.to_string().as_str();
    }

    println!("{}", temp);
}

fn part_one() -> u32 {
    let (mut input, width, _) = get_input();
    let mut sum = 0;
    for _ in 0..195 {
        sum += step(&mut input, width);
    }

    sum
}

use std::{thread, time};
fn part_two() -> u32 {
    let (mut input, width, _) = get_input();
    let mut steps = 1;
    while step(&mut input, width) != input.len() as u32 {
        steps += 1;

        // Bonus terminal visualization
        print_map(&input, width);
        thread::sleep(time::Duration::from_millis(70));
        print!("{esc}c", esc = 27 as char);
    }

    steps
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
