#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    point_a: Point,
    point_b: Point,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.point_a.x != self.point_b.x && self.point_a.y != self.point_b.y
    }

    fn map_vh(&self, mut map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        if !self.is_diagonal() {
            let (min_x, max_x) = if self.point_a.x < self.point_b.x {
                (self.point_a.x, self.point_b.x + 1)
            } else {
                (self.point_b.x, self.point_a.x + 1)
            };
            let (min_y, max_y) = if self.point_a.y < self.point_b.y {
                (self.point_a.y, self.point_b.y + 1)
            } else {
                (self.point_b.y, self.point_a.y + 1)
            };

            for x in min_x..max_x {
                for y in min_y..max_y {
                    map[y as usize][x as usize] += 1;
                }
            }
        }
        map
    }

    fn map_dg(&self, mut map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        if self.is_diagonal() {
            let (mut x, mut y, point_b) = if self.point_a.x < self.point_b.x {
                (self.point_a.x, self.point_a.y, &self.point_b)
            } else {
                (self.point_b.x, self.point_b.y, &self.point_a)
            };

            let slope = (point_b.y - y) / (point_b.x - x);

            while x <= point_b.x && (y <= point_b.y || y > point_b.y) {
                map[y as usize][x as usize] += 1;
                x += 1;
                y += slope;
            }
        }
        map
    }
}

fn get_input() -> Vec<Line> {
    return include_str!("input.txt")
        .lines()
        .map(|l| {
            let y: Vec<Vec<i32>> = l
                .split(" -> ")
                .map(|c| c.split(",").map(|n| n.parse().unwrap()).collect())
                .collect();
            Line {
                point_a: Point {
                    x: y[0][0],
                    y: y[0][1],
                },
                point_b: Point {
                    x: y[1][0],
                    y: y[1][1],
                },
            }
        })
        .collect();
}

use image::RgbImage;
fn output_as_image(map: Vec<Vec<u32>>, width: u32, height: u32) -> Result<(), image::ImageError> {
    let img_buffer: Vec<u8> = map
        .iter()
        .flatten()
        .map(|i| if i != &0 { [255, 255, 255] } else { [0, 0, 0] })
        .flatten()
        .collect();

    let img = RgbImage::from_raw(width, height, img_buffer)
        .expect("container should have the right size for the image dimensions");
    img.save("out.png")?;
    Ok(())
}

fn main() {
    let input = get_input();
    let mut map = vec![vec![0; 1000]; 1000];

    for line in &input {
        map = line.map_vh(map);
    }

    println!(
        "part one: {}",
        map.iter().flatten().filter(|b| b >= &&&2).count()
    );

    for line in input {
        map = line.map_dg(map);
    }

    println!(
        "part two: {}",
        map.iter().flatten().filter(|b| b >= &&&2).count()
    );

    output_as_image(map, 1000, 1000).expect("Problem creating the image");
}
