use std::vec;

const DARK: char = '.';
const LIGHT: char = '#';

#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Image {
    fn print_image(&self) {
        print!("\n");
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[self.width * y + x]);
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn get_input() -> (Vec<char>, Image) {
    let s = include_str!("input.txt");
    // let s = include_str!("test.txt");

    let (a, b) = s.split_once("\r\n\r\n").unwrap();

    let algo: Vec<char> = a.replace("\r\n", "").chars().collect();
    let data: Vec<Vec<char>> = b.split("\r\n").map(|x| x.chars().collect()).collect();

    let image = Image {
        width: data[0].len(),
        height: data.len(),
        data: data.concat(),
    };

    (algo, image)
}

fn get_decimal_from_binary(str: &String) -> isize {
    isize::from_str_radix(&str, 2).unwrap()

}

fn get_binary(str: &String) -> String {
    str.chars().map(|x| if x == LIGHT {'1'} else {'0'}).collect()
}

fn run_iter(algo: &Vec<char>, image: &Image) -> Image {
    let mut output = Image {
        width: image.width + 2,
        height: image.height + 2,
        data: vec![],
    };

    for y in 0..output.height {
        for x in 0..output.width {
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
            if identifier.contains(LIGHT)  {
                let bin = get_binary(&identifier);
                let num = get_decimal_from_binary(&bin);
                // println!("{} {}", bin, num);
                output.data.push(algo[num as usize]);
            } else {
                output.data.push(DARK);
            }

        }
    }

    // output.print_image();

    output
}

fn part_one() -> usize {
    let (algo, image) = get_input();
    // println!("{:?}", algo);
    // println!("{:?}", image);

    image.print_image();
    let mut o = image;
    // for _ in 0..2 {   
    //     o = run_iter(&algo, &o);
    // }
    o = run_iter(&algo, &o);
    o.print_image();

    o.data.into_iter().filter(|x| *x == LIGHT).count()
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}
