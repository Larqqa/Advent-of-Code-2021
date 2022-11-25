use std::vec;

const DARK: char = '.';
const LIGHT: char = '#';

#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    data: Vec<char>,
    void: bool,
}

impl Image {
    #[allow(dead_code)]
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

    fn count_lights(&self) -> usize {
        self.data
            .clone()
            .into_iter()
            .filter(|x| *x == LIGHT)
            .count()
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
        void: false,
    };

    (algo, image)
}

fn get_decimal_from_binary(str: &String) -> isize {
    isize::from_str_radix(&str, 2).unwrap()
}

fn get_binary(str: &String) -> String {
    str.chars()
        .map(|x| if x == LIGHT { '1' } else { '0' })
        .collect()
}

fn run_iter(algo: &Vec<char>, image: &Image) -> Image {
    let mut output = Image {
        width: image.width + 2,
        height: image.height + 2,
        data: vec![],
        void: image.void,
    };

    // Map enhanced image
    for y in 0..output.height {
        for x in 0..output.width {
            let first_x = x as i32 - 1;
            let first_y = y as i32 - 1;
            let mut identifier = String::new();

            // Map 3x3 region around pixel
            for region_y in first_y..(y + 2) as i32 {
                let mut pixel_row = String::new();

                for region_x in first_x..(x + 2) as i32 {
                    let image_x = region_x - 1;
                    let image_y = region_y - 1;

                    // If mapped pixel is outside of original image, add void pixel
                    let pixel = if image_x < 0
                        || image_y < 0
                        || image_x as usize >= image.width
                        || image_y as usize >= image.height
                    {
                        match output.void {
                            true => LIGHT,
                            _ => DARK,
                        }
                    } else {
                        image.data[image.width * image_y as usize + image_x as usize]
                    };
                    pixel_row.push(pixel);
                }
                identifier.push_str(pixel_row.as_str());
            }

            let bin = get_binary(&identifier);
            let num = get_decimal_from_binary(&bin);
            output.data.push(algo[num as usize]);
        }
    }

    // Flicker void if algorithm dictates the flicker
    output.void = if !output.void && algo[0] == LIGHT {
        true
    } else if output.void && algo.last() == Some(&DARK) {
        false
    } else {
        output.void
    };

    output
}

fn part_one() -> usize {
    let (algo, image) = get_input();
    let mut o = image;
    for _ in 0..2 {
        o = run_iter(&algo, &o);
    }
    o.count_lights()
}

fn part_two() -> usize {
    let (algo, image) = get_input();
    let mut o = image;
    for _ in 0..50 {
        o = run_iter(&algo, &o);
    }
    o.count_lights()
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
