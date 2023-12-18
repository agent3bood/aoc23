use std::collections::BTreeMap;

enum Instruction {
    L,
    R,
}

struct Sol {
    instructions: Vec<Instruction>,
    map: BTreeMap<String, (String, String)>,
}

impl Sol {
    fn new() -> Sol {
        Sol {
            instructions: vec![],
            map: Default::default(),
        }
    }
}

fn build_sol(input: &str, sol: &mut Sol) {
    let mut lines = input.lines();
    let ins = lines.next();
    for i in ins.unwrap().chars() {
        match i {
            'R' => {
                sol.instructions.push(Instruction::R);
            }
            'L' => {
                sol.instructions.push(Instruction::L);
            }
            _ => {
                panic!("Unknown instruction");
            }
        };
    }
    lines.next();
    for line in lines {
        let mut parts = line.split("=");
        let area = parts.next().unwrap().trim();
        let mut steps = parts.next().unwrap().split(",");
        let left = steps.next().unwrap().replace("(", "");
        let left = left.trim();
        let right = steps.next().unwrap().replace(")", "");
        let right = right.trim();
        sol.map.insert(area.into(), (left.into(), right.into()));
    }
}

fn part1_process(sol: &Sol) -> u64 {
    let mut step = 0;
    let mut ans = 0;
    let mut pos = "AAA";
    while pos != "ZZZ" {
        if step == sol.instructions.len() {
            step = 0;
        }
        let map = sol.map.get(pos).unwrap();
        let instruction = sol.instructions.get(step).unwrap();
        match instruction {
            Instruction::R => {
                pos = &*(*map).1;
            }
            Instruction::L => {
                pos = &*(*map).0;
            }
        }

        step += 1;
        ans += 1;
    }

    ans
}

fn part2_process(sol: &Sol) -> u64 {
    let mut poss: Vec<String> = sol
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect();
    let mut poss_end: Vec<u64> = vec![0; poss.len()];
    let mut instructions = sol.instructions.iter().cycle();

    for (i, pos) in poss.iter_mut().enumerate() {
        let pos_end = poss_end.get_mut(i).unwrap();
        while !pos.ends_with('Z') {
            let map = sol.map.get(pos).unwrap();
            let instruction = instructions.next().unwrap();
            match instruction {
                Instruction::L => {
                    *pos = map.0.clone();
                }
                Instruction::R => {
                    *pos = map.1.clone();
                }
            }
            *pos_end += 1;
        }
    }

    let mut ans = poss_end.get(0).unwrap().clone();
    for i in 1..poss_end.len() {
        ans = lcm(ans, poss_end.get(i).unwrap().clone());
    }

    ans
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn part1() {
    let input = include_str!("./input.txt");
    let mut sol = Sol::new();
    build_sol(input, &mut sol);
    println!("{}", part1_process(&sol));
}

pub fn part2() {
    let input = include_str!("./input.txt");
    let mut sol = Sol::new();
    build_sol(input, &mut sol);
    println!("{}", part2_process(&sol));
}

mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let mut sol = Sol::new();
        build_sol(input, &mut sol);
        assert_eq!(part1_process(&sol), 6);
    }

    #[test]
    fn example2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let mut sol = Sol::new();
        build_sol(input, &mut sol);
        assert_eq!(part2_process(&sol), 6);
    }
}
