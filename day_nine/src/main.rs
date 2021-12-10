fn get_input() -> (Vec<i32>, i32, i32) {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let width = &input[0].len();
    let height = &input.len();

    (
        input
            .iter()
            .map(|s| s.chars().map(|i| i.to_string().parse().unwrap()))
            .flatten()
            .collect(),
        *width as i32,
        *height as i32,
    )
}

fn find_bottoms(input: &Vec<i32>, width: i32) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();
    for n in 0..input.len() as i32 {
        let top = n - width;
        let bottom = n + width;
        let right = n + 1;
        let left = n - 1;
        let mut adjacent = 0;
        let mut hits = 0;
        if top > 0 {
            if input[top as usize] > input[n as usize] {
                hits += 1;
            }
            adjacent += 1
        }
        if bottom < input.len() as i32 {
            if input[bottom as usize] > input[n as usize] {
                hits += 1;
            }
            adjacent += 1
        }
        if right < input.len() as i32 {
            if input[right as usize] > input[n as usize] {
                hits += 1;
            }
            adjacent += 1
        }
        if left > 0 as i32 {
            if input[left as usize] > input[n as usize] {
                hits += 1;
            }
            adjacent += 1
        }

        if hits == adjacent {
            output.push(n as usize);
        }
    }

    output
}

fn find_lake_size(start: i32, width: i32, input: &Vec<i32>, map: &mut Vec<bool>) -> i32 {
    map[start as usize] = true; // mark visited
    let x = start % width;
    let top = start - width;
    let bottom = start + width;
    let right = start + 1;
    let left = start - 1;
    let mut visited = 1;

    if top >= 0 {
        if input[top as usize] != 9 && map[top as usize] != true {
            visited += find_lake_size(top, width, &input, map);
        }
    }
    if bottom < input.len() as i32 {
        if input[bottom as usize] != 9 && map[bottom as usize] != true {
            visited += find_lake_size(bottom, width, input, map);
        }
    }
    if x < width - 1 && right < input.len() as i32 {
        if input[right as usize] != 9 && map[right as usize] != true {
            visited += find_lake_size(right, width, input, map);
        }
    }
    if x > 0 && left >= 0 {
        if input[left as usize] != 9 && map[left as usize] != true {
            visited += find_lake_size(left, width, input, map);
        }
    }

    visited
}

use image::RgbImage;
fn output_as_image(map: Vec<i32>, width: i32, height: i32) -> Result<(), image::ImageError> {
    let img_buffer: Vec<u8> = map
        .iter()
        .map(|i| match i {
            0 => [0; 3],
            1 => [10; 3],
            2 => [30; 3],
            3 => [50; 3],
            4 => [80; 3],
            5 => [110; 3],
            6 => [140; 3],
            7 => [170; 3],
            8 => [200; 3],
            9 => [255; 3],
            _ => [0; 3],
        })
        .flatten()
        .collect();

    let img = RgbImage::from_raw(width as u32, height as u32, img_buffer)
        .expect("Width and height should match the buffer");
    img.save("out.png")?;
    Ok(())
}

fn main() {
    let (input, width, height) = get_input();
    let mut map: Vec<bool> = vec![false; (width * height) as usize];
    let mut lakes: Vec<i32> = find_bottoms(&input, width)
        .iter()
        .map(|l| find_lake_size(*l as i32, width, &input, &mut map))
        .collect();
    lakes.sort();
    lakes.reverse();

    println!(
        "part one: {}\npart two: {}",
        find_bottoms(&input, width)
            .iter()
            .fold(0, |a, b| a + 1 + input[*b]),
        lakes[0..3].iter().fold(1, |a, b| a * b)
    );

    output_as_image(input, width, height).expect("Problem creating the image");
}
