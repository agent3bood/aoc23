fn part1_process(input: &str) -> i64 {
    let mut ans: i64 = 0;
    for line in input.lines() {
        ans += extrapolate(line);
    }
    ans
}

fn extrapolate(line: &str) -> i64 {
    let mut matrex: Vec<Vec<i64>> = vec![];
    matrex.push(
        line.split_whitespace()
            .filter_map(|e| e.parse::<i64>().ok())
            .collect(),
    );
    let mut i = 0;
    loop {
        let mut all_zeros = true;
        let mut row_next = vec![];
        let row = matrex.get(i).unwrap();
        for j in 1..row.len() {
            let n = row.get(j).unwrap() - row.get(j - 1).unwrap();
            if all_zeros && n != 0 {
                all_zeros = false;
            }
            row_next.push(n);
        }
        matrex.push(row_next);
        i += 1;
        if all_zeros {
            break;
        }
    }

    let mut extrapolated = 0;
    for row in matrex.iter().rev() {
        let l = row.len();
        let last = row.get(l - 1).unwrap();
        extrapolated = last + extrapolated;
    }

    extrapolated
}

fn part2_process(input: &str) -> i64 {
    let mut ans: i64 = 0;
    for line in input.lines() {
        ans += extrapolate_back(line);
    }
    ans
}

fn extrapolate_back(line: &str) -> i64 {
    let mut matrex: Vec<Vec<i64>> = vec![];
    matrex.push(
        line.split_whitespace()
            .filter_map(|e| e.parse::<i64>().ok())
            .collect(),
    );
    let mut i = 0;
    loop {
        let mut all_zeros = true;
        let mut row_next = vec![];
        let row = matrex.get(i).unwrap();
        for j in 1..row.len() {
            let n = row.get(j).unwrap() - row.get(j - 1).unwrap();
            if all_zeros && n != 0 {
                all_zeros = false;
            }
            row_next.push(n);
        }
        matrex.push(row_next);
        i += 1;
        if all_zeros {
            break;
        }
    }

    let mut extrapolated = 0;
    for row in matrex.iter().rev() {
        let first = row.get(0).unwrap();
        extrapolated = first - extrapolated;
    }

    extrapolated
}

pub fn part1() {
    let input = include_str!("./input.txt");
    println!("{}", part1_process(input))
}

pub fn part2() {
    let input = include_str!("./input.txt");
    println!("{}", part2_process(input))
}

mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part1_process(input), 114);
    }

    #[test]
    fn example2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part2_process(input), 2);
    }
}
