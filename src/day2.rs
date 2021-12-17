type Direction = (char, i32);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Direction> {
    input.lines()
     .map(|line| {
         let mut split = line.split_whitespace();
         let direction = split.next().unwrap().chars().next().unwrap();
         let distance = split.next().unwrap().parse::<i32>().unwrap();
         (direction, distance)
     }).collect()

}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Direction] ) -> i32 {
    let pos = input.iter().fold((0,0), |pos, (dir, dist)| {
        match dir {
            'f' => (pos.0 + dist, pos.1),
            'd' => (pos.0 , pos.1 + dist),
            'u' => (pos.0, pos.1 - dist),
            _ => panic!("Unknown direction {}", dir)
        }
    });
    return pos.0 * pos.1;

}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Direction] ) -> i32 {
    let pos = input.iter().fold((0,0,0), |pos, (dir, dist)| {
        match dir {
            'f' => (pos.0 + dist, pos.1 + dist * pos.2, pos.2),
            'd' => (pos.0 , pos.1, pos.2 + dist),
            'u' => (pos.0, pos.1, pos.2 - dist),
            _ => panic!("Unknown direction {}", dir)
        }
    });
    return pos.0 * pos.1;

}