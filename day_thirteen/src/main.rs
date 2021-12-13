#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Fold {
    direction: String,
    position: u32,
}

fn get_input() -> (Vec<Coord>, Vec<Fold>) {
    let input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    (
        input[0]
            .lines()
            .map(|s| {
                let n: Vec<&str> = s.split(",").collect();
                Coord {
                    x: n[0].parse().unwrap(),
                    y: n[1].parse().unwrap(),
                }
            })
            .collect(),
        input[1]
            .lines()
            .map(|s| {
                let n: Vec<&str> = s.split("g ").collect::<Vec<&str>>()[1].split("=").collect();
                Fold {
                    direction: String::from(n[0]),
                    position: n[1].parse().unwrap(),
                }
            })
            .collect(),
    )
}

fn print_grid(paper: &Vec<char>, width: u32) {
    for i in 0..paper.len() {
        print!("{}", paper[i]);
        if i != 0 && i % (width) as usize == (width - 1) as usize {
            println!("");
        }
    }
    print!("\n");
}

fn fold_y(paper: &Vec<char>, width: u32, height: u32, position: u32) -> Vec<char> {
    let top_len = width * position;
    let bottom_len = width * (height - position);
    println!("t {:?}, b {:?}", top_len, bottom_len);

    let mut top = paper[..top_len as usize].to_owned();

    let temp = paper[bottom_len as usize..].to_owned();
    let mut bottom = temp.clone();
    for i in 0..temp.len() {
        let w = width as usize;
        let h = position as usize;
        let x = i % w;
        let y = (h - (i / w)) - 1;
        bottom[x + w * y] = temp[i];
    }

    for i in 0..top.len() {
        if bottom[i] == '•' {
            top[i] = bottom[i];
        }
    }

    top
}

fn fold_x(paper: &Vec<char>, width: u32, position: u32) -> Vec<char> {
    let mut left: Vec<char> = Vec::new();
    let mut right: Vec<char> = Vec::new();

    for i in 0..paper.len() {
        let x = i % width as usize;

        if x < position as usize {
            left.push(paper[i]);
        } else if x > position as usize {
            right.push(paper[i]);
        }
    }

    let mut temp = right.clone();
    for i in 0..right.len() {
        let w = position as usize;
        let x = (w - (i % w)) - 1;
        let y = i / w;
        temp[x + w * y] = right[i];
    }

    for i in 0..left.len() {
        if temp[i] == '•' {
            left[i] = temp[i];
        }
    }

    left
}

fn count_dots(paper: &Vec<char>) -> u32 {
    paper
        .iter()
        .fold(0, |a, b| if *b == '•' { a + 1 } else { a })
}

fn part_one() -> u32 {
    let (coords, folds) = get_input();

    let width = coords
        .iter()
        .reduce(|a, b| if a.x > b.x { a } else { b })
        .unwrap()
        .x
        + 1;

    let height = coords
        .iter()
        .reduce(|a, b| if a.y > b.y { a } else { b })
        .unwrap()
        .y
        + 1;

    println!("width {:?}, height {:?}", width, height);

    let mut paper = vec!['·'; (width * height) as usize];

    for c in coords {
        let i = c.x + width * c.y;
        paper[i as usize] = '•';
    }

    let mut temp = paper.clone();
    let mut w = width;
    let mut h = height;
    for i in 0..folds.len() {
        let fold = folds[i].clone();
        if fold.direction == "x" {
            temp = fold_x(&temp, w, fold.position);
            w = fold.position;
        } else {
            temp = fold_y(&temp, w, h, fold.position);
            h = fold.position;
        }
    }

    print_grid(&temp, w);
    count_dots(&temp)
}

fn main() {
    println!("part one: {}", part_one());
}
