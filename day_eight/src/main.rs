fn get_input() -> Vec<(Vec<&'static str>, Vec<&'static str>)> {
    return include_str!("input.txt")
        .lines()
        .map(|s| {
            let c = s.split(" | ").collect::<Vec<&str>>();
            (
                c[0].split_whitespace().collect(),
                c[1].split_whitespace().collect(),
            )
        })
        .collect();
}

fn part_one() -> usize {
    get_input().iter().fold(0, |a, b| {
        a + b
            .1
            .iter()
            .filter(|b| match b.len() {
                2 => true,
                4 => true,
                3 => true,
                7 => true,
                _ => false,
            })
            .count()
    })
}

fn solve_pair(digits: Vec<char>, nums: &Vec<&str>) -> (char, char) {
    let mut output = ('\0', '\0');
    for n in nums {
        let mut matches = 0;
        let mut i = 0;

        for z in 0..2 {
            if n.contains(digits[z]) {
                matches += 1;
            } else {
                i = z;
            }
        }

        if matches == 1 {
            output = (digits[i], digits[if i == 0 { 1 } else { 0 }]);
        }
    }
    output
}

fn part_two() -> usize {
    //  0000
    // 1    2
    // 1    2
    //  3333
    // 4    5z
    // 4    5
    //  6666

    let mut mapping = vec!['\0'; 7];
    let input = get_input();
    let mut sum: usize = 0;
    let mut one = "";
    let mut four = "";
    let mut seven = "";
    let mut eight = "";
    let mut zerosixnine: Vec<&str> = Vec::new();

    for j in input {
        for n in &j.0 {
            match n.len() {
                2 => one = n,
                3 => seven = n,
                4 => four = n,
                7 => eight = n,
                6 => zerosixnine.push(n),
                _ => (),
            }
        }

        let mut twofive: Vec<char> = Vec::new();
        for c in seven.chars() {
            if !one.contains(c) {
                mapping[0] = c;
            } else {
                twofive.push(c);
            }
        }

        let mask = twofive
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");
        let mut onethree: Vec<char> = Vec::new();
        for c in four.chars() {
            if !mask.contains(c) {
                onethree.push(c);
            }
        }

        let mut mask = onethree.clone();
        mask.append(&mut twofive.clone());
        mask.push(mapping[0]);
        let mut foursix: Vec<char> = Vec::new();
        for c in eight.chars() {
            if !mask.contains(&c) {
                foursix.push(c);
            }
        }

        let mut solved_pair = solve_pair(twofive, &zerosixnine);
        mapping[2] = solved_pair.0;
        mapping[5] = solved_pair.1;
        solved_pair = solve_pair(onethree, &zerosixnine);
        mapping[3] = solved_pair.0;
        mapping[1] = solved_pair.1;
        solved_pair = solve_pair(foursix, &zerosixnine);
        mapping[4] = solved_pair.0;
        mapping[6] = solved_pair.1;

        let two = format!(
            "{}{}{}{}{}",
            mapping[0], mapping[2], mapping[3], mapping[4], mapping[6]
        );
        let five = format!(
            "{}{}{}{}{}",
            mapping[0], mapping[1], mapping[3], mapping[5], mapping[6]
        );
        let six = format!(
            "{}{}{}{}{}{}",
            mapping[0], mapping[1], mapping[3], mapping[4], mapping[5], mapping[6]
        );
        let nine = format!(
            "{}{}{}{}{}{}",
            mapping[0], mapping[1], mapping[2], mapping[3], mapping[5], mapping[6]
        );

        let mut output = String::new();
        for k in &j.1 {
            match k.len() {
                2 => output.push('1'),
                3 => output.push('7'),
                4 => output.push('4'),
                7 => output.push('8'),
                5 => {
                    if k.chars().all(|k| two.contains(k)) {
                        output.push('2')
                    } else if k.chars().all(|k| five.contains(k)) {
                        output.push('5')
                    } else {
                        output.push('3')
                    }
                }
                6 => {
                    if k.chars().all(|k| six.contains(k)) {
                        output.push('6')
                    } else if k.chars().all(|k| nine.contains(k)) {
                        output.push('9')
                    } else {
                        output.push('0')
                    }
                }
                _ => (),
            }
        }

        sum += output.parse::<usize>().unwrap();
    }

    sum
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
