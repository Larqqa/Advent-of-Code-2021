fn get_input() -> (Vec<i32>, usize, usize) {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let width = input[0].len();
    let height = input.len();
    let grid = input
        .iter()
        .map(|s| s.chars().map(|i| i.to_string().parse().unwrap()))
        .flatten()
        .collect();

    (grid, width, height)
}

use std::collections::HashMap;
fn dijkstra(grid: Vec<i32>, src: usize, v: usize, width: usize) -> (HashMap<usize, i32>, Vec<i32>) {
    let mut spt_set = vec![false; v];
    let mut dist: HashMap<usize, i32> = HashMap::new();
    let mut non_visited: HashMap<usize, i32> = HashMap::new();
    let mut shortest = vec![i32::MAX; v];

    shortest[src] = 0;
    dist.insert(0, 0);
    non_visited.insert(0, 0);
    let len = grid.len();

    for i in 0..v - 1 {
        // if dist.len() % 1000 == 0 {
        //     println!("{}", dist.len());
        // }

        // Get minimum distance
        let key = non_visited
            .iter()
            .min_by_key(|entry| {
                if spt_set[*entry.0] {
                    &i32::MAX
                } else {
                    entry.1
                }
            })
            .unwrap();
        let u = *key.0;

        let top = u as i32 - width as i32;
        let bottom = u + width;
        let left = u as i32 - 1;
        let right = u + 1;
        let mut adjacency: Vec<usize> = Vec::new();

        if top >= 0 {
            adjacency.push(top as usize);
        }
        if bottom < len {
            adjacency.push(bottom);
        }
        if left >= 0 && (left as usize) % width < (width - 1) {
            adjacency.push(left as usize);
        }
        if (right % width) > 0 && right <= len {
            adjacency.push(right);
        }

        spt_set[u] = true;
        non_visited.remove(&u);

        // println!("{}", u);

        for j in adjacency {
            if !spt_set[j] && dist.contains_key(&u) {
                let k = if dist.contains_key(&j) {
                    *dist.get(&j).unwrap()
                } else {
                    i32::MAX
                };
                if dist.get(&u).unwrap() + grid[j] < k {
                    shortest[j] = u as i32;
                    let min = *dist.get(&u).unwrap() + grid[j];
                    non_visited.insert(j, min);
                    dist.insert(j, min);
                }
            }
        }
    }

    (dist, shortest)
}

fn stretch_grid(grid: Vec<i32>, width: usize) -> (Vec<i32>, usize) {
    let w = width * 5;
    let h = w;
    let mut g: Vec<i32> = vec![0; w * h];

    let mut old_y = 0;
    let mut j = 0;
    for i in 0..grid.len() {
        let y = i / width;
        if old_y != y {
            old_y = y;
            j = 0;
        }
        g[j + (w * y)] = grid[i];

        j += 1;
    }

    for k in 1..(h / width) {
        for y in 0..width {
            let offset = y + (k * width);
            for x in 0..width {
                g[x + w * offset] = g[x + w * (offset - width)] + 1;
                if g[x + w * offset] > 9 {
                    g[x + w * offset] = 1
                }
            }
        }
    }

    let mut i = 0;
    let mut i_w = 0;
    for _ in 0..(h / width) {
        for j in 0..(w * width) {
            if g[i + j] != 0 {
                continue;
            }

            g[i + j] = g[i + j - width] + 1;
            if g[i + j] > 9 {
                g[i + j] = 1
            }

            if i_w == w {
                i_w = 0
            } else {
                i_w += 1;
            }
        }
        i += w * width;
    }

    (g, w)
}

fn main() {
    let (grid, width, _) = get_input();
    let len = grid.len();
    let (d, _) = dijkstra(grid.clone(), 0, len, width);
    println!("part one: {:#?}", d.get(&(len - 1)));

    let (g, w) = stretch_grid(grid.clone(), width);
    println!("--dijkstra--");
    let len = g.len();
    let (d, mut s) = dijkstra(g, 0, len, w);
    println!("part two: {:#?}", d.get(&(len - 1)));

    // let mut i = s.len() - 1;
    // while i > 0 {
    //     // println!("{}", i);
    //     let j = s[i] as usize;
    //     s[i] = 0;
    //     i = j;
    // }

    // for i in 0..s.len() {
    //     if s[i] == 0 {
    //         print!("#");
    //     } else {
    //         print!("•");
    //     }

    //     if i != 0 && i % w == (w - 1) {
    //         print!("\n");
    //     }
    // }
}
