use std::cmp::max;

#[aoc(day17, part1)]
pub fn solve_part1(_input: &str) -> i32 {
    let target = (20, 30, 10, -5);
    // let target = (241, 275, -75, -49); // (xl, yl, xh, yh)
    let mut max_y = 0;
    for x in 0..100 {
        for y in -100..100 {
            let mut vel = (x, y);
            let mut pos = (0, 0);
            for _ in 0..400 {
                pos = (pos.0 + vel.0, pos.1 + vel.1);
                if  target.0 <= pos.0 && pos.0 <= target.1 && target.2 <= pos.1 && pos.1 <= target.3 {
                    max_y = max(max_y, pos.1);
                }
                vel.0 = max(0, vel.0 - 1);
                vel.1 = vel.1 - 1;
            }
        }
    }
    max_y
}
