use num::BigInt;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> String {
    let bigint = BigInt::parse_bytes(input.as_bytes(), 16).unwrap();
    let res = bigint.to_str_radix(2);
    let len_mod = res.len() % 4;
    match len_mod {
        0 => res,
        1 => format!("000{}", res),
        2 => format!("00{}", res),
        3 => format!("0{}", res),
        _ => panic!("Invalid input"),
    }
}

fn parse_literal(s: &String, i: usize) -> (usize, u64) {
    let mut ni = i;
    let mut res = String::new();
    loop {
        let curr = &s[ni + 1..ni + 5];
        res.push_str(curr);
        if s.as_bytes()[ni] == '0' as u8 {
            let number = u64::from_str_radix(res.as_str(), 2).unwrap();
            return (ni + 5, number);
        }
        ni += 5;
    }
}

fn parse_packets(s: &String, i: usize, versions: &mut Vec<u64>) -> (usize, u64) {
    let version = u64::from_str_radix(&s[i..i + 3], 2).unwrap();
    let tid = u8::from_str_radix(&s[i + 3..i + 6], 2).unwrap();

    versions.push(version);
    if tid == 4 {
        let (ni, val) = parse_literal(s, i + 6);
        return (ni, val);
    }
    let lid = s.as_bytes()[i + 6];
    let mut ni;
    let mut values = Vec::new();

    if lid == '0' as u8 {
        let total_len = u32::from_str_radix(&s[i + 7..i + 22], 2).unwrap();
        ni = i + 22;
        while ni < i + 22 + total_len as usize {
            let (ni2, val) = parse_packets(s, ni, versions);
            ni = ni2;
            values.push(val);
        }
    } else {
        let amount_packets = u8::from_str_radix(&s[i + 7..i + 18], 2).unwrap();
        ni = i + 18;
        for _ in 0..amount_packets {
            let (ni2, val) = parse_packets(s, ni, versions);
            ni = ni2;
            values.push(val);
        }
    }

    let res: u64 = match tid {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => (values[0] > values[1]) as u64,
        6 => (values[0] < values[1]) as u64,
        7 => (values[0] == values[1]) as u64,
        _ => panic!("Invalid TID"),
    };

    return (ni, res);
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &String) -> u64 {
    let mut versions = Vec::new();
    parse_packets(input, 0, &mut versions);
    let sum: u64 = versions.iter().sum();
    sum
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &String) -> u64 {
    let mut versions = Vec::new();
    let (_ , val) = parse_packets(input, 0, &mut versions);
    val
}