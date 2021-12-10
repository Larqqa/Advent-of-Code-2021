fn get_input() -> Vec<Vec<char>> {
    return include_str!("input.txt")
        .lines()
        .map(|s| s.chars().collect())
        .collect();
}

fn find_corrupted(input: Vec<Vec<char>>) -> (Vec<char>, Vec<Vec<char>>) {
    let mut corruptions: Vec<char> = Vec::new();
    let mut incomplete: Vec<Vec<char>> = Vec::new();

    for l in 0..input.len() {
        let mut stack: Vec<char> = Vec::new();
        let cor_len = corruptions.len();
        for c in &input[l] {
            match c {
                '(' => stack.push(*c),
                '[' => stack.push(*c),
                '{' => stack.push(*c),
                '<' => stack.push(*c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        corruptions.push(')')
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        corruptions.push(']')
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        corruptions.push('}')
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        corruptions.push('>')
                    }
                }
                _ => (),
            }
        }

        if corruptions.len() == cor_len {
            incomplete.push(input[l].to_owned());
        }
    }

    (corruptions, incomplete)
}

fn find_completions(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut corruptions: Vec<Vec<char>> = Vec::new();
    for l in 0..input.len() {
        let mut stack: Vec<char> = Vec::new();
        for c in &input[l] {
            match c {
                '(' => stack.push(*c),
                '[' => stack.push(*c),
                '{' => stack.push(*c),
                '<' => stack.push(*c),
                ')' => {
                    stack.pop();
                }
                ']' => {
                    stack.pop();
                }
                '}' => {
                    stack.pop();
                }
                '>' => {
                    stack.pop();
                }
                _ => (),
            }
        }
        corruptions.push(stack);
    }

    corruptions
}

fn part_one() -> u32 {
    let (corrupted, _) = find_corrupted(get_input());
    corrupted
        .iter()
        .map(|a| match a {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum()
}

fn part_two() -> i64 {
    let (_, incomplete) = find_corrupted(get_input());
    let incompletes = find_completions(incomplete);
    let mut sums = Vec::new();
    for mut i in incompletes {
        let mut score: i64 = 0;
        i.reverse();
        for j in i {
            let val = match j {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            };
            score = score * 5 + val;
        }
        sums.push(score);
    }
    sums.sort();
    let index = (sums.len() as f32 / 2.0).floor() as usize;
    sums[index]
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
