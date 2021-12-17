#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
#[aoc(day6, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let mut numbers = input.to_vec();
    for _ in 0..80 {
        for j in 0..numbers.len() {
            if numbers [j] == 0 {
                numbers[j] = 6;
                numbers.push(8);
            } else {
                numbers[j] -= 1;
            }
        }
    }
    numbers.len() as i32
}

pub fn f(a :  i32) -> u64{
    return if a > 0 {f(a-9) + f(a-7)} else {1};
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[i32]) -> u64 {
    let mut count: u64 = 0;
    let mut numbers = [(0,0),(1,0),(2,0),(3,0),(4,0),(5,0)];
    for &i in input {
        numbers[i as usize].1 += 1;
    }
    for (num, amount) in numbers {
        count += amount * f(80-num);
    }
    count
}