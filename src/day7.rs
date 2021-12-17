use std::cmp::min;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x|
            x.parse::<i32>().unwrap()
        )
        .collect()
}
#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    let len = input.len();
    let mut arr = input.clone();
    let (_, &mut median, _) = arr.select_nth_unstable(len / 2);
    let mut count = 0;
    for i in arr {
        count += (i - median).abs();
    }
    count
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let sum: i32 = input.iter().sum();
    let avg = sum as f32 / input.len() as f32;
    let (t1, t2) = (avg.ceil() as i32, avg.floor() as i32);

    let mut count1 = 0;
    let mut count2 = 0;
    for &i in input {
        let diff = (i - t1).abs();
        count1 += (diff * (diff + 1)) / 2;
        let diff = (i - t2).abs();
        count2 += (diff * (diff + 1)) / 2;
    }

    min(count1, count2)


}