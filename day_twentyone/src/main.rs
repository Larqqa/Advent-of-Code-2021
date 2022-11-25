#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct Player {
    id: u32,
    position: u32,
    score: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Players {
    one: Player,
    two: Player,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct DeterministicDice {
    number: u32,
    rolls: u32,
}
#[allow(dead_code)]
impl DeterministicDice {
    fn gen() -> DeterministicDice {
        DeterministicDice {
            number: 0,
            rolls: 0,
        }
    }
    fn roll(&mut self) -> u32 {
        self.number += 1;
        if self.number >= 100 {
            self.number = 0
        }
        self.rolls += 1;
        self.number
    }
}

#[allow(dead_code)]
struct Game {
    board: Vec<u32>,
    dirac: Vec<u32>,
}
#[allow(dead_code)]
impl Game {
    fn gen() -> Game {
        Game {
            board: (1..=10).collect(),
            dirac: vec![
                3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
            ],
        }
    }

    fn do_move(&self, player: &mut Player, dice: &mut DeterministicDice) -> bool {
        let mut total = player.position;
        for _ in 0..3 {
            total += dice.roll();
        }

        player.position = total % self.board.len() as u32;
        player.score = player.score + self.board[player.position as usize];

        if player.score >= 1000 {
            true
        } else {
            false
        }
    }

    fn do_dirac(&self, player: Player) -> (u32, Vec<Player>) {
        let mut wins = 0;
        let mut unfinished: Vec<Player> = vec![];

        for roll in self.dirac.iter() {
            let mut p = player.clone();
            // println!("p:{} s:{}", p.position, p.score);
            p.position = (p.position + (roll)) % self.board.len() as u32;
            p.score = p.score + self.board[p.position as usize];

            // println!("p:{} s:{}", p.position, p.score);
            // println!("-----");
            if p.score >= 21 {
                wins += 1
            } else {
                unfinished.push(p)
            }
        }

        (wins, unfinished)
    }
}

fn get_input() -> Players {
    let s: Vec<Player> = include_str!("test.txt")
        .lines()
        .map(|x| {
            let a: Vec<&str> = x.split("starting position").collect();
            Player {
                id: a[0].replace("Player", "").trim().parse().unwrap(),
                position: a[1].replace(":", "").trim().parse::<u32>().unwrap() - 1,
                score: 0,
            }
        })
        .collect();

    Players {
        one: s[0],
        two: s[1],
    }
}

fn part_one() -> u32 {
    let mut input = get_input();
    let game = Game::gen();
    let mut dice = DeterministicDice::gen();

    // println!("{:?}", input);
    let mut end = false;
    while !end {
        end = game.do_move(&mut input.one, &mut dice);
        if end {
            break;
        };
        end = game.do_move(&mut input.two, &mut dice);
        if end {
            break;
        };
    }
    println!("{:?} {}", input, dice.rolls);
    dice.rolls
        * if input.one.score > input.two.score {
            input.two.score
        } else {
            input.one.score
        }
}

fn part_two() -> u32 {
    let players = get_input();
    let game = Game::gen();

    let (mut w, mut p) = game.do_dirac(players.one);
    let (mut w2, mut p2) = game.do_dirac(players.two);

    loop {
        for pl in p.clone().iter() {
            let (v, b) = game.do_dirac(*pl);
            w += v;
            p = b;
        }
        if p.len() == 0 {
            break;
        }

        for pl in p2.clone().iter() {
            let (v, b) = game.do_dirac(*pl);
            w2 += v;
            p2 = b;
        }
        if p2.len() == 0 {
            break;
        }

        println!("p1: w: {} l: {}", w, p.len());
        println!("p2: w: {} l: {}", w2, p2.len());
        println!("------")
    }

    println!("p1: {}, p2: {}", w, w2);

    0
}

fn main() {
    // println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
