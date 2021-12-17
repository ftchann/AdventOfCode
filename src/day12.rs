use std::collections::{HashMap, HashSet};



fn dfs (cave: String, visited: Vec<String>, graph: &HashMap<String, HashSet<String>>) -> Vec<Vec<String>>{
    let str_cave: &str = &cave;
    match str_cave {
        "end" => {
            let mut new: Vec<String> = visited.clone();
            new.push(cave);
            return vec![new]
        },
        _ => {
            let mut ret: Vec<Vec<String>> = vec![];
            let mut new: Vec<String> = visited.clone();
            new.push(cave.clone());
            let neighbors = graph.get(&cave).unwrap();

            let mut visited_small: Vec<String> = new.clone();
            visited_small = visited_small.into_iter().filter(|x| {
                let first = x.chars().next().unwrap();
                if first.is_uppercase() {
                    return false;
                }
               true
            }).collect();

            let mut dedup = visited_small.clone();

            dedup.sort();
            dedup.dedup();




            let hasdup = dedup.len() != visited_small.len();



            for n in neighbors {
                if n.ne("start") && (!visited_small.contains(n) || !hasdup) {
                    let mut ans: Vec<Vec<String>> = dfs(n.clone(), new.clone(), graph);
                    ret.append(&mut ans);
                }
            }
            return ret
        }
    }
}

#[aoc(day12, part2)]
pub fn solve_part1(input: &str) -> u64 {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    let lines = input.lines();
    for line in lines {
        let mut splitted = line.split("-");
        let astr = splitted.next().unwrap();
        let bstr = splitted.next().unwrap();
        let a: String = astr.into();
        let b: String = bstr.into();

        let neighbours_a = graph.get_mut(&a);
        match neighbours_a {
            Some(neighbours) => {
                let _ = neighbours.insert(b.clone());
            },
            None => {
                let mut neighbours = HashSet::new();
                neighbours.insert(b.clone());
                graph.insert(a.clone(), neighbours);
            }
        }
        let neighbours_b = graph.get_mut(&b);
        match neighbours_b {
            Some(neighbours) => {
                let _ = neighbours.insert(a.clone());
            },
            None => {
                let mut neighbours = HashSet::new();
                neighbours.insert(a.clone());
                graph.insert(b.clone(), neighbours);
            }
        }
    }
    let start: String = "start".into();
    let ans = dfs(start, vec![], &graph);
    let mut dupe = ans.clone();
    dupe.sort();
    dupe.dedup();
    // println!("{:?}", dupe);
    dupe.len() as u64
}