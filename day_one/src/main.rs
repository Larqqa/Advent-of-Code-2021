fn get_nums() -> Vec<i32> {
    return include_str!("input.txt")
        .split('\n')
        .map(|s| s.trim().parse().unwrap())
        .collect();
}

fn part_one() -> i32 {
    let mut increments = 0;
    let mut last_num = i32::MAX;

    for num in get_nums() {
        if last_num < num {
            increments += 1;
        }
        last_num = num;
    }

    increments
}

fn part_two() -> i32 {
    let nums = get_nums();
    let mut last_sum = i32::MAX;
    let mut increments = 0;

    for i in 0..nums.len() {
        if i + 2 == nums.len() {
            break;
        }

        let sum = nums[i] + nums[i + 1] + nums[i + 2];

        if sum > last_sum {
            increments += 1;
        }

        last_sum = sum;
    }

    increments
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
