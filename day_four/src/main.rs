#[derive(Debug, Default)]
struct Board {
    won: bool,
    last_num: u32,
    rounds: u32,
    width: usize,
    height: usize,
    board: Vec<(u32, bool)>,
}

impl Board {
    fn check_numbers(&mut self, nums: &Vec<u32>) {
        for num in nums {
            if self.won {
                break;
            }
            self.last_num = *num;
            self.board = self
                .board
                .iter()
                .map(|b| if b.0 == *num { (b.0, true) } else { *b })
                .collect();
            self.check_winning_rows();
        }
    }

    fn check_winning_rows(&mut self) {
        self.rounds += 1;
        for i in 0..self.width {
            let mut h_acc = 0;
            let mut v_acc = 0;

            for j in 0..self.height {
                if self.board.get((j * self.width) + i).unwrap().1 == true {
                    h_acc += 1;
                }
                if self.board.get((i * self.width) + j).unwrap().1 == true {
                    v_acc += 1;
                }
            }

            if h_acc == self.width || v_acc == self.height {
                self.won = true;
                break;
            }
        }
    }

    fn sum_remaining(&self) -> u32 {
        let sum: u32 = self
            .board
            .iter()
            .filter(|b| b.1 == false)
            .fold(0, |acc, b| acc + b.0);

        sum * self.last_num
    }
}

fn get_inputs() -> (Vec<u32>, Vec<Board>) {
    let mut input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let boards: Vec<Board> = input
        .split_off(1)
        .iter()
        .map(|s| s.replace("  ", " ").replace("\n", " "))
        .map(|s| {
            let board = s
                .split_whitespace()
                .map(|i| (i.parse().unwrap(), false))
                .collect();
            Board {
                board,
                width: 5,
                height: 5,
                ..Default::default()
            }
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

fn main() {
    let (num, mut boa) = get_inputs();

    for i in 0..boa.len() {
        boa[i].check_numbers(&num);
    }

    let first = boa
        .iter()
        .reduce(|a, b| if a.rounds < b.rounds { a } else { b })
        .unwrap()
        .sum_remaining();
    let last = boa
        .iter()
        .reduce(|a, b| if a.rounds > b.rounds { a } else { b })
        .unwrap()
        .sum_remaining();

    println!("part one: {:?}\n part two: {}", first, last);
}
