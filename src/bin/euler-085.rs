// Counting rectangles
//
// By counting carefully it can be seen that a rectangular grid measuring 3 by 2 contains eighteen
// rectangles:
//
//   ——— --------     ——————— ----      ———————————
//  |   |   :   :    |       |   :     |           |
//   ——— --------     ——————— ----      ———————————
//  :   :   :   :    :   :   :   :     :   :   :   :
//  -------------    -------------     -------------
//        6                4                 2
//
//   ——— --------     ——————— ----      ———————————
//  |   |   :   :    |       |   :     |           |
//  |   |--------    |       |----     |           |
//  |   |   :   :    |       |   :     |           |
//   ——— --------     ——————— ----      ———————————
//        3                2                 1
//
// Although there exists no rectangular grid that contains exactly two million rectangles, find the
// area of the grid with the nearest solution.

// We can trivially prove that the number of rectangles is x.y.(x+1).(y+1) / 4
// tihs means that we only have to test for x in 1..2000

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

pub fn solve() -> i64 {
    let n = 2_000_000;
    let xm = (2.0*n as f64).sqrt() as i64 + 1;

    let r = (1..xm)
        .map(|x| {
            let d = 1.0 + (16.0 * n as f64) / (x as f64 * (x as f64 + 1.0));
            let y = ((d.sqrt() - 1.0) / 2.0).round() as i64;
            ((x*y*(x+1)*(y+1)/4 - n).abs(), x, y)
        })
        .min_by_key(|&a| a.0).unwrap();

    r.1 * r.2
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("best area {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_85() {
        let s = solve();
        assert_eq!(2772, s);
    }

    #[bench]
    fn bench_85(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

