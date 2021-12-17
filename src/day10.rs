use std::collections::LinkedList;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> u64 {
    let mut points = vec![];
    'line: for line in input {
        let mut stack = LinkedList::new();
        for c in line {
            match c {
                '(' => {
                    stack.push_front(c);
                }
                ')' => {
                    let &s = stack.pop_front().unwrap();
                    if s != '(' {
                        continue 'line;
                    }
                }
                '[' => {
                    stack.push_front(c);
                }
                ']' => {
                    let &s = stack.pop_front().unwrap();
                    if s != '[' {
                        continue 'line;
                    }
                }
                '{' => {
                    stack.push_front(c);
                }
                '}' => {
                    let &s = stack.pop_front().unwrap();
                    if s != '{' {
                        continue 'line;
                    }
                }
                '<' => {
                    stack.push_front(c);
                }
                '>' => {
                    let &s = stack.pop_front().unwrap();
                    if s != '<' {
                        continue 'line;
                    }
                }
                _ => break
            }
        }
        println!("{:?}", stack);
        let mut curr = 0;
        for &s in stack {
            curr *= 5;
            match s {
                '(' => {
                    curr += 1;
                }
                '[' => {
                    curr += 2;
                }
                '{' => {
                    curr += 3;
                }
                '<' => {
                    curr += 4;
                }
                _ => {}
            }
        }
        points.push(curr);
    }
    let len = points.len();
    let (_, score, _) = points.select_nth_unstable(len / 2);
    *score
}