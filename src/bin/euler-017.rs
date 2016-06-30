// Number letter counts
//
// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
//
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen)
// contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

static NUM_LEN : &'static [i32] = &[
    0,  // zero
    3,  // one
    3,  // two
    5,  // three
    4,  // four
    4,  // five
    3,  // six
    5,  // seven
    5,  // eight
    4,  // nine
    3,  // ten
    6,  // eleven
    6,  // twelve
    8,  // thirteen
    8,  // fourteen
    7,  // fifteen
    7,  // sixteen
    9,  // seventeen
    8,  // eighteen
    8,  // nineteen
];

static TENS_LEN : &'static [i32] = &[
    0,
    0,
    6,  // twenty
    6,  // thirty
    5,  // forty
    5,  // fifty
    5,  // sixty
    7,  // seventy
    6,  // eighty
    6,  // ninety
];

static HUN_LEN : i32 = 10;  // hundred and
static THO_LEN : i32 = 8;   // thousand

fn main() {
    let mut len : i32 = 0;

    for i in 1..20 {
        len += NUM_LEN[i];
    }

    for i in 20..100 {
        len += NUM_LEN[i%10] + TENS_LEN[i/10];
    }

    // the result for 1..100 is reproduced 10 times, one for each hundred
    len *= 10;

    // now we add x hundred and ...
    for i in 1..10 {
        len += 100 * (NUM_LEN[i] + HUN_LEN);
    }

    // the 'and' is not used for 100, 200,... 900
    len -= 3*9;

    // one thousand
    len += NUM_LEN[1] + THO_LEN;

    println!("sum = {}", len);
}
