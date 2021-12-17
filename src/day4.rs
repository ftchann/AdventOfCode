
type Field = (u32, bool);
type Board = (bool, [[Field; 5]; 5]);

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<Board>, Vec<u32>){
    let mut lines = input.lines();
    let numbers = lines.next().unwrap().split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut boards: Vec<Board> = Vec::new();
    loop {
        let empty = lines.next();
        if let None = empty {
            break;
        }
        let mut board: Board = (false, [[(0,false); 5]; 5]);
        for i in 0..5 {

            let line = lines.next().unwrap();

            let row = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            for j in 0..5 {
                board.1[i][j] = (row[j], false);
            }

        }
        boards.push(board);
    }
    (boards, numbers)


}


#[aoc(day4, part2)]
pub fn solve_part2((boards, numbers): &(Vec<Board>, Vec<u32>)) -> u32 {

    let mut boards = boards.clone();
    let numbers = numbers.clone();

    let mut winner: Board = (false, [[(0,false); 5]; 5]);
    let mut lastnumber = 0;

    let mut len = boards.len();

    'out: for number in numbers {
        for board in boards.iter_mut() {
            for i in 0..5 {
                for j in 0..5 {
                    if board.1[i][j].0 == number {
                        board.1[i][j] = (number, true);
                    }
                }
            }
            if board.0 {
                continue;
            }
            let mut valid = false;
            // Check for valid board
            for i in 0..5 {
                let mut ok = true;
                for j in 0..5 {
                    if board.1[i][j].1 == false {
                        ok = false;
                    }
                }
                if ok {
                    // board is valid
                    winner = board.clone();
                    lastnumber = number;
                    valid = true;
                }
            }
            for j in 0..5 {
                let mut ok = true;
                for i in 0..5 {
                    if board.1[i][j].1 == false {
                        ok = false;
                    }
                }
                if ok {
                    // board is valid
                    winner = board.clone();
                    lastnumber = number;
                    valid = true;
                }
            }
            if valid {
                board.0 = true;
                len -= 1;
                if len == 0 {
                    break 'out;
                }
            }
        }
    }
    let mut sum: u32 = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !winner.1[i][j].1 {
                sum += winner.1[i][j].0;
            }
        }
    }

    lastnumber * sum
}