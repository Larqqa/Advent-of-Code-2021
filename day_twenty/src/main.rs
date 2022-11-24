use std::vec;

const DARK: char = '.';
const LIGHT: char = '#';

#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    data: Vec<char>,
}

fn get_input() -> (Vec<char>, Image) {
    let s = include_str!("test.txt");
    let (a, b) = s.split_once("\n\n").unwrap();

    let algo: Vec<char> = a.replace("\n", "").chars().collect();
    let data: Vec<Vec<char>> = b.split("\n").map(|x| x.chars().collect()).collect();

    let image = Image {
        width: data[0].len(),
        height: data.len(),
        data: data.concat(),
    };

    (algo, image)
}

fn run_iter(algo: &Vec<char>, image: &Image) -> Image {
    let mut output = Image {
        width: image.width + 1,
        height: image.height + 1,
        data: vec![],
    };

    for y in 0..output.height + 1 {
        for x in 0..output.width + 1 {
            let first_x = x as i32 - 1;
            let first_y = y as i32 - 1;
            let mut identifier = String::new();

            for y2 in first_y..(y + 2) as i32 {
                let mut line = String::new();
                for x2 in first_x..(x + 2) as i32 {
                    let x3 = x2 - 1;
                    let y3 = y2 - 1;

                    let c = if x3 < 0 || y3 < 0 || x3 as usize >= image.width || y3 as usize >= image.height {
                        DARK
                    } else {
                        let i = image.width * y3 as usize + x3 as usize;
                        if i < image.data.len() {
                            image.data[i]
                        } else {
                            DARK
                        }
                    };
                    line.push(c);
                }

                identifier.push_str(line.as_str());
            }
            println!("{}", identifier);

            output.data.push(DARK);
        }
    }

    println!("{:?}", output);

    output
}

fn part_one() -> i32 {
    let (algo, image) = get_input();
    // println!("{:?}", algo);
    // println!("{:?}", image);

    run_iter(&algo, &image);

    0
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}
