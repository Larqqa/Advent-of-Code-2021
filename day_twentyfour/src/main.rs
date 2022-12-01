#[derive(Copy, Clone, Debug)]
struct ALU {
    w: i128,
    x: i128,
    y: i128,
    z: i128,
}

impl ALU {
    fn new() -> ALU {
        ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn get(&self, n: &str) -> i128 {
        match n {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            _ => n.parse().unwrap(),
        }
    }

    fn inp(&mut self, a: i128, n: &str) {
        match n {
            "w" => self.w = a,
            "x" => self.x = a,
            "y" => self.y = a,
            "z" => self.z = a,
            _ => (),
        }
    }

    fn add(&mut self, a: &str, b: &str) {
        self.inp(&self.get(a) + &self.get(b), a);
    }

    fn mul(&mut self, a: &str, b: &str) {
        self.inp(&self.get(a) * &self.get(b), a);
    }

    fn div(&mut self, a: &str, b: &str) {
        self.inp(&self.get(a) / &self.get(b), a);
    }

    fn modulo(&mut self, a: &str, b: &str) {
        self.inp(&self.get(a) % &self.get(b), a);
    }

    fn eql(&mut self, a: &str, b: &str) {
        self.inp(
            match self.get(a) == self.get(b) {
                true => 1,
                _ => 0,
            },
            a,
        );
    }

    fn operation(mut self, op: &str, input: &mut Vec<i128>) -> Self {
        let os: Vec<&str> = op.split(" ").collect();
        match os[0] {
            "inp" => {
                if input.len() > 0 {
                    self.inp(input.pop().unwrap(), os[1])
                }
            }
            "add" => self.add(os[1], os[2]),
            "mul" => self.mul(os[1], os[2]),
            "div" => self.div(os[1], os[2]),
            "mod" => self.modulo(os[1], os[2]),
            "eql" => self.eql(os[1], os[2]),
            _ => (),
        }
        self
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    target: usize,
    current: usize,
    offset: i32,
}
fn main() {
    let mut input: Vec<&str> = include_str!("input.txt").lines().map(|x| x).collect();
    let mut instructions = vec![];
    while input.len() > 0 {
        let (ins, rest) = input.split_at(18);
        instructions.push(ins.to_vec());
        input = rest.to_vec();
    }

    let mut i = 0;
    let mut pushes: Vec<Instruction> = vec![];
    let mut parsed_instructions: Vec<Instruction> = vec![];
    for ins in &instructions {
        let check = ins[5].split(" ").collect::<Vec<&str>>()[2]
            .parse::<i32>()
            .unwrap();
        let offset = ins[15].split(" ").collect::<Vec<&str>>()[2]
            .parse::<i32>()
            .unwrap();

        if check <= 0 {
            let p = pushes.pop().unwrap();
            parsed_instructions.push(Instruction {
                target: i,
                current: p.current,
                offset: check + p.offset,
            });
        } else {
            pushes.push(Instruction {
                target: 0,
                current: i,
                offset: offset,
            });
        }
        i += 1;
    }

    let mut ans = vec![0; 14];
    for i in &parsed_instructions {
        ans[i.current] = if i.offset < 0 { 9 } else { 9 - i.offset };
        ans[i.target] = ans[i.current] + i.offset;
    }
    let p1 = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .concat();

    let mut ans = vec![0; 14];
    for i in &parsed_instructions {
        ans[i.current] = if i.offset < 0 {
            i32::abs(i.offset) + 1
        } else {
            1
        };
        ans[i.target] = ans[i.current] + i.offset;
    }
    let p2 = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .concat();

    let mut alu = ALU::new();
    for i in &input {
        alu = alu.operation(
            i,
            &mut p1
                .clone()
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect(),
        );
    }
    println!("part one: {}, valid: {}", p1, alu.z == 0);

    let mut alu = ALU::new();
    for i in &input {
        alu = alu.operation(
            i,
            &mut p2
                .clone()
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect(),
        );
    }
    println!("part two: {}, valid: {}", p2, alu.z == 0);
}
