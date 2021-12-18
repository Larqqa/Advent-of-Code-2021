fn decode_to_binary(hex: String) -> String {
    let mut binary = String::new();

    for c in hex.chars() {
        binary += match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '4' => "0100",
            '3' => "0011",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        };
    }

    binary
}

fn convert_binary_to_dec(bin: &str) -> i128 {
    isize::from_str_radix(bin, 2).unwrap() as i128
}

#[derive(Debug)]
struct Header {
    version: i128,
    type_id: i128,
}

#[derive(Debug)]
struct Packet {
    full: String,
    header: Header,
    body: String,
}

fn make_packet(bin: &String) -> Packet {
    Packet {
        full: bin.to_string(),
        header: Header {
            version: convert_binary_to_dec(&bin[..3]),
            type_id: convert_binary_to_dec(&bin[3..6]),
        },
        body: bin[6..].to_string(),
    }
}

fn parse_packet(packet: Packet, tree: &mut Vec<String>) -> (Vec<i128>, &mut Vec<String>) {
    let mut stack = match packet.header.type_id {
        4 => parse_literal(packet.body, tree),
        _ => parse_operator(packet.body, tree),
    };

    let operator = match packet.header.type_id {
        0 => "+",
        1 => "*",
        2 => "min",
        3 => "max",
        5 => ">",
        6 => "<",
        7 => "=",
        _ => "",
    };

    println!("{:?}", operator);
    println!("{:?}", stack);
    println!("{:?}", tree);

    if operator != "" {
        let mut stack_string = String::new();
        stack_string.push_str("(");

        if stack.len() > 0 {
            stack_string += &stack
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(operator)
        } else {
            stack_string += &tree
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(operator);
            tree.clear();
        };
        stack_string.push_str(")");

        tree.push(stack_string);
        stack = Vec::new();
    }

    (stack, tree)
}

fn parse_literal(body: String, tree: &mut Vec<String>) -> Vec<i128> {
    // println!("literal: {:#?}", body);

    let bits = body.chars();
    let mut counter = 0;
    let mut prefix = '\0';
    let mut binary = String::new();
    let mut output = String::new();

    for bit in bits {
        if counter == 0 {
            prefix = bit;
        } else {
            binary += &bit.to_string();
        }

        if counter == 4 {
            counter = 0;
            output += &binary;

            if prefix == '0' {
                break;
            }

            binary = String::new();
        } else {
            counter += 1;
        }
    }

    let mut literals: Vec<i128> = vec![convert_binary_to_dec(&output)];

    let output_len = output.len() + (output.len() / 4);
    // println!("output length: {}", output_len);
    if output_len != body.len() {
        let next = &body[output_len..].to_string();
        // println!("--next--");
        if next.chars().any(|a| a == '1') {
            // println!("{}", next);
            let (mut packets, _) = parse_packet(make_packet(&next), tree);
            literals.append(&mut packets);
        } else {
            // println!("empty digits: {}", next);
        }
    }

    literals
}

fn parse_operator(body: String, tree: &mut Vec<String>) -> Vec<i128> {
    // println!("operator: {:#?}", body);

    let (length_type, length_bin, length) = match &body[..1] {
        "0" => {
            let length_type = 16;
            let length_bin = &body[1..16];
            let length = convert_binary_to_dec(length_bin);
            (length_type as usize, length_bin, length as usize)
        }
        "1" => {
            let length_type = 12;
            let length_bin = &body[1..12];
            let length = convert_binary_to_dec(length_bin) * 11;
            (length_type as usize, length_bin, length as usize)
        }
        _ => (0, "", 0),
    };
    // println!("length type: {}", length_type);
    // println!("length bin: {}", length_bin);
    // println!("packet length: {}", length);

    let sub_packet_end = length_type + length;
    // println!("sub packet end: {}", sub_packet_end);

    let sub_packets = &body[length_type..].to_string();
    // println!("sub packets: {}", sub_packets);

    let end_string = &body[sub_packet_end..].to_string();
    // println!("end string: {}", end_string);

    // println!("--sub packet--");
    let (packets, _) = parse_packet(make_packet(&sub_packets), tree);
    packets
}

fn run_operations(mut operations: Vec<i128>) -> Vec<i128> {
    println!("{:?}", operations);

    let mut curr = operations.pop();
    if curr != None && curr < Some(0) {
        return run_operations(operations);
    }

    let mut numbers: Vec<i128> = Vec::new();
    while curr != None && curr > Some(0) {
        numbers.push(curr.unwrap());
        curr = operations.pop();
    }

    // println!("{:?}", numbers);

    let result = match curr {
        Some(-1) => numbers.iter().fold(0, |a, b| a + b),
        Some(-2) => numbers.iter().fold(1, |a, b| a * b),
        Some(-3) => *numbers.iter().min().unwrap(),
        Some(-4) => *numbers.iter().max().unwrap(),
        Some(-6) => {
            if numbers[0] < numbers[1] {
                1
            } else {
                0
            }
        }
        Some(-7) => {
            if numbers[0] > numbers[1] {
                1
            } else {
                0
            }
        }
        Some(-8) => {
            if numbers.windows(2).all(|w| w[0] == w[1]) {
                1
            } else {
                0
            }
        }
        Some(_) => 0,
        None => 0,
    };

    if operations.len() > 0 {
        operations.push(result);
        return run_operations(operations);
    }

    // println!("{}", result);

    vec![result]
}

fn part_one() -> u32 {
    // let hex = String::from(include_str!("test.txt"));
    // let hex = String::from(include_str!("test2.txt"));
    // let hex = String::from(include_str!("test3.txt"));
    // let hex = String::from("8A004A801A8002F478");
    // let hex = String::from("620080001611562C8802118E34");
    // let hex = String::from("C0015000016115A2E0802F182340");
    // let hex = String::from("A0016C880162017C3686B18A3D4780");
    // let hex = String::from("C200B40A82");
    // let hex = String::from("04005AC33890");
    // let hex = String::from("880086C3E88112");
    // let hex = String::from("CE00C43D881120");
    // let hex = String::from("D8005AC2A8F0");
    // let hex = String::from("F600BC2D8F");
    // let hex = String::from("9C005AC2F8F0");
    let hex = String::from("9C0141080250320F1802104A08");
    // let hex = String::from(include_str!("input.txt"));

    let bin = decode_to_binary(hex);
    let packet = make_packet(&bin);
    let mut t = Vec::new();
    let (_, operations) = parse_packet(packet, &mut t);
    operations.reverse();
    println!("{:?}", operations);
    // run_operations(operations);

    0
}

// fn part_two() -> i128 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}
