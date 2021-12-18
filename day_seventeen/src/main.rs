#[derive(Debug, Clone)]
struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn make_range(range: &str) -> Range {
        let c: Vec<&str> = range.split("=").collect();
        let d: Vec<i32> = c[1].split("..").map(|k| k.parse().unwrap()).collect();
        Range {
            from: d[0],
            to: d[1],
        }
    }
}

#[derive(Debug, Clone)]
struct Target {
    x: Range,
    y: Range,
}

fn part_one(target: Target) -> i32 {
    println!("{:?}", target);
    let WIDTH = 100;
    let HEIGHT = 50;

    let mut map = vec!['.'; WIDTH * HEIGHT];
    let mut offset = 30;
    let mut speed: (i32, i32) = (7, 2);
    let mut position: (i32, i32) = (0, (HEIGHT - offset) as i32);
    map[(position.0 + WIDTH as i32 * position.1) as usize] = '#';
    let mut map_pos = position.0 + WIDTH as i32 * position.1;

    // Map target
    for y in target.y.from..target.y.to {
        for x in target.x.from..target.x.to {
            println!("{}, {}", x, HEIGHT - (offset as i32 + y) as usize);
            map[(x + WIDTH as i32 * (HEIGHT as i32 - (offset as i32 + y))) as usize] = 'T';
        }
    }

    // Map trajectory
    while map_pos < map.len() as i32 {
        position.0 += speed.0;
        position.1 -= speed.1;

        println!("speed: {:?}", speed);
        println!(
            "position: {:?}  {:?}",
            position.0,
            HEIGHT as i32 - (offset as i32 + position.1)
        );

        if map_pos > 0 && map_pos < map.len() as i32 {
            map[map_pos as usize] = '#';
        }

        if speed.0 != 0 {
            if speed.0 < 0 {
                speed.0 += 1;
            } else {
                speed.0 -= 1;
            }
        }
        speed.1 -= 1;

        map_pos = position.0 + WIDTH as i32 * position.1;
    }

    // Print map
    for i in 0..map.len() {
        print!("{}", map[i]);

        if i != 0 && i % WIDTH == WIDTH - 1 {
            print!("\n");
        }
    }
    print!("\n");

    0
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    let input = include_str!("test.txt").replace("target area: ", "");
    let targets: Vec<&str> = input.split(", ").collect();
    let target = Target {
        x: Range::make_range(targets[0]),
        y: Range::make_range(targets[1]),
    };

    println!("part one: {}", part_one(target));
    // println!("part two: {}", part_two());
}
