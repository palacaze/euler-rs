// Magic 5-gon ring
//
// Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6, and each line adding
// to nine.
//
//       4
//        \
//         3
//        / \
//       1 — 2 — 6
//      /
//     5
//
// Working clockwise, and starting from the group of three with the numerically lowest external
// node (4,3,2 in this example), each solution can be described uniquely. For example, the above
// solution can be described by the set: 4,3,2; 6,2,1; 5,1,3.
//
// It is possible to complete the ring with four different totals: 9, 10, 11, and 12. There are
// eight solutions in total.  Total	Solution Set
// 9	4,2,3; 5,3,1; 6,1,2
// 9	4,3,2; 6,2,1; 5,1,3
// 10	2,3,5; 4,5,1; 6,1,3
// 10	2,5,3; 6,3,1; 4,1,5
// 11	1,4,6; 3,6,2; 5,2,4
// 11	1,6,4; 5,4,2; 3,2,6
// 12	1,5,6; 2,6,4; 3,4,5
// 12	1,6,5; 3,5,4; 2,4,6
//
// By concatenating each group it is possible to form 9-digit strings; the maximum string for a
// 3-gon ring is 432621513.
//
// Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and
// 17-digit strings. What is the maximum 16-digit string for a "magic" 5-gon ring?
//
//        o
//         \
//          o     o
//        /   \  /
//      o       o
//     / \     /
//    o   o — o — o
//         \
//          o
//
// (supposed to be a pentagon, ascii-art wont let me do that)

fn main() {
// No programming here, just some simple deduction,
//
// first of all, we have numbers from 1 to 10, only 10 has 2 digits.
// The 16 digits requirement means that 10 is picked only once, an so is
// not in the inside pentagon.
//
// Now in order to produce the biggest number, the smallest of the five numbers
// in the outer ring must be as big as possible, so if we suppose that numbers
// from 1 to 5 are in the pentagon, are final concatenated number will start with
// a 6. We can't do better.
//
// Is it possible to arrange 1 through 5 in the pentagon so as to get different
// but continuous, sum on each side ? Yes, by placing the number in order jumping
// one position:
//
//          1
//        /   \
//      3       4
//       \     /
//        5 — 2
//
// In this configuration we have partial sums 4, 5, 6, 7 and 8
// Now we only need to fill 6 through 10, easy because we see that the sum of
// each branch is 14.
//
// We have 2 possibilities though. should I extend the 3-5 branch with the 6 next
// to the 3 or the 5. The 5 obviously because we want the biggest number!
//
// The final solution is:
//
//        9
//         \
//          1     8
//        /   \  /
//      3       4
//     / \     /
//   10   5 — 2 — 7
//         \
//          6

    println!("6531031914842725")
}

