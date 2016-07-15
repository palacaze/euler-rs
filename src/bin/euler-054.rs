// Poker hands
//
// In the card game poker, a hand consists of five cards and are ranked, from lowest
// to highest, in the following way:
//
//     High Card: Highest value card.
//     One Pair: Two cards of the same value.
//     Two Pairs: Two different pairs.
//     Three of a Kind: Three cards of the same value.
//     Straight: All cards are consecutive values.
//     Flush: All cards of the same suit.
//     Full House: Three of a kind and a pair.
//     Four of a Kind: Four cards of the same value.
//     Straight Flush: All cards are consecutive values of same suit.
//     Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
//
// The cards are valued in the order:
// 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
//
// If two players have the same ranked hands then the rank made up of the highest
// value wins; for example, a pair of eights beats a pair of fives (see example 1 below).
// But if two ranks tie, for example, both players have a pair of queens, then highest
// cards in each hand are compared (see example 4 below); if the highest cards tie then
// the next highest cards are compared, and so on.
//
// Consider the following five hands dealt to two players:
// Hand     Player 1            Player 2              Winner
// 1        5H 5C 6S 7S KD      2C 3S 8S 8D TD       Player 2
//          Pair of Fives       Pair of Eights
// 2        5D 8C 9S JS AC      2C 5C 7D 8S QH       Player 1
//          Highest card Ace    Highest card Queen
// 3        2D 9C AS AH AC      3D 6D 7D TD QD       Player 2
//          Three Aces          Flush with Diamonds
// 4        4D 6S 9H QH QC      3D 6D 7H QD QS       Player 1
//          Pair of Queens      Pair of Queens
//          Highest card Nine   Highest card Seven
// 5        2H 2D 4C 4D 4S      3C 3D 3S 9S 9D       Player 1
//          Full House          Full House
//          With Three Fours    with Three Threes
//
// The file, poker.txt, contains one-thousand random hands dealt to two players.
// Each line of the file contains ten cards (separated by a single space):
// the first five are Player 1's cards and the last five are Player 2's cards.
// You can assume that all hands are valid (no invalid characters or repeated cards),
// each player's hand is in no specific order, and in each hand there is a clear winner.
//
// How many hands does Player 1 win?

use std::env;
use std::io::Read;
use std::error::Error;
use std::fs::File;
use std::path::Path;

// card suit
#[derive(Debug,Copy,Clone,PartialEq)]
enum CardSuit { H, C, S, D }

const ACE: u32 = 14;

#[derive(Debug,Copy,Clone,PartialEq)]
struct Card {
    val: u32,
    suit: CardSuit,
}

type Hand = [Card; 5];

// bit offset for the hand grading system
#[derive(Debug,Copy,Clone,PartialEq)]
enum HandRank {
    // the 20th first bits are use for the 5 cards, 4 bits each
    Pair1    = 20,
    Pair2    = 21,
    Kind3    = 22,
    Flush    = 23,
    Straight = 24,
    House    = 25,
    Kind4    = 26,
    SFlush   = 27,
    RFlush   = 28,
}

type HandGrade = u32;

// add card value v in slot n
fn add_val(g: &mut HandGrade, s: usize, v: u32) {
    *g += v << ((s-1)*4);
}

fn add_rank(g: &mut HandGrade, h: HandRank) {
    *g += 1u32 << (h as u32);
}

// assign a grade that represents uniquely the value of a hand
fn grade_hand(h: &[Card]) -> HandGrade {
    let mut grade: HandGrade = 0;

    let mut c: [Card; 5] = [h[0], h[1], h[2], h[3], h[4]];
    c.sort_by(|a, b| (a.val as u32).cmp(&(b.val as u32)));

    // consecutive
    let consecutive = c[1].val == c[0].val + 1 && c[2].val == c[1].val + 1 &&
                      c[3].val == c[2].val + 1 && c[4].val == c[3].val + 1;
    if consecutive {
        add_rank(&mut grade, HandRank::Straight);
    }

    // flush
    let flush = h.iter().all(|&v| v.suit == h[0].suit);
    if flush {
        add_rank(&mut grade, HandRank::Flush);

        if consecutive {
            // royal flush
            if c[4].val == ACE {
                add_rank(&mut grade, HandRank::RFlush);
                return grade;
            }
            // straigh flush
            else {
                add_rank(&mut grade, HandRank::SFlush);
                add_val(&mut grade, 5, c[0].val);
                return grade;
            }
        }
    }

    // count card values
    let mut vals = [0; ACE as usize + 1];
    for i in &c {
        vals[i.val as usize] += 1;
    }

    let mut trios = 0;
    let mut pairs = 0;
    let mut solos = 0;

    // pairs, 3 and 4 of a kind
    for (i, n) in vals.iter().enumerate() {
        // 4 of a kind
        if n == &4 {
            add_rank(&mut grade, HandRank::Kind4);
            add_val(&mut grade, 5, i as u32);
        }
        // 3 of a kind
        else if n == &3 {
            add_rank(&mut grade, HandRank::Kind3);
            add_val(&mut grade, 5, i as u32);
            trios += 1;
        }
        // pair
        else if n == &2 {
            add_rank(&mut grade, if pairs == 0 { HandRank::Pair1 } else { HandRank::Pair2 });
            add_val(&mut grade, 4 + pairs, i as u32);
            pairs += 1;
        }
        // solo
        else if n == &1 {
            solos += 1;
            add_val(&mut grade, solos, i as u32);
        }
    }

    // let's fot forget the full house
    if pairs > 0 && trios > 0 {
        add_rank(&mut grade, HandRank::House);
    }

    grade
}

fn parse_card(s: &str) -> Card {
    let chars = s.chars().collect::<Vec<_>>();
    assert_eq!(chars.len(), 2);
    let v = match chars[0] {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => ACE,
        _   => panic!("wrong card value"),
    };

    let k = match chars[1] {
        'H' => CardSuit::H,
        'C' => CardSuit::C,
        'S' => CardSuit::S,
        'D' => CardSuit::D,
        _   => panic!("wrong card kind"),
    };

    Card { val: v, suit: k }
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");

    let path = Path::new(&path);
    let display = path.display();

    // open
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // read
    let mut data = String::new();
    if let Err(why) = file.read_to_string(&mut data) {
        panic!("couldn't read {}: {}", display, why.description());
    };

    // parsing, grade assignment, and counting
    let count = data.lines().map(|s| {
                    let cards = s.split(' ').map(|c| parse_card(c)).collect::<Vec<_>>();
                    (grade_hand(&cards[0..5]), grade_hand(&cards[5..10]))
                })
                .filter(|&(g1, g2)| g1 > g2)
                .count();

    println!("Player 1 wins {} hand", count);
}

