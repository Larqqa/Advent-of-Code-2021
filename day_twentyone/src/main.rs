#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct Player {
    id: u32,
    position: u32,
    score: u32,
    wins: u128,
}

#[derive(Debug, Clone, Copy)]
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
    dirac: Vec<(u32, u32)>,
}
#[allow(dead_code)]
impl Game {
    fn gen() -> Game {
        Game {
            board: (1..=10).collect(),
            dirac: vec![
                (3,1),
                (4,3),
                (5,6),
                (6,7),
                (7,6),
                (8,3),
                (9,1),
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

    fn play_dirac(&self, players: Players, turn: bool) -> (u128, u128) {
        if players.one.score >= 21 {
            (1,0)
        } else if players.two.score>= 21 {
            (0,1)
        } else {
            let mut p: Player;
            let mut ps: Players;
            let (mut x, mut y): (u128, u128);
            let mut a = 0;
            let mut b = 0;
            for roll in self.dirac.iter() {
                p = if !turn {players.one.clone()} else {players.two.clone()};
                p.position = (p.position + roll.0) % self.board.len() as u32;
                p.score += self.board[p.position as usize];

                ps = players.clone();
                if !turn {ps.one = p} else {ps.two = p}

                (x,y) = self.play_dirac(ps, !turn);
                a += x * roll.1 as u128;
                b += y * roll.1 as u128;
            }
            (a,b)
        }
    }
}

fn get_input() -> Players {
    let s: Vec<Player> = include_str!("input.txt")
        .lines()
        .map(|x| {
            let a: Vec<&str> = x.split("starting position").collect();
            Player {
                id: a[0].replace("Player", "").trim().parse().unwrap(),
                position: a[1].replace(":", "").trim().parse::<u32>().unwrap() - 1,
                score: 0,
                wins: 0
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
    dice.rolls
        * if input.one.score > input.two.score {
            input.two.score
        } else {
            input.one.score
        }
}

fn part_two() -> u128 {
    let players = get_input();
    let game = Game::gen();
    let out = game.play_dirac(players,false);

    out.0.max(out.1)
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
