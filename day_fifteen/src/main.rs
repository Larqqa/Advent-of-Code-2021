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

fn min_distance(dist: &Vec<i32>, spt_set: &Vec<bool>, v: usize) -> usize {
    let mut min = i32::MAX;
    let mut min_index = 0;

    for j in 0..v {
        if spt_set[j] == false && dist[j] <= min {
            min = dist[j];
            min_index = j;
        }
    }

    min_index
}

fn dijkstra(
    matrix: Vec<HashMap<usize, i32>>,
    src: usize,
    v: usize,
    width: usize,
) -> (Vec<i32>, Vec<i32>) {
    let mut spt_set = vec![false; v];
    let mut dist = vec![i32::MAX; v];
    let mut shortest = vec![i32::MAX; v];

    shortest[src] = 0;
    dist[src] = 0;
    let len = dist.len();

    for i in 0..v - 1 {
        // let u = min_distance(&dist, &spt_set, v);

        let top = i as i32 - width as i32;
        let bottom = i + width;
        let left = i as i32 - 1;
        let right = i + 1;
        let mut min = i32::MAX;
        let mut min_index = 0;

        if top >= 0 && spt_set[top as usize] == false && min > dist[top as usize] {
            min_index = top;
        }
        if bottom < len && spt_set[bottom as usize] == false && min > dist[bottom as usize] {
            min_index = bottom as i32;
        }
        if left >= 0
            && (left as usize) % width < (width - 1)
            && spt_set[left as usize] == false
            && min > dist[left as usize]
        {
            min_index = left;
        }
        if (right % width) > 0
            && right <= len
            && spt_set[right as usize] == false
            && min > dist[right as usize]
        {
            min_index = right as i32;
        }

        // println!("{}", i);

        let u = min_index as usize;
        spt_set[u] = true;

        // Eliminate this loop!!!
        for j in 0..v {
            if !spt_set[j]
                && matrix[u].contains_key(&j)
                && dist[u] != i32::MAX
                && dist[u] + matrix[u].get(&j).unwrap() < dist[j]
            {
                shortest[j] = u as i32;
                dist[j] = dist[u] + matrix[u].get(&j).unwrap();
            }
        }
    }

    (dist, shortest)
}

use std::collections::HashMap;
fn gen_matrix(grid: Vec<i32>, width: usize) -> Vec<HashMap<usize, i32>> {
    let mut m: Vec<HashMap<usize, i32>> = Vec::new();
    let len = grid.len();

    for i in 0..grid.len() {
        let top = i as i32 - width as i32;
        let bottom = i + width;
        let left = i as i32 - 1;
        let right = i + 1;

        // let mut adjacency: Vec<i32> = vec![0; grid.len()];
        let mut adjacency: HashMap<usize, i32> = HashMap::new();

        if top >= 0 {
            adjacency.insert(top as usize, grid[top as usize]);
        }
        if bottom < len {
            adjacency.insert(bottom, grid[bottom]);
        }
        if left >= 0 && (left as usize) % width < (width - 1) {
            adjacency.insert(left as usize, grid[left as usize]);
        }
        if (right % width) > 0 && right <= len {
            adjacency.insert(right, grid[right]);
        }

        m.push(adjacency);
    }

    m
}

fn print_grid(grid: &Vec<i32>, width: &usize) {
    print!("\n");
    for i in 0..grid.len() {
        print!("{}", grid[i]);

        if i != 0 && i % width == (width - 1) {
            print!("\n");
        }
    }
    print!("\n");
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

    // print_grid(&g, &w);
    (g, w)
}

fn main() {
    let (grid, width, _) = get_input();
    // println!("w {}, h {}", width, height);
    // print_grid(&grid, &width);
    let (g, w) = stretch_grid(grid, width);
    let m = gen_matrix(g, w);

    println!("--dijkstra--");
    let len = m.len();
    let (d, _) = dijkstra(m, 0, len, w);
    println!("part two: {}", d[len - 1]);

    // println!("--grid--");
    // println!("{:?}", grid);

    // println!("--matrix--");
    // let m = gen_matrix(grid, width);
    // // for i in &m {
    // //     println!("{:?}", i);
    // // }

    // println!("--dijkstra--");
    // let len = m.len();
    // let (d, mut s) = dijkstra(m, 0, len);
    // println!("part one: {}", d[len - 1]);

    // for i in 0..d.len() {
    //     if d[i] < 10 {
    //         print!(" {} ", d[i]);
    //     } else {
    //         print!("{} ", d[i]);
    //     }

    //     if i != 0 && i % width == (width - 1) {
    //         print!("\n");
    //     }
    // }
    // print!("\n");

    // for i in 0..s.len() {
    //     if s[i] < 10 {
    //         print!(" {} ", s[i]);
    //     } else {
    //         print!("{} ", s[i]);
    //     }

    //     if i != 0 && i % width == (width - 1) {
    //         print!("\n");
    //     }
    // }
    // print!("\n");

    // let mut i = s.len() - 1;
    // while i > 0 {
    //     // println!("{}", i);
    //     let j = s[i] as usize;
    //     s[i] = 0;
    //     i = j;
    // }

    // for i in 0..s.len() {
    //     if s[i] < 10 {
    //         print!(" {} ", s[i]);
    //     } else {
    //         print!("{} ", s[i]);
    //     }

    //     if i != 0 && i % width == (width - 1) {
    //         print!("\n");
    //     }
    // }
}
