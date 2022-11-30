use std::{
    cmp::{max, min},
    vec,
};

fn get_input() -> Vec<Step> {
    return include_str!("input.txt")
        .lines()
        .map(|s| {
            let sr: Vec<&str> = s.split(" ").map(|x| x).collect();
            let mut coords = sr[1].split(",").map(|x| {
                let c: Vec<&str> = x.split("=").map(|x| x).collect();
                let a: Vec<i128> = c[1].split("..").map(|x| x.parse().unwrap()).collect();
                (c[0], a[0], a[1])
            });

            let x = coords.find(|x| x.0 == "x").unwrap();
            let y = coords.find(|x| x.0 == "y").unwrap();
            let z = coords.find(|x| x.0 == "z").unwrap();

            Step {
                cube: Cuboid::new(
                    min(x.1, x.2),
                    max(x.1, x.2) + 1,
                    min(y.1, y.2),
                    max(y.1, y.2) + 1,
                    min(z.1, z.2),
                    max(z.1, z.2) + 1,
                ),
                switch: sr[0] == "on",
            }
        })
        .collect();
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct Step {
    cube: Cuboid,
    switch: bool,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct Cuboid {
    x_start: i128,
    x_end: i128,
    y_start: i128,
    y_end: i128,
    z_start: i128,
    z_end: i128,
}

impl Cuboid {
    fn new(x1: i128, x2: i128, y1: i128, y2: i128, z1: i128, z2: i128) -> Cuboid {
        let x_start = min(x1, x2);
        let x_end = max(x1, x2);

        let y_start = min(y1, y2);
        let y_end = max(y1, y2);

        let z_start = min(z1, z2);
        let z_end = max(z1, z2);

        Cuboid {
            x_start,
            x_end,
            y_start,
            y_end,
            z_start,
            z_end,
        }
    }

    fn volume(&self) -> i128 {
        let x = self.x_end - self.x_start;
        let y = self.y_end - self.y_start;
        let z = self.z_end - self.z_start;

        x * y * z
    }

    fn intersects(&self, targ: Cuboid) -> bool {
        let x_i = self.x_start <= targ.x_end && self.x_end >= targ.x_start;
        let y_i = self.y_start <= targ.y_end && self.y_end >= targ.y_start;
        let z_i = self.z_start <= targ.z_end && self.z_end >= targ.z_start;

        x_i && y_i && z_i
    }

    fn middlerange(a1: i128, a2: i128, b1: i128, b2: i128) -> Option<(i128, i128)> {
        if a1 <= b1 && a2 >= b2 {
            Some((b1, b2))
        } else if a1 > b1 && a2 >= b2 {
            Some((a1, b2))
        } else if a1 <= b1 && a2 < b2 {
            Some((b1, a2))
        } else if a1 > b1 && a2 < b2 {
            Some((a1, a2))
        } else {
            None
        }
    }

    fn get_offset(a1: i128, b1: i128, a2: i128, b2: i128) -> [bool; 3] {
        [a2 < a1, a2 <= b1 || a1 <= b2, b1 < b2]
    }

    fn get_range(a1: i128, b1: i128, a2: i128, b2: i128) -> [(i128, i128); 3] {
        [
            (a2, a1),
            Cuboid::middlerange(a2, b2, a1, b1).unwrap(),
            (b1, b2),
        ]
    }

    fn split(&self, target: Cuboid) -> Vec<Cuboid> {
        let x_offsets = Cuboid::get_offset(self.x_start, self.x_end, target.x_start, target.x_end);
        let y_offsets = Cuboid::get_offset(self.y_start, self.y_end, target.y_start, target.y_end);
        let z_offsets = Cuboid::get_offset(self.z_start, self.z_end, target.z_start, target.z_end);

        let x_ranges = Cuboid::get_range(self.x_start, self.x_end, target.x_start, target.x_end);
        let y_ranges = Cuboid::get_range(self.y_start, self.y_end, target.y_start, target.y_end);
        let z_ranges = Cuboid::get_range(self.z_start, self.z_end, target.z_start, target.z_end);

        let mut pieces = vec![];
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if x == 1 && y == 1 && z == 1 {
                        continue;
                    }
                    if x_offsets[x] && y_offsets[y] && z_offsets[z] {
                        pieces.push(Cuboid::new(
                            x_ranges[x].0,
                            x_ranges[x].1,
                            y_ranges[y].0,
                            y_ranges[y].1,
                            z_ranges[z].0,
                            z_ranges[z].1,
                        ));
                    }
                }
            }
        }

        pieces
    }
}

fn start_reactor(input: Vec<Step>, bound: Option<i128>) -> Vec<Cuboid> {
    let mut reactor: Vec<Cuboid> = vec![];
    for step in input {
        if bound.is_some() {
            let b = bound.unwrap();
            if (step.cube.x_start < -b || step.cube.x_start > b)
                || (step.cube.y_start < -b || step.cube.y_start > b)
                || (step.cube.z_start < -b || step.cube.z_start > b)
                || (step.cube.x_end < -b || step.cube.x_end > b)
                || (step.cube.y_end < -b || step.cube.y_end > b)
                || (step.cube.z_end < -b || step.cube.z_end > b)
            {
                continue;
            }
        }
        for i in (0..reactor.len()).rev() {
            let cube = reactor[i];
            if step.cube.intersects(cube) {
                let mut c = step.cube.split(cube);
                reactor.remove(i);

                if !c.is_empty() {
                    reactor.append(&mut c);
                }
            }
        }

        if step.switch {
            reactor.push(step.cube);
        }
    }
    reactor
}

fn part_one() -> i128 {
    let input = get_input();
    let reactor = start_reactor(input, Some(50));
    reactor.iter().map(|x| x.volume()).sum()
}

fn part_two() -> i128 {
    let input = get_input();
    let reactor = start_reactor(input, None);
    reactor.iter().map(|x| x.volume()).sum()
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
