#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line|
        line
            .chars()
            .map(|x|
                x as i32 - '0' as i32
            )
            .collect()
    ).collect()
}


fn dfs(y: isize, x: isize, grid:  &mut Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>) -> i32 {
    if seen[y as usize][x as usize] {
        return 0;
    }
    seen[y as usize][x as usize] = true;
    if grid[y as usize][x as usize] == 9 {
        return 0;
    }
    let mut count = 1;
    for (dy, dx) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let ny = y + dy;
        let nx = x + dx;
        if ny >= 0 && ny < grid.len() as isize && nx >= 0 && nx < grid[0].len() as isize {
            count += dfs(ny, nx, grid, seen);
        }
    }
    count
}

// fn print_grid(grid: &Vec<Vec<i32>>) {
//     for row in grid {
//         for cell in row {
//             print!("{}\t", cell);
//         }
//         println!();
//     }
// }

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Vec<i32>>) -> i32 {

    let mut grid = input.clone();
    let mut seen = vec![vec![false; input[0].len()]; input.len()];
    let len = grid.len();
    let width = grid[0].len();
    let mut values = vec![];
    for y in 0..len as isize {
        for x in 0..width as isize {
            let value = dfs(y, x, &mut grid, &mut seen);
            values.push(value);
        }
    }
    println!("{}", dfs(0, 0, &mut grid, &mut seen));
    values.sort();
    values.reverse();
    println!("{:?}", values);
    values[0] * values[1] * values[2]

}