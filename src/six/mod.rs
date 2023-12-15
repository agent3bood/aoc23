pub struct Sol {
    results: Vec<(u64, u64)>,
}

impl Sol {
    fn new() -> Sol {
        Sol { results: vec![] }
    }

    fn build(&mut self, input: &str) {
        let mut lines = input.lines();
        let time = lines
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .split_whitespace();
        let dist = lines
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .split_whitespace();
        for (time, dist) in time.zip(dist) {
            self.results
                .push((time.parse::<u64>().unwrap(), dist.parse::<u64>().unwrap()));
        }
    }

    fn build2(&mut self, input: &str) {
        let mut lines = input.lines();
        let time: String = lines
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect();
        let dist: String = lines
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect();

        self.results
            .push((time.parse::<u64>().unwrap(), dist.parse::<u64>().unwrap()));
    }

    fn part1_process(&self) -> u64 {
        let mut ans = 0;
        for e in self.results.iter() {
            let mut ans_e = 0;
            for i in 1..e.0 {
                let can_travel = (e.0 - i) * i;
                if can_travel > e.1 {
                    ans_e += 1;
                }
            }
            if ans_e > 0 {
                if ans == 0 {
                    ans = ans_e;
                } else {
                    ans *= ans_e;
                }
            }
        }
        ans
    }
}

pub fn part1() {
    let input = "Time:        48     93     84     66
Distance:   261   1192   1019   1063";
    let mut sol = Sol::new();
    sol.build(input);
    println!("{}", sol.part1_process());
}

pub fn part2() {
    let input = "Time:        48     93     84     66
Distance:   261   1192   1019   1063";
    let mut sol = Sol::new();
    sol.build2(input);
    println!("{}", sol.part1_process());
}

mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let mut sol = Sol::new();
        sol.build(input);
        assert_eq!(sol.part1_process(), 288);
    }
}
