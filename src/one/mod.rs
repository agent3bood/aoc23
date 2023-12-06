use std::io::BufRead;
use std::panic;
use regex::{Regex};
use crate::utils::read_file;

pub fn part1() {
    println!("Day One, Part One");
    let mut sum = 0;
    let input_reader = read_file("src/one/input1.txt");
    for line in input_reader.lines() {
        match line {
            Ok(line) => {
                let mut l: u32 = 0;
                let mut r: u32 = 0;
                for c in line.chars() {
                    if c.is_numeric() {
                        l = c.to_digit(10).unwrap();
                        break;
                    }
                }
                for c in line.chars().rev() {
                    if c.is_numeric() {
                        r = c.to_digit(10).unwrap();
                        break;
                    }
                }
                sum += format!("{}{}", l, r).parse::<u32>().unwrap();
            }
            Err(e) => panic!("{}", e),
        }
    }
    println!("{}", sum);
}

pub fn part2() {
    println!("Day One, Part Two");
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();

    let mut sum = 0;
    let reader = read_file("src/one/input1.txt");
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut maches: Vec<u32> = Vec::new();
                let mut remaining = line;
                while let Some(m) = re.find(&remaining) {
                    let n: u32;
                    if m.len() == 1 {
                        n = m.as_str().parse().unwrap();
                    } else {
                        match m.as_str() {
                            "one" => n = 1,
                            "two" => n = 2,
                            "three" => n = 3,
                            "four" => n = 4,
                            "five" => n = 5,
                            "six" => n = 6,
                            "seven" => n = 7,
                            "eight" => n = 8,
                            "nine" => n = 9,
                            m => panic!("{}", m),
                        }
                    }
                    maches.push(n);
                    remaining = remaining[1..].to_string();
                }
                let l: u32 = maches.first().unwrap().clone();
                let r: u32 = maches.last().unwrap().clone();
                sum += format!("{}{}", l, r).parse::<u32>().unwrap();
            }
            Err(e) => panic!("{}", e),
        }
    }


    println!("{}", sum);
}
