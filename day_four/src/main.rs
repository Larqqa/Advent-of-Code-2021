fn get_input() -> (Vec<u32>, Vec<Vec<(bool, u32)>>) {
    let mut input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let boards: Vec<Vec<(bool, u32)>> = input
        .split_off(1)
        .iter()
        .map(|s| s.replace("  ", " ").replace("\n", " "))
        .map(|s| {
            s.split_whitespace()
                .map(|i| (false, i.parse().unwrap()))
                .collect()
        })
        .collect();

    let numbers: Vec<u32> = input
        .get(0)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    (numbers, boards)
}

fn part_one() -> u32 {
    let width = 5;
    let height = width;
    let mut i = 0;
    let _x = i % width;
    let _y = i / width;
    let (numbers, mut boards) = get_input();
    let mut board_index = 0;
    let mut hit_count = 0;

    while i < numbers.len() {
        boards = boards
            .iter()
            .map(|board| {
                board
                    .iter()
                    .map(|num| {
                        if num.1 == numbers[i] {
                            (true, num.1)
                        } else {
                            (num.0, num.1)
                        }
                    })
                    .collect()
            })
            .collect::<Vec<Vec<(bool, u32)>>>();

        board_index = 0;
        for board in &boards {
            for i in 0..width {
                let mut h_acc = 0;
                let mut v_acc = 0;

                for j in 0..height {
                    let h_index = (j * width) + i;
                    let v_index = (i * width) + j;

                    if board.get(h_index).unwrap().0 == true {
                        h_acc += 1;
                    }
                    if board.get(v_index).unwrap().0 == true {
                        v_acc += 1;
                    }
                }

                if h_acc == 5 || v_acc == 5 {
                    hit_count += 1;
                    break;
                }
            }

            if hit_count == 1 {
                break;
            }

            board_index += 1;
        }

        if hit_count == 1 {
            break;
        }

        i += 1;
    }

    let sum: u32 = boards
        .get(board_index)
        .unwrap()
        .iter()
        .filter(|b| b.0 == false)
        .fold(0, |acc, b| acc + b.1);

    sum * numbers[i]
}

fn part_two() -> u32 {
    let width = 5;
    let height = width;
    let mut i = 0;
    let _x = i % width;
    let _y = i / width;
    let (numbers, mut boards) = get_input();
    let mut board_index: usize = 0;
    let mut hit_count = 0;
    let mut winner_boards: Vec<usize> = Vec::new();

    while i < numbers.len() {
        boards = boards
            .iter()
            .map(|board| {
                board
                    .iter()
                    .map(|num| {
                        if num.1 == numbers[i] {
                            (true, num.1)
                        } else {
                            (num.0, num.1)
                        }
                    })
                    .collect()
            })
            .collect::<Vec<Vec<(bool, u32)>>>();

        board_index = 0;
        for board in &boards {
            if winner_boards.contains(&board_index) {
                board_index += 1;
                continue;
            }

            for i in 0..width {
                let mut h_acc = 0;
                let mut v_acc = 0;

                for j in 0..height {
                    let h_index = (j * width) + i;
                    let v_index = (i * width) + j;

                    if board.get(h_index).unwrap().0 == true {
                        h_acc += 1;
                    }
                    if board.get(v_index).unwrap().0 == true {
                        v_acc += 1;
                    }
                }

                if h_acc == 5 || v_acc == 5 {
                    hit_count += 1;
                    winner_boards.push(board_index);
                    break;
                }
            }

            if hit_count == boards.len() {
                break;
            }

            board_index += 1;
        }

        if hit_count == boards.len() {
            break;
        }

        i += 1;
    }

    let sum: u32 = boards
        .get(board_index)
        .unwrap()
        .iter()
        .filter(|b| b.0 == false)
        .fold(0, |acc, b| acc + b.1);

    sum * numbers[i]
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
