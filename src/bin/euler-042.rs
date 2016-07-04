// The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1);
// so the first ten triangle numbers are:
//
// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//
// By converting each letter in a word to a number corresponding to its alphabetical position
// and adding these values we form a word value. For example, the word value for SKY is
// 19 + 11 + 25 = 55 = t10. If the word value is a triangle number then we shall call the
// word a triangle word.
//
// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing
// nearly two-thousand common English words, how many are triangle words?

use std::env;
use std::io::Read;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn count_chars(s : &String) -> u32 {
    s.chars().map(|c| c as u32 - 64).fold(0, |a, c| a+c)
}

fn is_triangle(t: &u32) -> bool {
    let n = (0.5 * (((1 + 8 * (*t)) as f64).sqrt() - 1.0)) as u32;
    (n+1)*n == 2 * (*t) || (n+2)*(n+1) == 2 * (*t)
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");
    
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut names = String::new();
    match file.read_to_string(&mut names) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {},
    }

    let v : Vec<String> = names.split(',').map(|s| s.trim_matches('"').to_string()).collect();
    let r = v.iter().map(count_chars).filter(is_triangle).count();

    println!("triangle words = {:?}", r);
}
