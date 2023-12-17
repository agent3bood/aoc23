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

fn part1_process(sol: &Sol) -> u32 {
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

pub fn part1() {
    let input = include_str!("./input.txt");
    let mut sol = Sol::new();
    build_sol(input, &mut sol);
    println!("{}", part1_process(&sol));
}

pub fn part2() {
    todo!()
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
}
