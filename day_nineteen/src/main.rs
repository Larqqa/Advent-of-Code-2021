#[derive(Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn to_abs(&self) -> Coordinate {
        Coordinate {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

#[derive(Debug)]
struct Scanner {
    id: i32,
    position: Coordinate,
    beacons: Vec<Coordinate>,
}

use std::collections::HashMap;
fn get_input() -> HashMap<i32, Scanner> {
    let input: Vec<&str> = include_str!("test.txt").split("\n\n").collect();
    let mut scanners: HashMap<i32, Scanner> = HashMap::new();
    for s in input {
        let mut i: Vec<&str> = s.lines().collect();

        let id: i32 = i[0]
            .replace("---", "")
            .replace("scanner", "")
            .trim()
            .parse()
            .unwrap();
        i.remove(0);

        let beacons: Vec<Coordinate> = i
            .iter()
            .map(|c| {
                let coords: Vec<&str> = c.split(",").collect();
                Coordinate {
                    x: coords[0].parse().unwrap(),
                    y: coords[1].parse().unwrap(),
                    z: coords[2].parse().unwrap(),
                }
            })
            .collect();
        scanners.insert(
            id,
            Scanner {
                id,
                position: Coordinate {
                    x: i32::MAX,
                    y: i32::MAX,
                    z: i32::MAX,
                },
                beacons,
            },
        );
    }

    scanners.get_mut(&0).unwrap().position = Coordinate { x: 0, y: 0, z: 0 };
    scanners.get_mut(&1).unwrap().position = Coordinate {
        x: 68,
        y: -1246,
        z: -43,
    };

    scanners
}

fn find_matching_beacons(s0: &Scanner, s1: &Scanner) {
    let mut x_m: HashMap<i32, i32> = HashMap::new();
    let mut y_m: HashMap<i32, i32> = HashMap::new();
    let mut z_m: HashMap<i32, i32> = HashMap::new();

    for b in &s0.beacons {
        let beacon = Coordinate {
            x: b.x - s0.position.x,
            y: b.y - s0.position.y,
            z: b.z - s0.position.z,
        };
        let beacon_abs = beacon.to_abs();
        for target in &s1.beacons {
            let target_abs = target.to_abs();
            let b_norm = Coordinate {
                x: beacon.x - target.x,
                y: beacon.y - target.y,
                z: beacon.z - target.z,
            };
            let b_abs = Coordinate {
                x: beacon_abs.x - target.x,
                y: beacon_abs.y - target.y,
                z: beacon_abs.z - target.z,
            };
            let t_norm = Coordinate {
                x: beacon.x - target_abs.x,
                y: beacon.y - target_abs.y,
                z: beacon.z - target_abs.z,
            };
            let t_abs = Coordinate {
                x: beacon_abs.x - target_abs.x,
                y: beacon_abs.y - target_abs.y,
                z: beacon_abs.z - target_abs.z,
            };

            // println!("--calc--");
            // println!("b: {:?},\tt: {:?}", beacon, target);
            // // println!("b: {:?},\tt: {:?}", beacon_abs, target_abs);
            // println!(
            //     "n: {:?},\tabs: {:?}\tn: {:?},\tabs: {:?}",
            //     b_norm, b_abs, t_norm, t_abs
            // );

            let x_s = vec![t_norm.x, t_abs.x];
            let y_s = vec![t_norm.y, t_abs.y];
            let z_s = vec![t_norm.z, t_abs.z];

            for x in x_s {
                let y = target.x.abs() - x;
                let y2 = target.x.abs() + x;
                if beacon.x == y || beacon.x == y2 {
                    *x_m.entry(x).or_insert(0) += 1;
                }
            }

            for y in y_s {
                let x = target.y.abs() + y;
                let x2 = target.y.abs() - y;
                if beacon.y == x || beacon.y == x2 {
                    *y_m.entry(y).or_insert(0) += 1;
                }
            }

            for z in z_s {
                let y = target.z.abs() - z;
                let y2 = target.z.abs() + z;
                if beacon.z == y || beacon.z == y2 {
                    *z_m.entry(z).or_insert(0) += 1;
                }
            }
        }
    }
    println!("xs {:?}", x_m.get(&68));
    println!("ys {:?}", y_m.get(&-1246));
    println!("zs {:?}", z_m.get(&-43));

    println!("x {:?}", x_m.iter().max_by(|a, b| a.1.cmp(&b.1)));
    println!("y {:?}", y_m.iter().max_by(|a, b| a.1.cmp(&b.1)));
    println!("z {:?}", z_m.iter().max_by(|a, b| a.1.cmp(&b.1)));
}

fn part_one() -> i32 {
    let input = get_input();
    // println!("{:?}", input);

    let s0 = input.get(&0).unwrap();
    let s1 = input.get(&1).unwrap();
    // println!("{:?}", s0);
    // println!("{:?}", s1);

    find_matching_beacons(s0, s1);

    0
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}
