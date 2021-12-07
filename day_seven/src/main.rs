fn main() {
    let input: Vec<i32> = include_str!("input.txt")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!(
        "part one: {}\npart two: {}",
        (0..*input.iter().max().unwrap())
            .map(|n| input.iter().fold(0, |a, b| a + (b - n as i32).abs()))
            .min()
            .unwrap(),
        (0..*input.iter().max().unwrap())
            .map(|n| {
                input.iter().fold(0, |a, b| {
                    let c = (b - n as i32).abs();
                    a + (c * (c + 1) / 2)
                })
            })
            .min()
            .unwrap()
    );
}
