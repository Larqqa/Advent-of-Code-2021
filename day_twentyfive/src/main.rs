#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    map: Vec<char>,
}
const EMPTY: char = '.';
const EAST: char = '>';
const SOUTH: char = 'v';

impl Grid {
    fn move_cucumbers(&self, direction: bool) -> Vec<char> {
        let mut temp: Vec<char> = self.map.clone();
        let matchable = if direction { EAST } else { SOUTH };

        for i in 0..self.map.len() {
            if self.map[i] != matchable {
                continue;
            }

            let x = i % self.width;
            let y = i / self.width;

            match direction {
                true => {
                    let mut xx = x + 1;
                    if xx >= self.width {
                        xx = 0;
                    }
                    let j = self.width * y + xx;
                    if self.map[j] == EMPTY {
                        temp[i] = EMPTY;
                        temp[j] = self.map[i];
                    }
                }
                false => {
                    let mut yy = y + 1;
                    if yy >= self.height {
                        yy = 0;
                    }
                    let j = self.width * yy + x;
                    if self.map[j] == EMPTY {
                        temp[i] = EMPTY;
                        temp[j] = self.map[i];
                    }
                }
            }
        }

        temp
    }

    fn print_map(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.width * y + x;
                print!("{}", self.map[i]);
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn get_input() -> Grid {
    let grid: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    Grid {
        width: grid[0].len(),
        height: grid.len(),
        map: grid.concat(),
    }
}

fn part_one() -> i32 {
    let mut input = get_input();
    // println!("{:?}", input);
    input.print_map();

    let mut loops = 1;
    let mut check: Vec<char>;
    loop {
        check = input.map.clone();
        input.map = input.move_cucumbers(true);
        input.map = input.move_cucumbers(false);

        if check == input.map {
            break;
        } else {
            loops += 1;
        }
    }
    input.print_map();

    loops
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}
