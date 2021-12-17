use std::collections::HashSet;
use std::iter::FromIterator;

type Point = (i64, i64);
type Fold = (char, i64);

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (Vec<Point>, Vec<Fold>){
   let mut lines = input.lines().into_iter();
   let mut points = Vec::new();
   let mut folds = Vec::new();

   let mut line = lines.next().unwrap();

    while !line.is_empty() {
        let parsed = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        points.push((parsed[0], parsed[1]));
        line = lines.next().unwrap();
    }
    let mut line_opt = lines.next();
    while line_opt.is_some() {
        let line = line_opt.unwrap();
        let splitted = line.split_whitespace().collect::<Vec<&str>>();
        let relevant = splitted[2];
        let blub = relevant.split("=").collect::<Vec<&str>>();
        let axis = blub[0].chars().next().unwrap();
        let value = blub[1].parse::<i64>().unwrap();

        let fold = (axis, value);
        folds.push(fold);

        line_opt = lines.next();
    }

    (points, folds)
}

#[aoc(day13, part1)]
pub fn solve_part1((points_in, folds): &(Vec<Point>, Vec<Fold>)) -> u64{
    let mut points: HashSet<Point> = HashSet::from_iter(points_in.clone());

    // println!("--------------");
    for fold in folds {
         match fold.0 {
            'x' => {
                let mut new_points = HashSet::new();
                for point in points.iter() {
                    let axis = fold.1;
                    let x = point.0;
                    let y = point.1;
                    if x == axis {
                        // do nothing
                    } else if x  > axis {
                        let diff = x - axis;
                        new_points.insert((axis -diff , y));
                    } else {
                        new_points.insert(point.clone());
                    }
                }
                points.clear();
                points.extend(new_points);
            },
            'y' => {
                let mut new_points = HashSet::new();
                let axis = fold.1;
                for point in points.iter() {


                    let x = point.0;
                    let y = point.1;
                    if y == axis {
                        // do nothing
                        println!("{} {}", x, y);
                    } else if y > axis {
                        let diff = y - axis;
                        new_points.insert((x , axis - diff));
                    } else {
                        new_points.insert(point.clone());
                    }
                }
                // println!("{:?}", new_points);
                points.clear();
                points.extend(new_points);
            },
            _ => panic!("Unknown axis")
         }
    }
    let max_y = points.iter().map(|x| x.1).max().unwrap();
    let max_x = points.iter().map(|x| x.0).max().unwrap();
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if points.contains(&(x, y)) {
                print!("$");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    // Count points
    points.len() as u64
}