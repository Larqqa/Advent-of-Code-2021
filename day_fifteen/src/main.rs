fn get_input() -> (Vec<i32>, usize, usize) {
    let input: Vec<&str> = include_str!("test.txt").lines().collect();

    let width = input[0].len();
    let height = input.len();
    let grid = input
        .iter()
        .map(|s| s.chars().map(|i| i.to_string().parse().unwrap()))
        .flatten()
        .collect();

    (grid, width, height)
}

// fn print_grid(grid: &Vec<i32>, width: &usize) {
//     print!("\n");
//     for i in 0..grid.len() {
//         print!("{}", grid[i]);

//         if i != 0 && i % width == (width - 1) {
//             print!("\n");
//         }
//     }
//     print!("\n");
// }

fn min_distance(dist: Vec<i32>, spt_set: Vec<bool>, v: usize) -> usize {
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

fn dijkstra(matrix: Vec<Vec<i32>>, src: usize, v: usize) -> Vec<i32> {
    let mut spt_set = vec![false; v];
    let mut dist = vec![i32::MAX; v];

    dist[src] = 0;

    for _ in 0..v - 1 {
        let u = min_distance(dist.clone(), spt_set.clone(), v);
        spt_set[u] = true;
        
        for j in 0..v {
            if !spt_set[j]
                && matrix[u][j] != 0
                && dist[u] != i32::MAX
                && dist[u] + matrix[u][j] < dist[j]
            {
                dist[j] = dist[u] + matrix[u][j];
            }
        }
    }

    dist
}

fn gen_matrix(grid: Vec<i32>, width: usize) -> Vec<Vec<i32>> {
    let mut m: Vec<Vec<i32>> = Vec::new();
    let len = grid.len();

    for i in 0..grid.len() {
        let top = i as i32 - width as i32;
        let bottom = i + width;
        let left = i as i32 - 1;
        let right = i + 1;

        let mut adjacency: Vec<i32> = vec![0; grid.len()];

        if top >= 0 {
            adjacency[top as usize] = grid[top as usize];
        }
        if bottom < len {
            adjacency[bottom] = grid[bottom];
        }
        if left >= 0 {
            adjacency[left as usize] = grid[left as usize];
        }
        if (right % width) < width && right < len {
            adjacency[right] = grid[right];
        }

        m.push(adjacency);
    }

    m
}

fn main() {
    let (grid, width, _) = get_input();
    // println!("w {}, h {}", width, height);
    // print_grid(&grid, &width);

    // println!("--grid--");
    // println!("{:?}", grid);
    // println!("--matrix--");
    let m = gen_matrix(grid, width);
    // for i in &m {
    //     println!("{:?}", i);
    // }

    println!("--dijkstra--");
    let len = m.len();
    let d = dijkstra(m, 0, len);
    println!("sum: {}", d[len - 1]);
    // println!("{:?}", d);

    // let matrix = vec![
    //     vec![0, 4, 0, 0, 0, 0, 0, 8, 0],
    //     vec![4, 0, 8, 0, 0, 0, 0, 11, 0],
    //     vec![0, 8, 0, 7, 0, 4, 0, 0, 2],
    //     vec![0, 0, 7, 0, 9, 14, 0, 0, 0],
    //     vec![0, 0, 0, 9, 0, 10, 0, 0, 0],
    //     vec![0, 0, 4, 14, 10, 0, 2, 0, 0],
    //     vec![0, 0, 0, 0, 0, 2, 0, 1, 6],
    //     vec![8, 11, 0, 0, 0, 0, 1, 0, 7],
    //     vec![0, 0, 2, 0, 0, 0, 6, 7, 0],
    // ];

    // dijkstra(matrix, 0, 9);
}
