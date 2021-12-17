#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| {
            line.chars()
                .collect()
        })
        .collect()

}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> u32 {
    let len = input[0].len();
    let mut number: Vec<char> = Vec::new();
    for j in 0..len {
        let mut count_ones = 0;
        for i in 0..input.len() {
            if input[i][j] == '1' {
                count_ones += 1;
            }
        }
        if count_ones >= input.len() / 2 {
            number.push('1');
        } else {
            number.push('0');
        }
    }
    let numberstr = number.iter().collect::<String>();
    let number_int = u32::from_str_radix(&numberstr, 2).unwrap();
    let numberstr_flipped = number.iter().map(|x| if *x == '1' { '0' } else { '1' }).collect::<String>();
    let number_flipped = u32::from_str_radix(&numberstr_flipped, 2).unwrap();
    number_int * number_flipped
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> u32 {
    let len = input[0].len();
    let mut numbers = input.clone();
    //Co2
    for j in 0..len {
        let mut count_zeros = 0;
        for i in 0..numbers.len() {
            if numbers[i][j] == '0' {
                count_zeros += 1;
            }
        }
        let keep : char;
        if count_zeros <= numbers.len() / 2 {
            keep = '0';
        } else {
            keep = '1';
        }
        numbers.retain(|x| x[j] == keep);
        if numbers.len() == 1 {
            break
        }
    }

    let co2_str = numbers[0].iter().collect::<String>();
    let co2 = u32::from_str_radix(&co2_str, 2).unwrap();
    //O2
    let mut numbers = input.clone();
    for j in 0..len {
        let mut count_ones = 0;
        for i in 0..numbers.len() {
            if numbers[i][j] == '1' {
                count_ones += 1;
            }
        }
        let keep : char;
        if count_ones > numbers.len() / 2 || (count_ones == numbers.len() / 2 && numbers.len() % 2 == 0)  {
            keep = '1';
        } else {
            keep = '0';
        }
        numbers.retain(|x| x[j] == keep);
        if numbers.len() == 1 {
            break
        }
    }
    let o2_str = numbers[0].iter().collect::<String>();
    let o2 = u32::from_str_radix(&o2_str, 2).unwrap();
    co2 * o2


}
