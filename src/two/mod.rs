use std::io::{BufRead};
use regex::{Regex};
use crate::utils;

pub fn part1() {
    let mut reader = utils::read_file("src/two/input1.txt");
    println!("{}", part1_process(&mut reader));
}

pub fn part1_process<R: BufRead>(reader: &mut R) -> u32 {
    let mut res = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if is_valid(&line) {
            res += game_id(&line);
        }
    }
    res
}


pub fn part2() {
    let mut reader = utils::read_file("src/two/input1.txt");
    println!("{}", part2_process(&mut reader));
}

pub fn part2_process<R: BufRead>(reader: &mut R) -> u32 {
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let cube: Cubes = get_cubes(&line);
        sum += cube.red * cube.green * cube.blue;
    }
    sum
}


/// ```
/// use advent23::two::*;
/// assert_eq!(is_valid(&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), true, "Game 1");
/// assert_eq!(is_valid(&"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), true, "Game 2");
/// assert_eq!(is_valid(&"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), false, "Game 3");
/// assert_eq!(is_valid(&"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), false, "Game 4");
/// assert_eq!(is_valid(&"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), true, "Game 5");
/// ```
pub fn is_valid(line: &str) -> bool {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let cubes = get_cubes(&line);
    cubes.red < 13 && cubes.green < 14 && cubes.blue < 15
}

/// ```
/// use advent23::two;
/// let id = two::game_id(&"Game 59: 7 red, 11 blue, 17 green; 5 red, 4 green, 7 blue; 8 red, 6 blue, 17 green; 16 green, 7 red, 6 blue; 5 blue, 12 green, 9 red; 7 blue, 3 red, 9 green");
/// assert_eq!(id, 59);
/// ```
pub fn game_id(line: &str) -> u32 {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let re = Regex::new(r"^Game (\d*):").unwrap();
    re.captures(line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap()
}


/// ```
/// use advent23::two::*;
/// let line = "Game 59: 7 red, 11 blue, 17 green; 5 red, 4 green, 7 blue; 8 red, 6 blue, 17 green; 16 green, 7 red, 6 blue; 5 blue, 12 green, 9 red; 7 blue, 3 red, 9 green";
/// let cubes:Cubes = Cubes {
///  blue: 11,
///  green: 17,
///  red: 9,
///  };
/// assert_eq!(get_cubes(&line), cubes);
/// ```
pub fn get_cubes(line: &str) -> Cubes {
    let mut cubes: Cubes = Cubes {
        blue: 0,
        green: 0,
        red: 0,
    };
    let re = Regex::new(r"(\d*) red").unwrap();
    for m in re.captures_iter(&line) {
        let val = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
        if val > cubes.red {
            cubes.red = val;
        }
    }
    let re = Regex::new(r"(\d*) blue").unwrap();
    for m in re.captures_iter(&line) {
        let val = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
        if val > cubes.blue {
            cubes.blue = val;
        }
    }
    let re = Regex::new(r"(\d*) green").unwrap();
    for m in re.captures_iter(&line) {
        let val = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
        if val > cubes.green {
            cubes.green = val;
        }
    }


    cubes
}

#[derive(Debug)]
pub struct Cubes {
    pub blue: u32,
    pub green: u32,
    pub red: u32,
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let mut reader = Cursor::new(input);
        assert_eq!(part1_process(&mut reader), 8);
    }

    #[test]
    fn part2_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let mut reader = Cursor::new(input);
        assert_eq!(part2_process(&mut reader), 2286);
    }
}
