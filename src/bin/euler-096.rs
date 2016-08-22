// Su Doku
//
// Su Doku (Japanese meaning number place) is the name given to a popular puzzle concept. Its
// origin is unclear, but credit must be attributed to Leonhard Euler who invented a similar, and
// much more difficult, puzzle idea called Latin Squares. The objective of Su Doku puzzles,
// however, is to replace the blanks (or zeros) in a 9 by 9 grid in such that each row, column, and
// 3 by 3 box contains each of the digits 1 to 9. Below is an example of a typical starting puzzle
// grid and its solution grid.
//
// 0 0 3  0 2 0  6 0 0     4 8 3  9 2 1  6 5 7
// 9 0 0  3 0 5  0 0 1     9 6 7  3 4 5  8 2 1
// 0 0 1  8 0 6  4 0 0     2 5 1  8 7 6  4 9 3
//
// 0 0 8  1 0 2  9 0 0     5 4 8  1 3 2  9 7 6
// 7 0 0  0 0 0  0 0 8     7 2 9  5 6 4  1 3 8
// 0 0 6  7 0 8  2 0 0     1 3 6  7 9 8  2 4 5
//
// 0 0 2  6 0 9  5 0 0     3 7 2  6 8 9  5 1 4
// 8 0 0  2 0 3  0 0 9     8 1 4  2 5 3  7 6 9
// 0 0 5  0 1 0  3 0 0     6 9 5  4 1 7  3 8 2
//
// A well constructed Su Doku puzzle has a unique solution and can be solved by logic, although it
// may be necessary to employ "guess and test" methods in order to eliminate options (there is much
// contested opinion over this). The complexity of the search determines the difficulty of the
// puzzle; the example above is considered easy because it can be solved by straight forward direct
// deduction.
//
// The 6K text file, sudoku.txt (right click and 'Save Link/Target As...'), contains fifty
// different Su Doku puzzles ranging in difficulty, but all with unique solutions (the first puzzle
// in the file is the example above).
//
// By solving all fifty puzzles find the sum of the 3-digit numbers found in the top left corner of
// each solution grid; for example, 483 is the 3-digit number found in the top left corner of the
// solution grid above.

#![feature(conservative_impl_trait)]
#![feature(try_from)]
#![feature(test)]
extern crate test;
extern crate time;
extern crate itertools;

use time::PreciseTime;
use itertools::Itertools;

use std::convert::TryFrom;
use std::ops::{Index, IndexMut, Add};
use std::fmt;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// A simple bitset used to keep track of digits from 1 to 9
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Digits(u16);

impl Digits {
    fn set(&mut self, d: usize) {
        assert!(d < 10);
        self.0 |= 1 << d;
    }

    fn unset(&mut self, d: usize) {
        assert!(d < 10);
        self.0 &= !(1 << d)
    }

    fn is_set(&self, d: usize) -> bool {
        assert!(d < 10);
        self.0 & (1 << d) == 1 << d
    }

    fn reverse(&self) -> Self {
        let d = (!self.0) & ((1 << 10) -1);
        Digits(d)
    }

    fn count(&self) -> usize {
        self.0.count_ones() as usize - self.is_set(0) as usize
    }

    fn set_digits(self) -> impl Iterator<Item=usize> {
        (1..10).filter(move |&i| self.is_set(i))
    }
}

impl Add<Digits> for Digits {
    type Output = Digits;
    fn add(self, rhs: Digits) -> Self::Output {
        Digits(self.0 | rhs.0)
    }
}

/// A 9 x 9 grid with 2d access
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
struct Grid<T>(Vec<T>);

impl<T> Grid<T> where T: Clone {
    fn new(val: T) -> Self {
        Grid(vec![val; 9 * 9])
    }

    fn new_from_iter<I: Iterator<Item=T>>(it: I) -> Self {
        let v = it.take(9 * 9).collect::<Vec<_>>();
        assert_eq!(v.len(), 9 * 9);
        Grid(v)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> where T: Clone {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[row*9+col]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> where T: Clone {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row*9+col]
    }
}

/// A cell represents data in a sudoku grid at any given moment
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Cell {
    Value(u8),           // a true and final value
    Guess(u8),           // a temporary guess
    Deduc(u8, usize),    // a value deducted after guess number i
    Empty,               // empty Cell
}

impl TryFrom<Cell> for usize {
    type Err = ();
    fn try_from(cell: Cell) -> Result<Self, Self::Err> {
        match cell {
            Cell::Value(x) | Cell::Guess(x) | Cell::Deduc(x,_) => Ok(x as usize),
            Cell::Empty => Err(()),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Sudoku {
    grid: Grid<Cell>,                       // sudo grid
    state: Grid<Digits>,                    // grid tracking allowed digits for every empty cell
    guesses: Vec<((usize, usize), Digits)>, // position of guesses and invalidated digits
}

impl Sudoku {
    fn new(data: &[u8]) -> Self {
        let it = data.iter()
            .map(|&i| if i == 0 { Cell::Empty } else { Cell::Value(i) });

        let mut sudo = Sudoku {
            grid: Grid::new_from_iter(it),
            state: Grid::new(Digits::default()),
            guesses: Vec::new()
        };

        sudo.update_state();
        sudo
    }

    // iterator over elements in square at given position
    fn square_at<'a>(&'a self, (r, c): (usize, usize)) -> impl Iterator<Item=Cell> {
        let r = (r / 3) * 3;
        let c = (c / 3) * 3;
        (0..9).map(move |i| self.grid[(r+i/3, c+i%3)])
    }

    // iterator over elements at given col
    fn col<'a>(&'a self, n: usize) -> impl Iterator<Item=Cell> {
        (0..9).map(move |i| self.grid[(i, n)])
    }

    // iterator over elements at given row
    fn row<'a>(&'a self, n: usize) -> impl Iterator<Item=Cell> {
        (0..9).map(move |i| self.grid[(n, i)])
    }

    fn count_digits<I: Iterator<Item=Cell>>(&self, it: I) -> Digits {
    it.filter_map(|c|
           if let Ok(x) = usize::try_from(c) { Some(x) } else { None })
      .fold(Digits::default(), |mut a, i| {a.set(i); a})
    }

    // evaluate allowed digits for each empty Cell
    // return false in case of invalid state
    fn update_state(&mut self) -> bool {
        let dc = (0..9).map(|i| self.count_digits(self.col(i))).collect::<Vec<_>>();
        let dr = (0..9).map(|i| self.count_digits(self.row(i))).collect::<Vec<_>>();
        let ds = (0..9).map(|i| self.count_digits(self.square_at((i, 3*(i%3))))).collect::<Vec<_>>();

        for r in 0..9 {
            for c in 0..9 {
                let p = (r, c);
                if self.grid[p] != Cell::Empty { continue; }
                let d = dc[c] + dr[r] + ds[3*(r/3) + c/3];
                let d = d.reverse();
                if d.count() == 0 { return false; }
                self.state[p] = d;
            }
        }
        true
    }

    // deduce or guess a new digit
    fn guess_digit(&mut self) -> bool {
        if let Some(p) = self.find_easiest() {
            let d = self.state[p].set_digits().nth(0).unwrap() as u8;
            // if count == 1, no need for guessing
            self.grid[p] = if self.state[p].count() == 1 {
                Cell::Deduc(d, self.guesses.len())
            }
            else {
                self.guesses.push((p, Digits::default()));
                Cell::Guess(d)
            };
            true
        }
        else {
            false
        }
    }

    // A conflict happened, it is either a guessing error or an unsolvable sudoku
    // This cancel the last guess and subsequent deductions and mark the guess as
    // invalid. If no there is no more available digits for a guess, it means a
    // previous guess was also wrong and we must backtrack by invalidating a
    // previous guess
    fn invalidate_last_guess(&mut self) -> bool {
        if let Some((p, mut invalid_d)) = self.guesses.pop() {
            if let Cell::Guess(val) = self.grid[p] {
                // remove guess and subsequent deductions
                self.grid[p] = Cell::Empty;
                let len = self.guesses.len()+1;
                for e in self.grid.0.iter_mut() {
                    if let Cell::Deduc(_, x) = *e {
                        if x == len {
                            *e = Cell::Empty;
                        }
                    }
                }

                // we rewound to previious state so this should work
                self.update_state();

                // list of allowed digits in cell p
                invalid_d.set(val as usize);
                let mut d = self.state[p];
                for i in invalid_d.set_digits() {
                    d.unset(i);
                }

                // no more possible digit, we thus made an error in a previous guess
                if d.count() == 0 {
                    return self.invalidate_last_guess();
                }
                else {
                    self.guesses.push((p, invalid_d));
                    let val = d.set_digits().nth(0).unwrap() as u8;
                    self.grid[p] = Cell::Guess(val);
                    return true;
                }
            }
            else {
                panic!("Should not pass here");
            }
        }

        // an error even though we did not guess any number, the
        // sudoku appears to be unsolvable
        false
    }

    // find the cell with the least number of possible digits
    fn find_easiest(&self) -> Option<(usize, usize)> {
        let mut min = 10;
        let mut min_coord = (0, 0);
        for r in 0..9 {
            for c in 0..9 {
                let p = (r, c);
                if self.grid[p] != Cell::Empty { continue; }

                let count = self.state[p].count();

                // only one allowed digit, early exit
                if count == 1 { return Some(p); }

                if count > 0 && count < min {
                    min = count;
                    min_coord = p;
                }
            }
        }
        if min > 9 { None } else { Some(min_coord) }
    }

    fn solve(&mut self) -> bool {
        // until we can complete, we make deduction and guesses and track
        // erroneous guesses in order to progress
        while self.guess_digit() {
            while !self.update_state() {
                if !self.invalidate_last_guess() { return false; }
            }
        }

        true
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.grid.0.iter()
            .map(|&c|
                if let Ok(x) = usize::try_from(c) {
                    std::char::from_digit(x as u32, 10).unwrap()
                }
                else { ' ' }
            ).collect::<Vec<_>>();

        let line = "———————————————————\n";
        let mut g = line.to_string();
        for (i, c) in s.chunks(9).enumerate() {
            let r =  ['⎜',' ',' ','⎮',' ',' ','⎮',' ',' ','⎟'].iter()
                .interleave(c).cloned().collect::<String>();
            g += &(r + &"\n");
            if (i + 1) % 3 == 0 {
                g += &line.to_string();
            }
        }

        write!(f, "{}", g)
    }
}

fn read_data(path: &str) -> Vec<Sudoku> {
    let path = Path::new(&path);
    let mut sudo = Vec::new();
    let mut buf = Vec::new();

    for l in BufReader::new(File::open(&path).unwrap()).lines() {
        let l = l.unwrap();
        if l.starts_with("Grid") {
            if !buf.is_empty() {
                sudo.push(Sudoku::new(&buf));
                buf.clear();
            }
        }
        else {
            buf.extend(l.chars().map(|c| c.to_digit(10).unwrap() as u8));
        }
    }
    sudo.push(Sudoku::new(&buf));
    sudo
}

pub fn solve(path: &str) -> usize {
    let mut sudo = read_data(path);
    let mut sum = 0;

    for s in sudo.iter_mut() {
        s.solve();
        sum += 100 * usize::try_from(s.grid[(0,0)]).unwrap();
        sum += 10 * usize::try_from(s.grid[(0,1)]).unwrap();
        sum += usize::try_from(s.grid[(0,2)]).unwrap();
        // println!("{}", s);
    }

    sum
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");
    let start = PreciseTime::now();
    let s = solve(&path);
    println!("sum: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_96() {
        let path = "data/p096_sudoku.txt";
        let s = solve(&path);
        assert_eq!(24702, s);
    }

    #[bench]
    fn bench_96(b: &mut Bencher) {
        let path = "data/p096_sudoku.txt";
        b.iter(|| black_box(solve(&path)));
    }
}

