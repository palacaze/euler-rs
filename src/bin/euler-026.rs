// Reciprocal cycles
//
// A unit fraction contains 1 in the numerator.
// The decimal representation of the unit fractions with denominators 2 to 10 are given:
//
//     1/2	= 	0.5
//     1/3	= 	0.(3)
//     1/4	= 	0.25
//     1/5	= 	0.2
//     1/6	= 	0.1(6)
//     1/7	= 	0.(142857)
//     1/8	= 	0.125
//     1/9	= 	0.(1)
//     1/10	= 	0.1
//
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle.
// It can be seen that 1/7 has a 6-digit recurring cycle.
//
// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

// A recurring cycle will happen when by calculating a new decimal digit, an already encountered
// remainder is obtained.
// So, we will calculate each digit in turn and store remainders at each step and watch
// out for duplicates, within a limit in order to manage irrational numbers

fn cycle_length(n : usize) -> usize {
    let mut r = 1;
    let mut vr = vec![r];

    for _ in 1..2000 {
        r *= 10;
        if r >= n {
            // classical division, don't even bother to keep the digit
            let q = r / n;
            r = r - n * q;

            // exact division, so no cycle
            if r == 0 {
                return 0;
            }

            // else lookout for already encountered remainder
            if let Some(i) = vr.iter().position(|&x| x == r) {
                return vr.len() - i;
            }
        }

        // store remainder
        vr.push(r);
    }

    // if we get here this means the number appears irrational
    0
}

fn main() {
    let nb = 1000;
    let res = (2..nb).map(cycle_length).enumerate().map(|(i,x)| (x,i)).max().unwrap();

    // we add 2 because we started counting from 2 but enumerate() starts at 0
    println!("best at {}, cycle length {}", res.1 + 2, res.0);
}
