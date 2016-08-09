// Monopoly odds
//
// In the game, Monopoly, the standard board is set up in the following way:
// GO  A1  CC1 A2  T1  R1  B1  CH1 B2  B3  JAIL
// H2                                        C1
// T2                                        U1
// H1                                        C2
// CH3                                       C3
// R4                                        R2
// G3                                        D1
// CC3                                      CC2
// G2                                        D2
// G1                                        D3
// G2J F3  U2  F2  F1  R3  E3  E2  CH2  E1   FP
//
// A player starts on the GO square and adds the scores on two 6-sided dice to determine the number
// of squares they advance in a clockwise direction. Without any further rules we would expect to
// visit each square with equal probability: 2.5%. However, landing on G2J (Go To Jail), CC
// (community chest), and CH (chance) changes this distribution.
//
// In addition to G2J, and one card from each of CC and CH, that orders the player to go directly
// to jail, if a player rolls three consecutive doubles, they do not advance the result of their
// 3rd roll. Instead they proceed directly to jail.
//
// At the beginning of the game, the CC and CH cards are shuffled. When a player lands on CC or CH
// they take a card from the top of the respective pile and, after following the instructions, it
// is returned to the bottom of the pile. There are sixteen cards in each pile, but for the purpose
// of this problem we are only concerned with cards that order a movement; any instruction not
// concerned with movement will be ignored and the player will remain on the CC/CH square.
//
//  Community Chest (2/16 cards):
//      Advance to GO
//      Go to JAIL
//  Chance (10/16 cards):
//      Advance to GO
//      Go to JAIL
//      Go to C1
//      Go to E3
//      Go to H2
//      Go to R1
//      Go to next R (railway company)
//      Go to next R
//      Go to next U (utility company)
//      Go back 3 squares.
//
// The heart of this problem concerns the likelihood of visiting a particular square. That is, the
// probability of finishing at that square after a roll. For this reason it should be clear that,
// with the exception of G2J for which the probability of finishing on it is zero, the CH squares
// will have the lowest probabilities, as 5/8 request a movement to another square, and it is the
// final square that the player finishes at on each roll that we are interested in. We shall make
// no distinction between "Just Visiting" and being sent to JAIL, and we shall also ignore the rule
// about requiring a double to "get out of jail", assuming that they pay to get out on their next
// turn.
//
// By starting at GO and numbering the squares sequentially from 00 to 39 we can concatenate these
// two-digit numbers to produce strings that correspond with sets of squares.
//
// Statistically it can be shown that the three most popular squares, in order, are JAIL (6.24%) =
// Square 10, E3 (3.18%) = Square 24, and GO (3.09%) = Square 00. So these three most popular
// squares can be listed with the six-digit modal string: 102400.
//
// If, instead of using two 6-sided dice, two 4-sided dice are used, find the six-digit modal
// string.

#![feature(test)]
extern crate test;
extern crate time;
#[macro_use] extern crate enum_primitive;
extern crate num;
extern crate rand;
extern crate rulinalg;

use rulinalg::matrix::*;
use time::PreciseTime;
use rand::distributions::{IndependentSample, Range};
use num::FromPrimitive;

struct Random {
    range: Range<usize>,
    rng: rand::ThreadRng,
}

impl Random {
    fn new(max: usize) -> Self {
        Random { range: Range::new(1, max+1), rng: rand::thread_rng() }
    }

    fn rand(&mut self) -> usize {
        self.range.ind_sample(&mut self.rng)
    }
}

struct Dice(Random);

impl Dice {
    fn new(max: usize) -> Self {
        Dice(Random::new(max))
    }

    fn throw(&mut self) -> (usize, usize) {
        (self.0.rand(), self.0.rand())
    }
}

#[derive(Copy, Clone)]
enum Card {
    AdvanceGo,
    GoToJail,
    GoToC1,
    GoToE3,
    GoToH2,
    GoToR1,
    GoNextRailway,
    GoNextUtility,
    GoBack3,
}

struct CChest(Random);
struct Chance(Random);

impl CChest {
    fn new() -> Self {
        CChest(Random::new(16))
    }

    fn pick(&mut self) -> Option<Card> {
        match self.0.rand() {
            1 => Some(Card::AdvanceGo),
            2 => Some(Card::GoToJail),
            _ => None,
        }
    }
}

impl Chance {
    fn new() -> Self {
        Chance(Random::new(16))
    }

    fn pick(&mut self) -> Option<Card> {
        match self.0.rand() {
            1 => Some(Card::AdvanceGo),
            2 => Some(Card::GoToJail),
            3 => Some(Card::GoToC1),
            4 => Some(Card::GoToE3),
            5 => Some(Card::GoToH2),
            6 => Some(Card::GoToR1),
            7 => Some(Card::GoNextRailway),
            8 => Some(Card::GoNextRailway),
            9 => Some(Card::GoNextUtility),
            10 => Some(Card::GoBack3),
            _ => None,
        }
    }
}

enum_from_primitive! {
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
enum Square {
    GO, A1, CC1, A2, T1, R1, B1, CH1, B2, B3,
    JAIL, C1, U1, C2, C3, R2, D1, CC2, D2, D3,
    FP, E1, CH2, E2, E3, R3, F1, F2, U2, F3,
    G2J, G1, G2, CC3, G3, R4, CH3, H1, T2, H2,
}
}

mod monopoly {
    use super::Square;
    use super::num::FromPrimitive;

    fn square(x: usize) -> Square {
        Square::from_usize(x).unwrap()
    }

    fn next_railway(x: usize) -> Square {
        match square(x) {
            Square::CH1 => Square::R2,
            Square::CH2 => Square::R3,
            Square::CH3 => Square::R1,
            _ => panic!("Should be on a Chance square"),
        }
    }

    fn next_utility(x: usize) -> Square {
        match square(x) {
            Square::CH1 => Square::U1,
            Square::CH2 => Square::U2,
            Square::CH3 => Square::U1,
            _ => panic!("Should be on a Chance square"),
        }
    }

    pub fn affect_weights(p: usize, w: f64, mut mat: &mut [f64]) {
        match square(p) {
            Square::G2J => mat[Square::JAIL as usize] += w,
            Square::CC1 |
            Square::CC2 |
            Square::CC3 => {
                mat[Square::GO as usize] += w / 16.0;
                mat[Square::JAIL as usize] += w / 16.0;
                mat[p] += 14.0 * w / 16.0;
            },
            Square::CH1 |
            Square::CH2 |
            Square::CH3 => {
                mat[Square::GO as usize] += w / 16.0;
                mat[Square::JAIL as usize] += w / 16.0;
                mat[Square::C1 as usize] += w / 16.0;
                mat[Square::E3 as usize] += w / 16.0;
                mat[Square::H2 as usize] += w / 16.0;
                mat[Square::R1 as usize] += w / 16.0;
                let next_rail = next_railway(p);
                mat[next_rail as usize] += w / 8.0;
                let next_util = next_utility(p);
                mat[next_util as usize] += w / 16.0;
                let y = if p < 3 { p + 37 } else { p - 3 };
                affect_weights(y, w / 16.0, &mut mat);
                mat[p] += 6.0 * w / 16.0;
            },
            _ => mat[p] += w,
        }
    }
}

struct State {
    pos: usize,
    doubles: usize,
    dice: Dice,
    cchest: CChest,
    chance: Chance,
}

impl State {
    fn new() -> Self {
        State {
            pos: 0,
            doubles: 0,
            dice: Dice::new(4),
            cchest: CChest::new(),
            chance: Chance::new(),
        }
    }

    fn curr_square(&self) -> Square {
        Square::from_usize(self.pos).unwrap()
    }

    fn go_jail(&mut self) {
        self.doubles = 0;
        self.move_to(Square::JAIL);
    }

    fn move_to(&mut self, sq: Square) {
        self.pos = sq as usize;
    }

    fn next_railway(&self) -> Square {
        match self.curr_square() {
            Square::CH1 => Square::R2,
            Square::CH2 => Square::R3,
            Square::CH3 => Square::R1,
            _ => panic!("Should be on a Chance square"),
        }
    }

    fn next_utility(&self) -> Square {
        match self.curr_square() {
            Square::CH1 => Square::U1,
            Square::CH2 => Square::U2,
            Square::CH3 => Square::U1,
            _ => panic!("Should be on a Chance square"),
        }
    }

    fn update_pos(&mut self) {
        let sq = self.curr_square();

        match sq {
            Square::G2J => self.go_jail(),
            Square::CC1 |
            Square::CC2 |
            Square::CC3 =>
                match self.cchest.pick() {
                    Some(Card::AdvanceGo) => self.move_to(Square::GO),
                    Some(Card::GoToJail) => self.go_jail(),
                    _ => {},
                },
            Square::CH1 |
            Square::CH2 |
            Square::CH3 =>
                match self.chance.pick() {
                    Some(Card::AdvanceGo) => self.move_to(Square::GO),
                    Some(Card::GoToJail) => self.go_jail(),
                    Some(Card::GoToC1) => self.move_to(Square::C1),
                    Some(Card::GoToE3) => self.move_to(Square::E3),
                    Some(Card::GoToH2) => self.move_to(Square::H2),
                    Some(Card::GoToR1) => self.move_to(Square::R1),
                    Some(Card::GoNextRailway) => {
                        let next = self.next_railway();
                        self.move_to(next);
                    },
                    Some(Card::GoNextUtility) => {
                        let next = self.next_utility();
                        self.move_to(next);
                    },
                    Some(Card::GoBack3) => {
                        if self.pos < 3 { self.pos += 37; }
                        else { self.pos -= 3 }
                        self.update_pos();
                    },
                    _ => {},
                },
            _ => {},
        }
    }

    fn move_player(&mut self) -> usize {
        // start with dice throw
        let d = self.dice.throw();
        if d.0 == d.1 {
            self.doubles += 1;
            if self.doubles == 3 {
                self.go_jail();
                return self.pos;
            }
        }
        else {
            self.doubles = 0;
        }

        // update position
        self.pos = (self.pos + d.0 + d.1) % 40;
        self.update_pos();
        self.pos
    }
}

// monte carlo solution
pub fn solve() -> String {
    let mut state = State::new();
    let mut count = vec![0; 40];
    let nb = 10_000_000;

    for _ in 0..nb {
        let p = state.move_player();
        count[p] += 1;
    }

    let mut stat = count.iter().enumerate()
        .map(|(i, &x)| (100.0 * (x as f64) / (nb as f64), i)).collect::<Vec<_>>();
    stat.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    format!("{:02}{:02}{:02}", stat[0].1, stat[1].1, stat[2].1)
}

pub fn solve_markov() -> String {
    let dice_size = 6;
    let dice_size_f = dice_size as f64;

    // probability of having 3 doubles
    let p3d = 1.0 / (dice_size_f * dice_size_f * dice_size_f);
    let n3d = 1.0 - p3d;

    // probability for each dice
    let dice = (2..2*dice_size+1)
        .map(|i| dice_size_f - (i as f64 - dice_size_f - 1.0).abs())
        .map(|i| i * n3d / (dice_size_f * dice_size_f))
        .collect::<Vec<_>>();

    let mut mat = Matrix::<f64>::zeros(40, 40);

    // build stochastic matrix
    for s in 0..40 {
        let mut row = mat.get_row_mut(s).unwrap();
        // effect of 3 doubles
        row[Square::JAIL as usize] += p3d;

        // probability for each dice result
        for (d, dw) in dice.iter().enumerate() {
            let p = (s + d + 2) % 40;
            monopoly::affect_weights(p, *dw, row);
        }
    }

    // multiply matrix with itself a lot of times (1024) to reach invariant
    for _ in 0..10 {
        mat = &mat * &mat;
    }

    // each row is composed of distribution probabilities
    let mut stat = mat.get_row(0).unwrap().iter().enumerate()
        .map(|(i, &x)| (100.0 * x, i)).collect::<Vec<_>>();
    stat.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    format!("{:02}{:02}{:02}", stat[0].1, stat[1].1, stat[2].1)
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("best squares: {} ({})", s, start.to(PreciseTime::now()));

    let start = PreciseTime::now();
    let s = solve_markov();
    println!("best squares: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_84() {
        let s = solve();
        assert_eq!("101524", s);
    }

    #[test]
    fn test_markov_84() {
        let s = solve_markov();
        assert_eq!("101524", s);
    }

    #[bench]
    fn bench_84(b: &mut Bencher) {
        b.iter(|| black_box(solve_markov()));
    }
}

