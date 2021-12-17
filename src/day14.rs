use std::collections::HashMap;

type EL = (char, char);
type ElMap = HashMap<EL,u64>;
type Reactions = HashMap<EL,(EL,EL)>;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> (ElMap, Reactions, char, char) {

    let mut lines = input.lines();
    let first = lines.next().unwrap();

    let chars = first.chars().collect::<Vec<char>>();

    let front = chars[1..].to_vec();
    let back = chars[0..chars.len() - 1].to_vec();
    let zipped: Vec<(&char, &char)> = back.iter().zip(front.iter()).collect();
    let mut elements: ElMap = HashMap::new();
    for (&a, &b) in zipped {
        let el = (a, b);
        let count = elements.entry(el).or_insert(0);
        *count += 1;
    }
    // read whiteline
    lines.next();

    let mut reactions: Reactions = HashMap::new();
    for line in lines {
        let split = line.split(" -> ").collect::<Vec<&str>>();
        let first = split[0].chars().collect::<Vec<char>>();
        let left = (first[0], first[1]);
        let second = split[1].chars().collect::<Vec<char>>();
        let right = second[0];
        let r1 = (left.0, right);
        let r2 = (right, left.1);
        reactions.insert(left, (r1, r2));
    }

    (elements, reactions, chars[0], chars[chars.len() - 1])
}

#[aoc(day14, part1)]
pub fn solve_part1((elements, reactions, start, end): &(ElMap, Reactions, char, char)) -> u64 {
    let mut elems = elements.clone();
    for _ in 0..40 {
        let mut new_elems: ElMap = HashMap::new();
        for (el, n) in &elems {
            let (r1, r2) = reactions.get(&el).unwrap();
            let n1 = new_elems.entry(*r1).or_insert(0);
            *n1 += n;
            let n2 = new_elems.entry(*r2).or_insert(0);
            *n2 += n;
        }
        elems = new_elems;
    }
    let mut char_count: HashMap<char, u64> = HashMap::new();
    for (el, n) in &elems {
        let char1 = char_count.entry(el.0).or_insert(0);
        *char1 += n;
        let char2 = char_count.entry(el.1).or_insert(0);
        *char2 += n;
    }
    let char = char_count.entry(*start).or_insert(0);
    *char += 1;
    let char = char_count.entry(*end).or_insert(0);
    *char += 1;

    let max = char_count.values().max().unwrap();
    let min = char_count.values().min().unwrap();


    (max - min) / 2
}

