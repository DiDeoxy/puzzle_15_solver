use keyed_priority_queue::KeyedPriorityQueue;
use std::cmp::{Ord, Ordering, PartialOrd, Reverse};

static CORRECT_ORDER: [i32; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
static ROW: [i32; 16] = [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
static COLUMN: [i32; 16] = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];

#[derive(Debug, Copy, Clone)]
struct State {
    moves: Vec<&'static str>,
    f: i32,
    h: i32,
}

// impl std::fmt::Display for State {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(
//             f,
//             "{:?}\n{:?}\n{:?}\n{:?}",
//             &self.order[0..4],
//             &self.order[4..8],
//             &self.order[8..12],
//             &self.order[12..16]
//         )
//     }
// }

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.h.cmp(&other.h))
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.h.cmp(&other.h) == Ordering::Equal
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.h.partial_cmp(&other.h).unwrap()
    }
}

impl State {
    fn init(order: &[i32; 16]) -> State {
        State {
            moves: String::from(""),
            f: 0,
            h: State::calc_g(&order),
        }
    }

    fn find_index(order: &[i32; 16], tile: &i32) -> usize {
        order.iter().position(|&x| x == *tile).unwrap()
    }

    fn calc_g(current: &[i32; 16]) -> i32 {
        let mut g = 0;
        for tile in current.iter() {
            let current_index = State::find_index(current, &tile);
            let correct_index = State::find_index(&CORRECT_ORDER, &tile);
            g += (COLUMN[current_index] - COLUMN[correct_index]).abs()
                + (ROW[current_index] - ROW[correct_index]).abs();
        }
        g
    }

    fn make_move(
        &self,
        order: &[i32; 16],
        dir: fn(usize, [i32; 16]) -> ([i32; 16], &'static str),
    ) -> ([i32; 16], State) {
        let mut old_order = *order;
        let new_f = self.f + 1;
        let new_g = State::calc_g(order);
        let (new_order, from) = dir(State::find_index(&old_order, &0), *order);
        let new_state = State {
            moves: slef.moves.push(from),
            f: new_f,
            h: new_f + new_g,
        };
        return (new_order, new_state);
    }

    fn left(index: usize, mut order: [i32; 16]) -> ([i32; 16], &'static str) {
        order.swap(index, index - 1);
        return (order, "l");
    }

    fn right(index: usize, mut order: [i32; 16]) -> ([i32; 16], &'static str) {
        order.swap(index, index + 1);
        return (order, "r");
    }

    fn up(index: usize, mut order: [i32; 16]) -> ([i32; 16], &'static str) {
        order.swap(index, index - 4);
        return (order, "u");
    }

    fn down(index: usize, mut order: [i32; 16]) -> ([i32; 16], &'static str) {
        order.swap(index, index + 4);
        return (order, "d");
    }

    fn children(&self, order: &[i32; 16]) -> Vec<([i32; 16], State)> {
        let index = State::find_index(order, &0);
        let mut new_states: Vec<([i32; 16], State)> = Vec::new();
        if COLUMN[index] > 0 {
            new_states.push(self.make_move(order, State::left))
        }
        if COLUMN[index] < 3 {
            new_states.push(self.make_move(order, State::right))
        }
        if ROW[index] > 0 {
            new_states.push(self.make_move(order, State::up))
        }
        if ROW[index] < 3 {
            new_states.push(self.make_move(order, State::down))
        }
        new_states
    }
}

fn main() {
    let mut open_states = KeyedPriorityQueue::<[i32; 16], Reverse<State>>::new();
    let start_order = [15, 14, 1, 6, 9, 11, 4, 12, 0, 10, 7, 3, 13, 8, 5, 2];
    open_states.push(start_order, Reverse(State::init(&start_order)));
    let mut closed_states = KeyedPriorityQueue::<[i32; 16], Reverse<State>>::new();

    'outer: while let Some((parent_order, Reverse(parent_state))) = open_states.pop() {
        for (child_order, child_state) in parent_state.children(&parent_order) {
            match open_states.get_priority(&child_order) {
                None => {
                    open_states.push(child_order, Reverse(child_state));
                }
                Some(&Reverse(old_state)) if old_state.h > child_state.h => {
                    open_states.set_priority(&child_order, Reverse(child_state));
                }
                _ => {}
            };
            match closed_states.get_priority(&child_order) {
                Some(&Reverse(old_state)) if old_state.h > child_state.h => {
                    open_states.push(child_order, Reverse(child_state));
                }
                _ => {}
            };
        }
        closed_states.push(parent_order, Reverse(parent_state));

        // for neighbour_State in neighbour_States {
        //     if neighbour_State.order.eq(&CORRECT_ORDER) {
        //         println!(
        //             "The new final configuartion of the State is:\n{}",
        //             neighbour_State
        //         );
        //         println!("The moves used were {}.", neighbour_State.moves);
        //         break 'outer;
        //     }
        //     let neighbour_key = make_key(&neighbour_State);
        //     if open_States.contains_key(&neighbour_key) {
        //         let old_State = open_States.get(&neighbour_key).unwrap();
        //         if old_State.h <= neighbour_State.h {
        //             continue;
        //         } else {
        //             open_States.insert(neighbour_key, neighbour_State);
        //         }
        //     } else if closed_States.contains_key(&neighbour_key) {
        //         let old_State = closed_States.get(&neighbour_key).unwrap();
        //         if old_State.h <= neighbour_State.h {
        //             continue;
        //         } else {
        //             closed_States.insert(neighbour_key, neighbour_State);
        //         }
        //     } else {
        //         open_States.insert(neighbour_key, neighbour_State);
        //     }
        // }
        // closed_States.insert(parent_key, parent_State);
        // println!(
        //     "There are {} entries in the open hashmap.",
        //     open_States.len()
        // );
        // println!(
        //     "There are {} entries in the closed hashmap.",
        //     closed_States.len()
        // )
    }
}

// fn main() {
// let mut open_States = HashMap::new();
// open_States.insert(
//     make_key(&State::new()),
//     State::new(),
// );
// let mut closed_States = HashMap::new();
// closed_States.insert(
//     make_key(&State::new()),
//     State::new(),
// );

// 'outer: while open_States.len() > 0 {
//     let parent_State = open_States.values().min_by_key(|State| State.h).unwrap();
//     let parent_key = make_key(parent_State);
//     let parent_State = open_States.remove(&parent_key).unwrap();
//     let neighbour_States = parent_State.children();

//     println!("Parent f-score: {}", parent_State.f);
//     println!("Parent g-score: {}", parent_State.g);
//     println!("Parent h-score: {}", parent_State.h);

//     for neighbour_State in neighbour_States {
//         if neighbour_State.order.eq(&CORRECT_ORDER) {
//             println!(
//                 "The new final configuartion of the State is:\n{}",
//                 neighbour_State
//             );
//             println!("The moves used were {}.", neighbour_State.moves);
//             break 'outer;
//         }
//         let neighbour_key = make_key(&neighbour_State);
//         if open_States.contains_key(&neighbour_key) {
//             let old_State = open_States.get(&neighbour_key).unwrap();
//             if old_State.h <= neighbour_State.h {
//                 continue;
//             } else {
//                 open_States.insert(neighbour_key, neighbour_State);
//             }
//         } else if closed_States.contains_key(&neighbour_key) {
//             let old_State = closed_States.get(&neighbour_key).unwrap();
//             if old_State.h <= neighbour_State.h {
//                 continue;
//             } else {
//                 closed_States.insert(neighbour_key, neighbour_State);
//             }
//         } else {
//             open_States.insert(neighbour_key, neighbour_State);
//         }
//     }
//     closed_States.insert(parent_key, parent_State);
//     println!(
//         "There are {} entries in the open hashmap.",
//         open_States.len()
//     );
//     println!(
//         "There are {} entries in the closed hashmap.",
//         closed_States.len()
//     )
// }
// }
