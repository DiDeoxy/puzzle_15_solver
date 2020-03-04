use std::collections::HashMap;

static CORRECT_ORDER: [i32; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
static ROW: [i32; 16] = [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
static COLUMN: [i32; 16] = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];

#[derive(Debug, Clone)]
struct Board {
    order: [i32; 16],
    moves: String,
    f: i32,
    g: i32,
    h: i32,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}\n{:?}\n{:?}\n{:?}",
            &self.order[0..4],
            &self.order[4..8],
            &self.order[8..12],
            &self.order[12..16]
        )
    }
}

impl Board {
    fn new() -> Board {
        let new_order: [i32; 16] = [15, 14, 1, 6, 9, 11, 4, 12, 0, 10, 7, 3, 13, 8, 5, 2];

        let current_dist = calc_distance(&new_order, &CORRECT_ORDER);

        Board {
            order: new_order,
            moves: String::from(""),
            f: 0,
            g: current_dist,
            h: 0,
        }
    }

    fn neighbours(&self) -> Vec<Board> {
        let mut new_boards: Vec<Board> = Vec::new();
        let current_index = find_index(&self.order, &0);
        if COLUMN[current_index] > 0 {
            new_boards.push(self.make_move(left))
        }
        if COLUMN[current_index] < 3 {
            new_boards.push(self.make_move(right))
        }
        if ROW[current_index] > 0 {
            new_boards.push(self.make_move(up))
        }
        if ROW[current_index] < 3 {
            new_boards.push(self.make_move(down))
        }
        new_boards
    }

    fn make_move(&self, dir: fn(usize, [i32; 16]) -> ([i32; 16], String)) -> Board {
        let current_index = find_index(&self.order, &0);
        let (new_order, from) = dir(current_index, self.order);
        let new_f = self.f + 1;
        let new_g = calc_distance(&new_order, &CORRECT_ORDER);
        Board {
            order: new_order,
            moves: format!("{}{}", &self.moves, &from),
            f: new_f,
            g: new_g,
            h: new_f + new_g,
        }
    }
}

fn left(index: usize, mut order: [i32; 16]) -> ([i32; 16], String) {
    order.swap(index, index - 1);
    return(order, String::from("l"))
}

fn right(index: usize, mut order: [i32; 16]) -> ([i32; 16], String)  {
    order.swap(index, index + 1);
    return(order, String::from("r"))
}

fn up(index: usize, mut order: [i32; 16]) -> ([i32; 16], String)  {
    order.swap(index, index - 4);
    return(order, String::from("u"))
}

fn down(index: usize, mut order: [i32; 16]) -> ([i32; 16], String)  {
    order.swap(index, index + 4);
    return(order, String::from("d"))
}


fn find_index(order: &[i32; 16], tile: &i32) -> usize {
    order.iter().position(|&x| x == *tile).unwrap()
}

fn calc_distance(current: &[i32; 16], correct: &[i32; 16]) -> i32 {
    let mut dist = 0;
    for tile in correct.iter() {
        if *tile != 0i32 {
            let current_index = find_index(current, &tile);
            let correct_index = find_index(correct, &tile);
            dist += (COLUMN[current_index] - COLUMN[correct_index]).abs()
                + (ROW[current_index] - ROW[correct_index]).abs();
        }
        
    }
    dist
}

fn make_key(board: &Board) -> String {
    board
        .order
        .iter()
        .fold(String::from(""), |key, x| format!("{}{}", key, x))
}

fn main() {
    let mut open_boards = HashMap::new();
    open_boards.insert(
        make_key(&Board::new()),
        Board::new(),
    );
    let mut closed_boards = HashMap::new();
    closed_boards.insert(
        make_key(&Board::new()),
        Board::new(),
    );

    'outer: while open_boards.len() > 0 {
        let parent_board = open_boards.values().min_by_key(|board| board.h).unwrap();
        let parent_key = make_key(parent_board);
        let parent_board = open_boards.remove(&parent_key).unwrap();
        let neighbour_boards = parent_board.neighbours();

        println!("Parent f-score: {}", parent_board.f);
        println!("Parent g-score: {}", parent_board.g);
        println!("Parent h-score: {}", parent_board.h);

        for neighbour_board in neighbour_boards {
            if neighbour_board.order.eq(&CORRECT_ORDER) {
                println!(
                    "The new final configuartion of the board is:\n{}",
                    neighbour_board
                );
                println!("The moves used were {}.", neighbour_board.moves);
                break 'outer;
            }
            let neighbour_key = make_key(&neighbour_board);
            if open_boards.contains_key(&neighbour_key) {
                let old_board = open_boards.get(&neighbour_key).unwrap();
                if old_board.h <= neighbour_board.h {
                    continue;
                } else {
                    open_boards.insert(neighbour_key, neighbour_board);
                }
            } else if closed_boards.contains_key(&neighbour_key) {
                let old_board = closed_boards.get(&neighbour_key).unwrap();
                if old_board.h <= neighbour_board.h {
                    continue;
                } else {
                    closed_boards.insert(neighbour_key, neighbour_board);
                }
            } else {
                open_boards.insert(neighbour_key, neighbour_board);
            }
        }
        closed_boards.insert(parent_key, parent_board);
        println!(
            "There are {} entries in the open hashmap.",
            open_boards.len()
        );
        println!(
            "There are {} entries in the closed hashmap.",
            closed_boards.len()
        )
    }
}