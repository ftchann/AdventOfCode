use std::cmp::Ordering;
use std::collections::{BinaryHeap};

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line|
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        ).collect()
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coordinate {
    x: usize,
    y: usize
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Square {
    pos: Coordinate,
    cost: usize,
}

impl PartialOrd<Self> for Square {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Square {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(grid: &Vec<Vec<usize>>) -> usize {
    let len = grid.len();
    let width = grid[0].len();
    let maxint = usize::MAX;
    let mut distances = vec![vec![maxint; width]; len];
    let mut queue = BinaryHeap::new();

    queue.push(Square { pos: Coordinate{x: 0, y: 0}, cost: 0 });
    distances[0][0] = 0;
    while !queue.is_empty() {
        let square = queue.pop().unwrap();
        let x = square.pos.x;
        let y = square.pos.y;
        let dist = square.cost;
        if y > 0 {
            let new_dist = dist + grid[y-1][x];
            if new_dist < distances[y-1][x] {
                distances[y-1][x] = new_dist;
                queue.push(Square { pos: Coordinate{x, y: y-1}, cost: new_dist });
            }
        }
        if x > 0 {
            let new_dist = dist + grid[y][x-1];
            if new_dist < distances[y][x-1] {
                distances[y][x-1] = new_dist;
                queue.push(Square { pos: Coordinate{x: x-1, y}, cost: new_dist });
            }
        }
        if y < len-1 {
            let new_dist = dist + grid[y+1][x];
            if new_dist < distances[y+1][x] {
                distances[y+1][x] = new_dist;
                queue.push( Square { pos: Coordinate{x, y: y+1}, cost: new_dist });
            }
        }
        if x < width-1 {
            let new_dist = dist + grid[y][x+1];
            if new_dist < distances[y][x+1] {
                distances[y][x+1] = new_dist;
                queue.push( Square { pos: Coordinate{x: x+1, y}, cost: new_dist });
            }
        }
    }
    // distances.iter().for_each(|it| {
    //     println!("{:?}", it);
    // });
    distances[len-1][width-1]
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    let len = input.len();
    let width = input[0].len();
    let maxint = usize::MAX;
    let mut distances = vec![vec![maxint; width*5]; len * 5];

    let mut grid = vec![vec![0; width*5]; len*5];

    for y_mul in 0..5 {
        for x_mul in 0..5 {
            for y in 0..len {
                for x in 0..width {
                    grid[y_mul*len+y][x_mul*width+x] = (input[y][x] + y_mul + x_mul - 1) % 9 + 1;
                }
            }
        }
    }

    // grid.iter().for_each(|it| {
    //     println!("{:?}", it);
    // });

    let gridlen = grid.len();
    let gridwidth = grid[0].len();


    let mut queue = BinaryHeap::new();

    queue.push(Square { pos: Coordinate{x: 0, y: 0}, cost: 0 });
    distances[0][0] = 0;
    while !queue.is_empty() {

        let square = queue.pop().unwrap();
        let x = square.pos.x;
        let y = square.pos.y;
        if x == gridwidth-1 && y == gridlen-1 {
            return distances[y][x];
        }
        let dist = square.cost;
        if y > 0 {
            let new_dist = dist + grid[y-1][x];
            if new_dist < distances[y-1][x] {
                distances[y-1][x] = new_dist;
                queue.push(Square { pos: Coordinate{x, y: y-1}, cost: new_dist });
            }
        }
        if x > 0 {
            let new_dist = dist + grid[y][x-1];
            if new_dist < distances[y][x-1] {
                distances[y][x-1] = new_dist;
                queue.push(Square { pos: Coordinate{x: x-1, y}, cost: new_dist });
            }
        }
        if y < gridlen-1 {
            let new_dist = dist + grid[y+1][x];
            if new_dist < distances[y+1][x] {
                distances[y+1][x] = new_dist;
                queue.push( Square { pos: Coordinate{x, y: y+1}, cost: new_dist });
            }
        }
        if x < gridwidth-1 {
            let new_dist = dist + grid[y][x+1];
            if new_dist < distances[y][x+1] {
                distances[y][x+1] = new_dist;
                queue.push( Square { pos: Coordinate{x: x+1, y}, cost: new_dist });
            }
        }
    }
    // distances.iter().for_each(|it| {
    //     println!("{:?}", it);
    // });
    distances[gridlen-1][gridwidth-1]
}

