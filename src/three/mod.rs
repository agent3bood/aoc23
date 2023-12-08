use std::collections::BTreeMap;
use std::io::{BufRead};
use crate::utils;

type Table = BTreeMap<(i32, i32), Code>;

#[derive(Debug, PartialOrd, PartialEq)]
enum Code {
    Symbol(char),
    Empty,
    Number(u32),
}


pub fn part1() {
    let mut reader = utils::read_file("src/three/input1.txt");
    println!("{}", part1_process(&mut reader));
}

fn part1_process<R: BufRead>(reader: &mut R) -> u32 {
    let mut res: u32 = 0;
    let mut table: Table = Default::default();
    build_table(reader, &mut table);
    //
    let mut num: u32 = 0;
    let mut has_adjacent_symbol = false;
    for e in table.iter() {
        let y = e.0.0;
        let x = e.0.1;
        let c = e.1;
        match c {
            Code::Symbol(_) | Code::Empty =>
                {
                    if num != 0 && has_adjacent_symbol {
                        res += num;
                    }
                    num = 0;
                    has_adjacent_symbol = false;
                }
            Code::Number(n) => {
                num *= 10;
                num += n;
                if has_adjacent_symbol == false {
                    has_adjacent_symbol = check_has_adjacent_symbol(&table, y.clone(), x.clone());
                }
            }
        }
    };
    res
}

fn part2_process<R: BufRead>(reader: &mut R) -> u32 {
    let mut res: u32 = 0;
    let mut table: Table = Default::default();
    build_table(reader, &mut table);
    //
    let mut table_stars: BTreeMap<(i32, i32), Vec<u32>> = Default::default();
    //
    let mut num: u32 = 0;
    let mut adjacent_star: Option<(i32, i32)> = None;
    for e in table.iter() {
        let y = e.0.0;
        let x = e.0.1;
        let c = e.1;
        match c {
            Code::Symbol(_) | Code::Empty =>
                {
                    if num != 0 && adjacent_star.is_some() {
                        let k = adjacent_star.unwrap();
                        let entry = table_stars.entry(k).or_default();
                        entry.push(num);
                    }
                    num = 0;
                    adjacent_star = None;
                }
            Code::Number(n) => {
                num *= 10;
                num += n;
                if adjacent_star.is_none() {
                    adjacent_star = check_adjacent_star(&table, y.clone(), x.clone());
                }
            }
        }
    };

    for (_, v) in table_stars {
        if v.len() == 2 {
            res += v.iter().cloned().reduce(|acc, e| acc * e).unwrap();
        }
        if v.len() > 2 {
            panic!("Unsupported!");
        }
    }

    res
}

fn build_table<R: BufRead>(reader: &mut R, table: &mut Table) {
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => table.insert((y as i32, x as i32), Code::Empty),
                c if c.is_digit(10) => {
                    table.insert((y as i32, x as i32), Code::Number(c.to_digit(10).unwrap()))
                }
                s => table.insert((y as i32, x as i32), Code::Symbol(s)),
            };
        }
    }
}


fn check_has_adjacent_symbol(table: &Table, y: i32, x: i32) -> bool {
    match &table.get(&(y - 1, x - 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(_) => {
                    return true;
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y - 1, x)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(_) => {
                    return true;
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y - 1, x + 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(_) => {
                    return true;
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y, x - 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(_) => {
                    return true;
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y, x + 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(_) => {
                    return true;
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y + 1, x - 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(_) => {
                    return true;
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y + 1, x)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(_) => {
                    return true;
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y + 1, x + 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(_) => {
                    return true;
                }
                _ => {}
            }
        }
    };

    false
}

fn check_adjacent_star(table: &Table, y: i32, x: i32) -> Option<(i32, i32)> {
    match &table.get(&(y - 1, x - 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(s) if s == '*' => {
                    return Some((y - 1, x - 1));
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y - 1, x)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(s) if s == '*' => {
                    return Some((y - 1, x));
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y - 1, x + 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(s) if s == '*' => {
                    return Some((y - 1, x + 1));
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y, x - 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(s) if s == '*' => {
                    return Some((y, x - 1));
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y, x + 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(s) if s == '*' => {
                    return Some((y, x + 1));
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y + 1, x - 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(s) if s == '*' => {
                    return Some((y + 1, x - 1));
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y + 1, x)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(s) if s == '*' => {
                    return Some((y + 1, x));
                }
                _ => {}
            }
        }
    };
    match &table.get(&(y + 1, x + 1)) {
        None => {}
        Some(c) => {
            match **c {
                Code::Symbol(s) if s == '*' => {
                    return Some((y + 1, x + 1));
                }
                _ => {}
            }
        }
    };

    None
}

pub fn part2() {
    let mut reader = utils::read_file("src/three/input1.txt");
    println!("{}", part2_process(&mut reader));
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::three::*;

    #[test]
    fn part1_example() {
        let input =
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let mut reader = Cursor::new(input);
        assert_eq!(part1_process(&mut reader), 4361);
    }

    #[test]
    fn part2_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let mut reader = Cursor::new(input);
        assert_eq!(part2_process(&mut reader), 467835);
    }
}
