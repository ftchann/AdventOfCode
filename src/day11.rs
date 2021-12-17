#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| line.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
    ).collect()

}

fn flash (x: isize, y: isize, grid: &mut Vec<Vec<i32>>) {
    let len = grid.len();
    if x >= 0 && x < len as isize && y >= 0 && y < len as isize {
        grid[x as usize][y as usize] += 1;
        if grid[x as usize][y as usize] == 10 {
            let neighbours = [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y), (x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)];
            for (x1, y1) in neighbours {
                flash(x1, y1, grid);
            }
        }
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<Vec<i32>>) -> u64 {
    let mut grid = input.clone();
    let mut count = 0;
    let rounds = 100;
    let len = grid.len();
    for _ in 0..rounds {
        for i in 0..len as isize {
            for j in 0..len as isize {
                if grid[i as usize][j as usize] == 10 {
                    continue;
                }
                grid[i as usize][j as usize] += 1;
                if grid[i as usize][j as usize] == 10 {
                    let neighbours = [(i-1, j-1), (i-1, j), (i-1, j+1), (i, j-1), (i, j+1), (i+1, j-1), (i+1, j), (i+1, j+1)];
                    for (x, y) in neighbours {
                        flash(x, y, &mut grid);
                    }
                }
            }
        }
        for i in 0..len {
            for j in 0..len {
                if grid[i][j] > 9 {
                    grid[i][j] = 0;
                    count += 1;
                }
            }
        }
    }

    count
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<Vec<i32>>) -> u64 {
    let mut grid = input.clone();
    let len = grid.len();
    for round in 1..900 {
        for i in 0..len as isize {
            for j in 0..len as isize {
                if grid[i as usize][j as usize] == 10 {
                    continue;
                }
                grid[i as usize][j as usize] += 1;
                if grid[i as usize][j as usize] == 10 {
                    let neighbours = [(i-1, j-1), (i-1, j), (i-1, j+1), (i, j-1), (i, j+1), (i+1, j-1), (i+1, j), (i+1, j+1)];
                    for (x, y) in neighbours {
                        flash(x, y, &mut grid);
                    }
                }
            }
        }
        let mut count = 0;
        for i in 0..len {
            for j in 0..len {
                if grid[i][j] > 9 {
                    grid[i][j] = 0;
                    count += 1;
                }
            }
        }
        if count == len * len {
            println!("{:?}", grid);
            return round as u64;
        }
    }

    0
}