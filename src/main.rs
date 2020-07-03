#![feature(test)]
extern crate test;

use rustc_hash::{FxHashMap, FxHasher};
use std::cmp::max;
use std::hash::BuildHasherDefault;

static mut TOTAL_EVAL: usize = 0;

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

/// 真ん中の 3 つ。フリーならば判定を早められるので。
const LINE3: &[(u64, u64)] = &[
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_01_01_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_01_00_00_00_00_00,
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_01_01_01_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_01_00_00_00_01_00_00_00_00_00_00_00_00_00_00,
    ),
    (
        0b00_00_00_00_00_00_01_01_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_01_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    ),
    (
        0b00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00,
        0b00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00,
    ),
    (
        0b00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00,
        0b00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00,
    ),
    (
        0b00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00,
        0b00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00,
    ),
    (
        0b00_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_00,
        0b01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01,
    ),
    (
        0b00_00_00_00_00_00_00_00_01_00_00_00_01_00_00_00_01_00_00_00_00_00_00_00_00,
        0b00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00,
    ),
];

// フリー 2
const LINE2: &[(u64, u64, (usize, usize), (usize, usize))] = &[
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_01_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_01,
        (0, 0),
        (0, 3),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_01_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_01_00,
        (0, 1),
        (0, 4),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_01_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_01_00_00_00_00_00,
        (1, 0),
        (1, 3),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_01_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_01_00_00_00_00_00_00,
        (1, 1),
        (1, 4),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_01_01_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_01_00_00_01_00_00_00_00_00_00_00_00_00_00,
        (2, 0),
        (2, 3),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_01_01_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_01_00_00_01_00_00_00_00_00_00_00_00_00_00_00,
        (2, 1),
        (2, 4),
    ),
    (
        0b00_00_00_00_00_00_00_01_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_01_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        (3, 0),
        (3, 3),
    ),
    (
        0b00_00_00_00_00_00_01_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_01_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        (3, 1),
        (3, 4),
    ),
    (
        0b00_00_01_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_01_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        (4, 0),
        (4, 3),
    ),
    (
        0b00_01_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b01_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        (4, 1),
        (4, 4),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01,
        (0, 0),
        (3, 0),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00,
        (1, 0),
        (4, 0),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00,
        (0, 1),
        (3, 1),
    ),
    (
        0b00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00,
        (1, 1),
        (4, 1),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00,
        (0, 2),
        (3, 2),
    ),
    (
        0b00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00,
        (1, 2),
        (4, 2),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00,
        (0, 3),
        (3, 3),
    ),
    (
        0b00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00,
        (1, 3),
        (4, 3),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00,
        (0, 4),
        (3, 4),
    ),
    (
        0b00_00_00_00_00_01_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00,
        (1, 4),
        (4, 4),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_00,
        0b00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01,
        (0, 0),
        (3, 3),
    ),
    (
        0b00_00_00_00_00_00_01_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00,
        0b01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00,
        (1, 1),
        (4, 4),
    ),
    (
        0b00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_01_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00,
        (0, 4),
        (3, 1),
    ),
    (
        0b00_00_00_00_00_00_00_00_01_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00,
        (1, 3),
        (4, 0),
    ),
];

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
        Board(self.0 | ((player as u64) << ((y * 5 + x) << 1)))
    }

    fn get(self, y: usize, x: usize) -> State {
        State::from((self.0 >> ((y * 5 + x) << 1)) & 0b11)
    }

    fn is_available(self, y: usize, x: usize) -> bool {
        self.get(y, x) == State::None
    }

    #[must_use]
    fn rotate_90(self) -> Board {
        let mut ans = Board::new();
        for y in 0..5 {
            for x in 0..5 {
                ans = ans.set(5 - x - 1, y, self.get(y, x));
            }
        }

        ans
    }

    #[must_use]
    fn mirrored(self) -> Board {
        let mut ans = Board::new();
        for y in 0..5 {
            ans.0 |= ((self.0 >> (10 * y)) & 0b_11_11_11_11_11) << (40 - 10 * y);
        }

        ans
    }

    // fn print(self) {
    //     println!("+{}+", "-".repeat(5));
    //     for y in 0..5 {
    //         print!("|");
    //         for x in 0..5 {
    //             print!(
    //                 "{}",
    //                 match self.get(y, x) {
    //                     State::None => ' ',
    //                     State::P1 => 'o',
    //                     State::P2 => 'x',
    //                     _ => panic!("internal error!"),
    //                 }
    //             );
    //         }
    //         println!("|");
    //     }
    //     println!("+{}+", "-".repeat(5));
    // }

    fn settled(self, cur_player: State) -> State {
        // 4つ揃っている
        for player in 0..2 {
            for &line in LINE {
                if ((self.0 >> player) & line).count_ones() >= 4 {
                    return State::from(player + 1);
                }
            }
        }

        // それ以外で全てのマスに石が置かれていたらもう引き分け
        if self.0.count_ones() == 25 {
            return State::Draw;
        }

        // 一つ止めフリー3
        // 自分が3つ並んでいて、片方の端だけが止められているならゴール
        for &(line, line_end) in LINE3 {
            if ((self.0 >> (cur_player as u64 - 1)) & line).count_ones() == 3
                && (self.0 & (line_end << (cur_player as u64 & 1))).count_ones() <= 1
            {
                return cur_player;
            }
        }

        // 完全フリー3
        let mut line3 = [false; 2];
        #[allow(clippy::needless_range_loop)]
        'outer: for player in 0..2 {
            for &(line, line_end) in LINE3 {
                if ((self.0 >> player) & line).count_ones() == 3
                    && (self.0 & (line_end | (line_end << 1))).count_ones() == 0
                {
                    line3[player] = true;
                    continue 'outer;
                }
            }
        }

        if line3[0] && line3[1] {
            return cur_player;
        } else if line3[0] {
            return State::P1;
        } else if line3[1] {
            return State::P2;
        }

        // 続行したとして望みがあるかどうかを確認。
        // 4 のうち一つでも完成しうるかどうかを確認する。
        for &line in LINE {
            if (self.0 & line).count_ones() == 0 || ((self.0 >> 1) & line).count_ones() == 0 {
                return State::None;
            }
        }

        // いずれの 4 も両方の石が侵入しているならもはや選びようがないのでドロー。
        State::Draw
    }

    // WARNING: この実装は全探索にならない！！たとえばもともと 4 つしかない列に free-2 があっても、
    // 今すぐに対応しなければいけないわけではない。
    fn has_free2(self, cur_player: State) -> Option<((usize, usize), (usize, usize))> {
        let mut line2 = [None; 2];
        #[allow(clippy::needless_range_loop)]
        'outer: for player in 0..2 {
            for &(line, line_end, start, end) in LINE2 {
                if ((self.0 >> player) & line).count_ones() == 2
                    && (self.0 & (line_end | (line_end << 1))).count_ones() == 0
                {
                    line2[player] = Some((start, end));
                    continue 'outer;
                }
            }
        }

        if line2[0].is_some() && line2[1].is_some() {
            // 両方とも free-2 を持っている場合、自分の free-2 を伸ばせば勝ち
            line2[cur_player as usize - 1]
        } else if line2[0].is_some() {
            // player 1 だけ free-2 なら敵も味方も free-2 を伸ばしに or 阻止しに行く
            line2[0]
        } else if line2[1].is_some() {
            line2[1]
        } else {
            None
        }
    }

    fn eval(mut self, memo: &mut FxHashMap<Board, Eval>, depth: usize) -> Eval {
        if depth > 11 {
            return Eval::new(0);
        }

        if depth <= 2 {
            println!("evaluating at depth {} of board {:050b}", depth, self.0);
        }

        unsafe {
            TOTAL_EVAL += 1;
        }

        // 先に判定（メモリ節約なるか？）
        let current = State::from(((depth & 1) + 1) as u64);
        let settled = self.settled(current);
        if settled != State::None {
            let value = if settled == current {
                1
            } else if settled == State::Draw {
                0
            } else {
                -1
            };

            return Eval::new(value);
        }

        // 再帰する前にすでに現れたかどうかチェック。単に鏡像・回転したものは同じとみなす。
        for _ in 0..2 {
            for _ in 0..4 {
                // この局面が保存してあるならそれを取得
                if let Some(&eval) = memo.get(&self) {
                    return eval;
                }
                self = self.rotate_90();
            }
            self = self.mirrored();
        }

        let mut value = -1;
        // 終了するかどうかを返す。
        let mut evaluator = |y: usize, x: usize| {
            if !self.is_available(y, x) {
                return false;
            }

            let next = self.set(y, x, current);
            let eval = next.eval(memo, depth + 1);

            // 自分必勝かどうかは次相手が必敗かどうかなので反転する。
            value = max(value, -eval.value);

            // 終了するかどうか
            value == 1
        };

        if let Some((start, end)) = self.has_free2(current) {
            if !evaluator(start.0, start.1) {
                evaluator(end.0, end.1);
            }
        } else {
            'outer: for y in 0..5 {
                for x in 0..5 {
                    if evaluator(y, x) {
                        break 'outer;
                    }
                }
            }
        }

        let eval = Eval::new(value);
        memo.insert(self, eval);

        eval
    }
}

#[test]
fn test_board() {
    let mut board = Board::new();
    board = board.set(0, 0, State::P1);
    board = board.set(1, 0, State::P1);
    board = board.set(2, 0, State::P1);
    assert_eq!(board.settled(State::P1), State::None);
    board = board.set(3, 0, State::P1);
    assert_eq!(board.settled(State::P1), State::P1);

    let mut board = Board::new();
    board = board.set(1, 0, State::P1);
    board = board.set(2, 0, State::P1);
    board = board.set(3, 0, State::P1);
    assert_eq!(board.settled(State::P1), State::None);
    board = board.set(4, 0, State::P1);
    assert_eq!(board.settled(State::P1), State::P1);

    let mut board = Board::new();
    board = board.set(0, 0, State::P1);
    board = board.set(0, 1, State::P1);
    board = board.set(0, 2, State::P1);
    assert_eq!(board.settled(State::P1), State::None);
    board = board.set(0, 3, State::P1);
    assert_eq!(board.settled(State::P1), State::P1);

    let mut board = Board::new();
    board = board.set(0, 1, State::P1);
    board = board.set(0, 2, State::P1);
    board = board.set(0, 3, State::P1);
    assert_eq!(board.settled(State::P1), State::None);
    board = board.set(0, 4, State::P1);
    assert_eq!(board.settled(State::P1), State::P1);

    let mut board = Board::new();
    board = board.set(0, 0, State::P1);
    board = board.set(1, 1, State::P1);
    board = board.set(2, 2, State::P1);
    assert_eq!(board.settled(State::P1), State::None);
    board = board.set(3, 3, State::P1);
    assert_eq!(board.settled(State::P1), State::P1);

    let mut board = Board::new();
    board = board.set(0, 0, State::P2);
    board = board.set(1, 1, State::P2);
    board = board.set(2, 2, State::P2);
    assert_eq!(board.settled(State::P1), State::None);
    board = board.set(3, 3, State::P2);
    assert_eq!(board.settled(State::P1), State::P2);

    let mut board = Board::new();
    board = board.set(1, 1, State::P1);
    board = board.set(2, 1, State::P1);
    board = board.set(3, 1, State::P1);
    assert_eq!(board.settled(State::P1), State::P1);
    assert_eq!(board.settled(State::P2), State::P1);
    board = board.set(1, 2, State::P2);
    board = board.set(2, 2, State::P2);
    board = board.set(3, 2, State::P2);
    assert_eq!(board.settled(State::P1), State::P1);
    assert_eq!(board.settled(State::P2), State::P2);
}

#[bench]
fn bench_board_set(b: &mut test::Bencher) {
    b.iter(|| {
        for _ in 0..1_000_000 {
            let mut board = Board::new();
            for i in 0..5 {
                for j in 0..5 {
                    board = test::black_box(board.set(i, j, State::from(((i + j) & 1) as u64)));
                }
            }
        }
    });
}

#[bench]
fn bench_board_get(b: &mut test::Bencher) {
    b.iter(|| {
        for _ in 0..1_000_000 {
            let board = Board::new();
            for i in 0..5 {
                for j in 0..5 {
                    test::black_box(board.get(i, j));
                }
            }
        }
    });
}

#[bench]
fn bench_board_rotate(b: &mut test::Bencher) {
    b.iter(|| {
        let mut board = Board::new();
        for i in 0..5 {
            for j in 0..5 {
                board = board.set(i, j, State::from(((i + j) & 1) as u64));
            }
        }

        for _ in 0..1_000_000 {
            board = test::black_box(board.rotate_90());
        }
    })
}

#[bench]
fn bench_board_mirror(b: &mut test::Bencher) {
    b.iter(|| {
        let mut board = Board::new();
        for i in 0..5 {
            for j in 0..5 {
                board = board.set(i, j, State::from(((i + j) & 1) as u64));
            }
        }

        for _ in 0..1_000_000 {
            board = test::black_box(board.mirrored());
        }
    })
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Eval {
    value: i8,
}

impl Eval {
    fn new(value: i8) -> Eval {
        Eval { value }
    }
}

fn main() {
    let mut board = Board::new();
    let mut memo = FxHashMap::with_capacity_and_hasher(
        1_000_000_000,
        BuildHasherDefault::<FxHasher>::default(),
    );

    board = board.set(2, 2, State::P1);
    board = board.set(1, 1, State::P2);
    let eval = board.eval(&mut memo, 0);
    println!("Result: {} ({})", eval.value, unsafe { TOTAL_EVAL });

    // let rng = rand::thread_rng();
    // let cpu = CPU::new(rng, b.eval(&mut memo, 0));
    // let human = Human;

    // do_game([Box::new(human) as _, Box::new(cpu) as _]);
    // do_game([Box::new(cpu) as _, Box::new(human) as _]);
}
