

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32] ) -> i32 {
    let mut count = 0;
    let mut last = i32::MAX;
    for &i in input.iter() {
        if i > last {
            count += 1;
        }
        last = i;
    }
    return count;
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut count = 0;
    let mut last = i32::MAX;
    let len = input.len();
    for i in 0..len-2 {
        let curr = input[i] + input[i+1] + input[i+2];
        if curr > last {
            count += 1;
        }
        last = curr;
    }
    return count;
}



