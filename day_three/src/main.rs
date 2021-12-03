fn part_one() -> u32 {
    isize::from_str_radix(
        include_str!("input.txt")
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .fold(Vec::new(), |mut sol, b| {
                let j: Vec<char> = b.chars().collect();
                for p in 0..j.len() {
                    let c = j[p].to_digit(10).unwrap();
                    if sol.get(p) == None {
                        sol.push((0, 0));
                    }
                    if c == 1 {
                        sol[p] = (sol[p].0 + 1, sol[p].1);
                    } else {
                        sol[p] = (sol[p].0, sol[p].1 + 1);
                    }
                }
                sol
            })
            .iter()
            .map(|s| if s.1 > s.0 { "1" } else { "0" })
            .collect::<Vec<&str>>()
            .join("")
            .as_str(),
        2,
    )
    .unwrap() as u32
}

fn part_two(mut list: Vec<&str>, flip: bool) -> isize {
    let mut i: usize = 0;
    while list.len() > 1 {
        let sol = list.iter().fold((0, 0), |mut a, b| {
            if b.chars().collect::<Vec<char>>()[i].to_digit(10).unwrap() == 1 {
                a.0 += 1
            } else {
                a.1 += 1
            }
            a
        });
        if list.len() > 1 {
            list = list
                .iter()
                .filter(|s| {
                    s.chars().collect::<Vec<char>>()[i].to_digit(10).unwrap()
                        == if (sol.0 >= sol.1) == flip { 1 } else { 0 }
                })
                .cloned()
                .collect();
        }
        i += 1;
    }

    isize::from_str_radix(list[0], 2).unwrap()
}

fn main() {
    let input = part_one();
    println!("part one: {}", (0b111111111111 ^ input) * input);

    let input = include_str!("input.txt").lines().collect::<Vec<&str>>();
    println!(
        "part two: {}",
        part_two(input.clone(), true) * part_two(input, false)
    );
}
