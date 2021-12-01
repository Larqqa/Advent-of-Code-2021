fn get_nums() -> Vec<i32> {
    return include_str!("input.txt")
        .split('\n')
        .map(|s| s.trim().parse().unwrap())
        .collect();
}

fn part_one() -> i32 {
    let nums = get_nums();
    let mut increments = 0;

    for i in 0..nums.len() - 1 {
        if nums[i] < nums[i + 1] {
            increments += 1;
        }
    }

    increments
}

fn part_two() -> i32 {
    let nums = get_nums();
    let mut increments = 0;

    for i in 0..nums.len() - 3 {
        if nums[i] < nums[i + 3] {
            increments += 1;
        }
    }

    increments
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());

    // Uber optimized solutions
    println!(
        "part one: {}",
        include_str!("input.txt")
            .split('\n')
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|a| a[0] < a[1])
            .count()
    );
    println!(
        "part two: {}",
        include_str!("input.txt")
            .split('\n')
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<i32>>()
            .windows(4)
            .filter(|a| a[0] < a[3])
            .count()
    );
}
