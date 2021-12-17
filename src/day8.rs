use itertools::Itertools;
pub fn input_generator(input: &str) -> Vec<Vec<Vec<&str>>> {
    let t: Vec<Vec<Vec<&str>>> = input
        .lines()
        .map(|line| {
            let arr: Vec<Vec<&str>> = line
                .split('|')
                .map(|x| x.split_whitespace().collect::<Vec<&str>>())
                .collect();
            arr
        })
        .collect();

    t
}



#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let parsed = input_generator(input);
    let digits = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
    let digits_int_cmp = digits.map(|digit|
                                    digit.chars().map(|y|
                                        y as usize - 97
                                    ).collect::<Vec<usize>>()

    );
    let mut count = 0;
    for row in parsed {
        let items: Vec<usize> = vec![0,1,2,3,4,5,6,];
        let mut good_perm = items.clone();
        'perm: for perm in items.iter().permutations(items.len()) {
            for digit in row[0].iter() {
                let digits_int = digit.chars().map(|y|
                    y as usize - 97
                ).collect::<Vec<usize>>();
                let mut real_digit = digits_int.iter().map(|&x|{
                    let &num = perm[x];
                    num
                }).collect::<Vec<usize>>();

                real_digit.sort();
                // println!("{:?}", real_digit);

                if !digits_int_cmp.contains(&real_digit) {
                    continue 'perm;
                }
            }
            good_perm = perm.iter().map(|&&x| x ).collect();
            break;
        }
        for digit in row[1].iter() {
            let digits_int = digit.chars().map(|y|
                y as usize - 97
            ).collect::<Vec<usize>>();
            let mut real_digit = digits_int.iter().map(|&x|{
                let num = good_perm[x];
                num
            }).collect::<Vec<usize>>();
            real_digit.sort();
            let x = digits_int_cmp.iter().position(|y| y.clone() == real_digit).unwrap();
            if [1,4,7,8].contains(&x) {
                count += 1;
            }
        }

    }
    count
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let parsed = input_generator(input);
    let digits = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
    let digits_int_cmp = digits.map(|digit|
        digit.chars().map(|y|
            y as usize - 97
        ).collect::<Vec<usize>>()
    );
    let mut count = 0;
    for row in parsed {
        let items: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, ];
        let mut good_perm = items.clone();
        'perm: for perm in items.iter().permutations(items.len()) {
            for digit in row[0].iter() {
                let digits_int = digit.chars().map(|y|
                    y as usize - 97
                ).collect::<Vec<usize>>();
                let mut real_digit = digits_int.iter().map(|&x| {
                    let &num = perm[x];
                    num
                }).collect::<Vec<usize>>();

                real_digit.sort();
                // println!("{:?}", real_digit);

                if !digits_int_cmp.contains(&real_digit) {
                    continue 'perm;
                }
            }
            good_perm = perm.iter().map(|&&x| x).collect();
            break;
        }
        let mut num = 0;
        let base: usize = 10;
        for (i, digit) in row[1].iter().rev().enumerate() {
            let digits_int = digit.chars().map(|y|
                y as usize - 97
            ).collect::<Vec<usize>>();
            let mut real_digit = digits_int.iter().map(|&x| {
                let num = good_perm[x];
                num
            }).collect::<Vec<usize>>();
            real_digit.sort();
            let x = digits_int_cmp.iter().position(|y| y.clone() == real_digit).unwrap();
            let curr = x * base.pow(i as u32);
            num += curr;

        }

        count += num;
    }
    count as i32
}