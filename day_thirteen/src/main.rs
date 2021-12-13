#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Fold {
    direction: String,
    position: usize,
}

fn get_input() -> (Vec<Coord>, Vec<Fold>) {
    let input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let coords = input[0]
        .lines()
        .map(|s| {
            let n: Vec<&str> = s.split(",").collect();
            Coord {
                x: n[0].parse().unwrap(),
                y: n[1].parse().unwrap(),
            }
        })
        .collect();

    let folds = input[1]
        .lines()
        .map(|s| {
            let n: Vec<&str> = s.split("g ").collect::<Vec<&str>>()[1].split("=").collect();
            Fold {
                direction: String::from(n[0]),
                position: n[1].parse().unwrap(),
            }
        })
        .collect();

    (coords, folds)
}

fn print_grid(paper: &Vec<char>, width: usize, print_readable: bool) {
    for i in 0..paper.len() {
        if print_readable {
            if paper[i] == '·' {
                print!(" ");
            } else {
                print!("█");
            }
        } else {
            print!("{}", paper[i]);
        }

        if i != 0 && i % (width) as usize == (width - 1) as usize {
            println!("");
        }
    }
    print!("\n");
}

fn fold_y(paper: &Vec<char>, width: usize, height: usize, position: usize) -> Vec<char> {
    let top_len = width * position;
    let bottom_len = width * (height - position);

    // Split grid
    let mut top = paper[..top_len].to_owned();
    let bottom = paper[bottom_len..].to_owned();

    // Fold bottom to top
    for i in 0..bottom.len() {
        let x = i % width;
        let y = (position - (i / width)) - 1;

        if bottom[i] == '•' {
            top[x + width * y] = bottom[i];
        }
    }

    top
}

fn fold_x(paper: &Vec<char>, width: usize, position: usize) -> Vec<char> {
    let mut left: Vec<char> = Vec::new();
    let mut right: Vec<char> = Vec::new();

    // Split grid
    for i in 0..paper.len() {
        let x = i % width;

        if x < position {
            left.push(paper[i]);
        } else if x > position {
            right.push(paper[i]);
        }
    }

    // Fold right to left
    for i in 0..right.len() {
        let x = (position - (i % position)) - 1;
        let y = i / position;

        if right[i] == '•' {
            left[x + position * y] = right[i];
        }
    }

    left
}

fn get_size(input: &Vec<Coord>) -> (usize, usize) {
    let width = input
        .iter()
        .reduce(|a, b| if a.x > b.x { a } else { b })
        .unwrap()
        .x
        + 1;

    let height = input
        .iter()
        .reduce(|a, b| if a.y > b.y { a } else { b })
        .unwrap()
        .y
        + 1;

    (width, height)
}

fn part_one(mut paper_copy: Vec<char>, first_fold: &Fold, width: usize) -> u32 {
    match first_fold.direction.as_str() {
        "x" => paper_copy = fold_x(&paper_copy, width, first_fold.position),
        "y" => paper_copy = fold_x(&paper_copy, width, first_fold.position),
        _ => (),
    }

    paper_copy
        .iter()
        .fold(0, |a, b| if *b == '•' { a + 1 } else { a })
}

fn main() {
    let (coords, folds) = get_input();
    let (mut width, mut height) = get_size(&coords);

    // Generate paper
    let mut paper = vec!['·'; width * height];
    for coord in coords {
        paper[coord.x + width * coord.y] = '•';
    }

    println!("part one: {}", part_one(paper.clone(), &folds[0], width));

    for fold in folds {
        match fold.direction.as_str() {
            "x" => {
                paper = fold_x(&paper, width, fold.position);
                width = fold.position;
            }
            "y" => {
                paper = fold_y(&paper, width, height, fold.position);
                height = fold.position;
            }
            _ => (),
        }
    }

    println!("part two:");
    print_grid(&paper, width, false);
    print_grid(&paper, width, true);
}
