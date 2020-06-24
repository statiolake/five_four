// use rand::seq::SliceRandom;
// use rand::Rng;
use std::collections::HashMap;
// use std::fmt;
// use std::io::prelude::*;
// use std::io::{stdin, stdout};
use std::cmp::max;
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref STOP_DRAW: Arc<Eval> = Arc::new(Eval {
        children: vec![],
        value: 0,
    });
}

const LINE: &[u64] = &[
    0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_01_01_01,
    0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_01_01_01_00,
    0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_01_01_01_00_00_00_00_00,
    0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_01_01_01_00_00_00_00_00_00,
    0b00_00_00_00_00_00_00_00_00_00_00_01_01_01_01_00_00_00_00_00_00_00_00_00_00,
    0b00_00_00_00_00_00_00_00_00_00_01_01_01_01_00_00_00_00_00_00_00_00_00_00_00,
    0b00_00_00_00_00_00_01_01_01_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    0b00_00_00_00_00_01_01_01_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    0b00_01_01_01_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    0b01_01_01_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    0b00_00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01,
    0b00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00,
    0b00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00,
    0b00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00,
    0b00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00,
    0b00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00,
    0b00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00,
    0b00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00,
    0b00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00,
    0b01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00_00,
    0b00_01_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00,
    0b00_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_01,
    0b01_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_00,
    0b00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_01_00,
    0b00_00_00_01_00_00_00_01_00_00_00_01_00_00_00_01_00_00_00_00_00_00_00_00_00,
    0b00_00_00_00_00_00_00_00_01_00_00_00_01_00_00_00_01_00_00_00_01_00_00_00_00,
    0b00_00_00_00_01_00_00_00_01_00_00_00_01_00_00_00_01_00_00_00_00_00_00_00_00,
    0b00_00_00_00_00_00_00_00_00_01_00_00_00_01_00_00_00_01_00_00_00_01_00_00_00,
];

const SIZE: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    None,
    P1,
    P2,
    Draw,
}

impl From<u64> for State {
    fn from(x: u64) -> State {
        match x {
            1 => State::P1,
            2 => State::P2,
            3 => State::Draw,
            _ => State::None,
        }
    }
}

// 2 ビット × 25 = 50 ビット
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Board(u64);

impl Board {
    fn new() -> Board {
        Board(0)
    }

    #[must_use]
    fn set(self, y: usize, x: usize, player: State) -> Board {
        assert!(self.is_available(y, x));
        Board(self.0 | ((player as u64) << ((y * SIZE + x) * 2)))
    }

    fn get(self, y: usize, x: usize) -> State {
        State::from((self.0 >> ((y * SIZE + x) * 2)) & 0b11)
    }

    fn is_available(self, y: usize, x: usize) -> bool {
        self.get(y, x) == State::None
    }

    #[must_use]
    fn rotate_90(self) -> Board {
        let mut ans = Board::new();
        for y in 0..SIZE {
            for x in 0..SIZE {
                ans = ans.set(SIZE - x - 1, y, self.get(y, x));
            }
        }

        ans
    }

    fn print(self) {
        println!("+{}+", "-".repeat(SIZE));
        for y in 0..SIZE {
            print!("|");
            for x in 0..SIZE {
                print!(
                    "{}",
                    match self.get(y, x) {
                        State::None => ' ',
                        State::P1 => 'o',
                        State::P2 => 'x',
                        _ => panic!("internal error!"),
                    }
                );
            }
            println!("|");
        }
        println!("+{}+", "-".repeat(SIZE));
    }

    fn settled(self) -> State {
        for player in 0..2 {
            for &line in LINE {
                if ((self.0 >> player) & line).count_ones() >= 4 {
                    return State::from(player + 1);
                }
            }
        }

        if self.0.count_ones() == 25 {
            return State::Draw;
        }

        State::None
    }

    fn eval(mut self, memo: &mut HashMap<Board, Arc<Eval>>, depth: usize) -> Arc<Eval> {
        if depth > 7 {
            return Arc::clone(&*STOP_DRAW);
        }

        if depth <= 2 {
            println!("evaluating at depth {} of board {:050b}", depth, self.0);
        }

        // 単に回転したものは同じとみなす。
        for _ in 0..4 {
            // この局面が保存してあるなら OK
            if let Some(eval) = memo.get(&self) {
                return Arc::clone(eval);
            }
            self = self.rotate_90();
        }

        let current = State::from(((depth & 1) + 1) as u64);

        let settled = self.settled();
        if settled != State::None {
            // 必ず次の手番で判定されるため。
            assert_ne!(settled, current);
            let value = if settled == State::Draw { 0 } else { -1 };

            let eval = Arc::new(Eval {
                children: vec![],
                value,
            });

            memo.insert(self, Arc::clone(&eval));
            return eval;
        }

        let mut value = -1;
        let mut children = Vec::new();
        for y in 0..5 {
            for x in 0..5 {
                if !self.is_available(y, x) {
                    continue;
                }

                let next = self.set(y, x, current);
                let eval = next.eval(memo, depth + 1);

                // 自分必勝かどうかは次相手が必敗かどうかなので反転する。
                value = max(value, -eval.value);

                children.push((y, x, eval));
            }
        }

        let eval = Arc::new(Eval { children, value });

        memo.insert(self, Arc::clone(&eval));

        eval
    }
}

#[test]
fn test_board() {
    let mut board = Board::new();
    board = board.set(0, 0, State::P1);
    board = board.set(1, 0, State::P1);
    board = board.set(2, 0, State::P1);
    assert_eq!(board.settled(), State::None);
    board = board.set(3, 0, State::P1);
    assert_eq!(board.settled(), State::P1);

    let mut board = Board::new();
    board = board.set(1, 0, State::P1);
    board = board.set(2, 0, State::P1);
    board = board.set(3, 0, State::P1);
    assert_eq!(board.settled(), State::None);
    board = board.set(4, 0, State::P1);
    assert_eq!(board.settled(), State::P1);

    let mut board = Board::new();
    board = board.set(0, 0, State::P1);
    board = board.set(0, 1, State::P1);
    board = board.set(0, 2, State::P1);
    assert_eq!(board.settled(), State::None);
    board = board.set(0, 3, State::P1);
    assert_eq!(board.settled(), State::P1);

    let mut board = Board::new();
    board = board.set(0, 1, State::P1);
    board = board.set(0, 2, State::P1);
    board = board.set(0, 3, State::P1);
    assert_eq!(board.settled(), State::None);
    board = board.set(0, 4, State::P1);
    assert_eq!(board.settled(), State::P1);

    let mut board = Board::new();
    board = board.set(0, 0, State::P1);
    board = board.set(1, 1, State::P1);
    board = board.set(2, 2, State::P1);
    assert_eq!(board.settled(), State::None);
    board = board.set(3, 3, State::P1);
    assert_eq!(board.settled(), State::P1);

    let mut board = Board::new();
    board = board.set(0, 0, State::P2);
    board = board.set(1, 1, State::P2);
    board = board.set(2, 2, State::P2);
    assert_eq!(board.settled(), State::None);
    board = board.set(3, 3, State::P2);
    assert_eq!(board.settled(), State::P2);
}

struct Eval {
    children: Vec<(usize, usize, Arc<Eval>)>,
    value: i64,
}

// trait Player {
//     fn notify_others_hand(&mut self, others_hand: (usize, usize));
//     fn choose_hand(&mut self) -> (usize, usize);
//     fn should_show_front(&self) -> bool;
// }

// struct CPU<R> {
//     rng: R,
//     eval: Arc<Eval>,
// }

// impl<R> CPU<R> {
//     fn new(rng: R, eval: Arc<Eval>) -> CPU<R> {
//         CPU { rng, eval }
//     }
// }

// impl<R: Rng> Player for CPU<R> {
//     fn notify_others_hand(&mut self, others_hand: Hand) {
//         let next_eval = self
//             .eval
//             .children
//             .iter()
//             .find(|(hand, _)| *hand == others_hand)
//             .map(|(_, eval)| Arc::clone(eval))
//             .expect("failed to find hand: your hand seems invalid.");

//         self.eval = next_eval;
//     }

//     fn choose_hand(&mut self) -> Hand {
//         let mut hand_cands = Vec::new();
//         // 勝ちの手を探す
//         for &(hand, ref eval) in &self.eval.children {
//             if !eval.win {
//                 hand_cands.push(hand);
//             }
//         }

//         if hand_cands.is_empty() {
//             // eprintln!("I'll lose!");
//             hand_cands = self
//                 .eval
//                 .children
//                 .iter()
//                 .map(|(hand, _)| hand)
//                 .copied()
//                 .collect();
//         } else {
//             // eprintln!("I'll win.");
//         }

//         let &hand = hand_cands
//             .choose(&mut self.rng)
//             .expect("internal error: failed to choose next hand");

//         self.notify_others_hand(hand);

//         hand
//     }

//     fn should_show_front(&self) -> bool {
//         false
//     }
// }

// struct Human;

// impl Player for Human {
//     fn notify_others_hand(&mut self, others_hand: Hand) {
//         println!(
//             "other moved {} to {}",
//             others_hand.line(),
//             others_hand.pos()
//         );
//     }

//     fn choose_hand(&mut self) -> Hand {
//         let (line, pos) = loop {
//             print!("next > ");
//             stdout().flush().unwrap();
//             let mut s = String::new();
//             stdin().read_line(&mut s).unwrap();
//             let s = s.trim();
//             let mut it = s.split_whitespace();
//             match (
//                 it.next().and_then(|x| x.parse().ok()),
//                 it.next().and_then(|x| x.parse().ok()),
//             ) {
//                 (Some(line), Some(pos)) => break (line, pos),
//                 _ => println!("invalid input: {}", s),
//             }
//         };

//         Hand::new(line, pos)
//     }

//     fn should_show_front(&self) -> bool {
//         true
//     }
// }

// fn do_game(mut players: [Box<dyn Player>; 2]) {
//     let mut board = Board::new();
//     let mut turn = 0;

//     while !board.settled() {
//         if players[turn].should_show_front() {
//             board.print();
//         } else {
//             board.transpose().print();
//         }

//         let hand = loop {
//             let hand = players[turn].choose_hand();
//             if board.is_available(hand.line(), hand.pos()) {
//                 break hand;
//             } else {
//                 println!("invalid hand; try again.");
//             }
//         };

//         board = board.set(0, hand.line(), hand.pos());
//         board = board.transpose();

//         turn += 1;
//         turn &= 1;
//         players[turn].notify_others_hand(hand);
//     }

//     println!("player {} win!", ((turn + 1) & 1) + 1);

//     if players[turn].should_show_front() {
//         board.print();
//     } else {
//         board.transpose().print();
//     }
// }

fn main() {
    let mut board = Board::new();
    let mut memo = HashMap::new();

    board = board.set(2, 2, State::P1);
    board = board.set(1, 1, State::P2);
    let eval = board.eval(&mut memo, 0);
    println!("Result: {}", eval.value);

    // let rng = rand::thread_rng();
    // let cpu = CPU::new(rng, b.eval(&mut memo, 0));
    // let human = Human;

    // do_game([Box::new(human) as _, Box::new(cpu) as _]);
    // do_game([Box::new(cpu) as _, Box::new(human) as _]);
}
