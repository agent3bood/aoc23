use std::env;
use advent23::{one, three, two};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];
    println!("Running {}::{}", day, part);

    let day_num: u8 = day.parse::<u8>().unwrap();
    let part_num: u8 = part.parse().unwrap();
    match day_num {
        1 => match part_num {
            1 => one::part1(),
            2 => one::part2(),
            _ => panic!()
        },
        2 => match part_num {
            1 => two::part1(),
            2 => two::part2(),
            _ => panic!()
        },
        3 => match part_num {
            1 => three::part1(),
            2 => three::part2(),
            _ => panic!()
        },
        _ => panic!(),
    };
}
