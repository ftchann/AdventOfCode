type Point = (usize, usize);
type Arrow = (Point, Point);


#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Arrow> {
    input
        .lines()
        .map(|line| {
            let pointsstr = line.split("->");
            let points: Vec<Point> = pointsstr.map(|point| {
                let mut point = point.split(",");
                let x = point.next().unwrap().trim().parse::<usize>().unwrap();
                let y = point.next().unwrap().trim().parse::<usize>().unwrap();
                (x, y)
            }).collect();
            (points[0], points[1])
        })
        .collect()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Arrow]) -> i32 {
    const LEN: usize = 1000;
    let mut table = [[0; LEN]; LEN];
    for &((x1, y1), (x2, y2)) in input {
        if x1 == x2 {
            let (b, e): (usize, usize) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

            for y in b..e + 1 {
                table[y][x1] += 1;
            }
        } else if y1 == y2 {
            let (b, e) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };

            for x in b..e+1 {
                table[y1][x] += 1;
            }
        } else {
            let x1 = x1 as i32;
            let y1 = y1 as i32;
            let x2 = x2 as i32;
            let y2 = y2 as i32;

            let len = (x2 - x1).abs();

            let dx = (x2 - x1) / len;
            let dy = (y2 - y1) / len;
            for i in 0..len+1 {
                let x = x1 + i * dx;
                let y = y1 + i * dy;
                table[y as usize][x as usize] += 1;
            }

        }
    }
    let mut count = 0;
    for x in 0..LEN {
        for y in 0..LEN {
            if table[x][y] > 1 {
                count += 1;
            }
        }
    }

    count
}