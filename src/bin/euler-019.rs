// Counting Sundays
//
// You are given the following information, but you may prefer to do some research for yourself.
//
//     1 Jan 1900 was a Monday.
//     Thirty days has September,
//     April, June and November.
//     All the rest have thirty-one,
//     Saving February alone,
//     Which has twenty-eight, rain or shine.
//     And on leap years, twenty-nine.
//     A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
//
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

use std::ops;
use std::convert;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl convert::From<i32> for Day {
    fn from(n : i32) -> Self {
        match n % 7 {
            0 => Day::Sunday,
            1 => Day::Monday,
            2 => Day::Tuesday,
            3 => Day::Wednesday,
            4 => Day::Thursday,
            5 => Day::Friday,
            6 => Day::Saturday,
            _ => unreachable!(),
        }
    }
}

impl ops::Add<i32> for Day {
    type Output = Day;
    fn add(self, rhs : i32) -> Day {
        Day::from(self as i32 + rhs)
    }
}

fn is_leap(year : i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

fn days_in_year(year : i32) -> i32 {
    365 + is_leap(year) as i32
}

fn first_of_month(year : i32) -> Vec<Day> {
    // day of 1 Jan
    let base : Day = Day::Monday + days_in_year(1901);
    let first : Day = base + (1900..year).map(days_in_year).fold(0, |a, c| a+c);

    // evaluate the first day of each month and collect in a vector of Days
    let num_days = vec![0, 31, 28 + is_leap(year) as i32, 31, 30, 31, 30, 31, 31, 30, 31, 30];
    num_days.iter().scan(first, |day, &num| { *day = *day + num; Some(*day) }).collect()
}

fn main() {
    let days = (1901..2001).map(first_of_month).fold(0, |a, v| a + v.iter().filter(|x| *x == &Day::Sunday).count());
    println!("sundays = {:?}", days);
}
