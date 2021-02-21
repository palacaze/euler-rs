// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names,
// begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value
// by its alphabetical position in the list to obtain a name score.
//
// For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53,
// is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
//
// What is the total of all the name scores in the file?

use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn count_chars(s : &str) -> u32 {
    s.chars().map(|c| c as u32 - 64).fold(0, |a, c| a+c)
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");

    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut names = String::new();
    if let Err(why) = file.read_to_string(&mut names) {
        panic!("couldn't read {}: {}", display, why);
    }

    let mut v : Vec<&str> = names.split(',').map(|s| s.trim_matches('"')).collect();
    v.sort();
    let r = v.into_iter().map(count_chars).enumerate().fold(0, |a, (i, n)| a + (i+1)*(n as usize));

    println!("sum = {:?}", r);
}
