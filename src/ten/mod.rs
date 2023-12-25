pub fn part1() {
    let input = include_str!("./input.txt");
    let mut sol: Sol = vec![];
    println!("{}", part1_process(input, &mut sol));
}

pub fn part2() {
    let input = include_str!("./input.txt");
    let mut sol: Sol = vec![];
    println!("{}", part2_process(input, &mut sol));
}

type Sol = Vec<Vec<Pipe>>;

fn is_monster(map: &Sol, pos: (usize, usize)) -> Option<bool> {
    Some(map.get(pos.0)?.get(pos.1)?.monster)
}

fn is_visited(map: &Sol, pos: (usize, usize)) -> Option<bool> {
    Some(map.get(pos.0)?.get(pos.1)?.visited)
}

fn north(map: &Sol, pos: (usize, usize)) -> Option<(usize, usize)> {
    if pos.0 == 0 {
        return None;
    }
    let e = map.get(pos.0 - 1)?.get(pos.1)?;
    Some((e.x, e.y))
}
fn south(map: &Sol, pos: (usize, usize)) -> Option<(usize, usize)> {
    let e = map.get(pos.0 + 1)?.get(pos.1)?;
    Some((e.x, e.y))
}
fn east(map: &Sol, pos: (usize, usize)) -> Option<(usize, usize)> {
    let e = map.get(pos.0)?.get(pos.1 + 1)?;
    Some((e.x, e.y))
}

fn west(map: &Sol, pos: (usize, usize)) -> Option<(usize, usize)> {
    if pos.1 == 0 {
        return None;
    }
    let e = map.get(pos.0)?.get(pos.1 - 1)?;
    Some((e.x, e.y))
}

fn right(map: &Sol, pos: (usize, usize)) -> Option<(usize, usize)> {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this
    let node = map.get(pos.0)?.get(pos.1)?;
    let n;
    let n2;
    match node.drawing {
        '|' => {
            n = south(map, pos);
            n2 = north(map, pos);
        }
        '-' => {
            n = east(map, pos);
            n2 = west(map, pos);
        }
        'L' => {
            n = east(map, pos);
            n2 = north(map, pos);
        }
        'J' => {
            n = west(map, pos);
            n2 = north(map, pos);
        }
        '7' => {
            n = west(map, pos);
            n2 = south(map, pos);
        }
        'F' => {
            n = east(map, pos);
            n2 = south(map, pos);
        }
        _ => {
            panic!("Unknown map drawing")
        }
    }

    if is_visited(map, n?)? && is_visited(map, n2?)? {
        return if is_monster(map, n.unwrap())? {
            n
        } else if is_monster(map, n2.unwrap())? {
            n2
        } else {
            panic!()
        };
    }

    return if n.is_some() && !is_visited(map, n.unwrap()).unwrap_or(true) {
        n
    } else {
        n2
    };
}

struct Pipe {
    drawing: char,
    x: usize,
    y: usize,
    visited: bool,
    monster: bool,
}

impl Pipe {
    fn new(p: char, x: usize, y: usize) -> Pipe {
        Pipe {
            drawing: p,
            x,
            y,
            visited: false,
            monster: false,
        }
    }
}

fn map_get(map: &Sol, pos: (usize, usize)) -> Option<&Pipe> {
    map.get(pos.0)?.get(pos.1)
}

fn part1_process(input: &str, map: &mut Sol) -> u64 {
    let mut ans = 0;
    let mut monster = (0, 0);
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, p) in line.chars().enumerate() {
            let mut pipe = Pipe::new(p, i, j);
            if p == 'S' {
                pipe.monster = true;
                monster = (i, j);
            }
            row.push(pipe);
        }
        map.push(row);
    }

    let mut pos = monster;
    loop {
        if is_monster(&map, pos).unwrap_or(false) {
            let mut pos_next: Option<(usize, usize)> = None;
            let mut n = false;
            let mut s = false;
            let mut e = false;
            let mut w = false;
            match north(&map, pos) {
                Some(p) => {
                    let pipe = map_get(&map, p);
                    match pipe {
                        Some(pipe) => match pipe.drawing {
                            '|' | '7' | 'F' => {
                                n = true;
                                pos_next = Some(p);
                            }
                            _ => {}
                        },
                        None => {}
                    };
                }
                _ => {}
            };

            match south(&map, pos) {
                Some(p) => {
                    let pipe = map_get(&map, p);
                    match pipe {
                        Some(pipe) => match pipe.drawing {
                            '|' | 'L' | 'J' => {
                                s = true;
                                if pos_next.is_none() {
                                    pos_next = Some(p);
                                }
                            }
                            _ => {}
                        },
                        None => {}
                    };
                }
                _ => {}
            };

            match east(&map, pos) {
                Some(p) => {
                    let pipe = map_get(&map, p);
                    match pipe {
                        Some(pipe) => match pipe.drawing {
                            '-' | 'J' | '7' => {
                                e = true;
                                if pos_next.is_none() {
                                    pos_next = Some(p);
                                }
                            }
                            _ => {}
                        },
                        None => {}
                    };
                }
                _ => {}
            };

            match west(&map, pos) {
                Some(p) => {
                    let pipe = map_get(&map, p);
                    match pipe {
                        Some(pipe) => match pipe.drawing {
                            '-' | 'L' | 'F' => {
                                w = true;
                                if pos_next.is_none() {
                                    pos_next = Some(p);
                                }
                            }
                            _ => {}
                        },
                        None => {}
                    };
                }
                _ => {}
            };

            let pos_next = pos_next.unwrap();
            let mut monster = map.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap();
            monster.visited = true;
            if n && s {
                monster.drawing = '|';
            } else if n && e {
                monster.drawing = 'L';
            } else if n && w {
                monster.drawing = 'J';
            } else if s && e {
                monster.drawing = 'F';
            } else if s && w {
                monster.drawing = '7';
            } else if e && w {
                monster.drawing = '-';
            }
            println!("monster is on {}", monster.drawing);

            println!(
                "{:?} {} -> {:?} {}",
                pos,
                map_get(&map, pos).unwrap().drawing,
                pos_next,
                map_get(&map, pos_next).unwrap().drawing
            );
            pos = pos_next;
        } else {
            let pos_next = right(&map, pos).unwrap();
            println!(
                "{:?} {} -> {:?} {}",
                pos,
                map_get(&map, pos).unwrap().drawing,
                pos_next,
                map_get(&map, pos_next).unwrap().drawing
            );
            map.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap().visited = true;
            pos = pos_next;
        }
        ans += 1;
        if is_monster(&map, pos).unwrap_or(false) {
            break;
        }
    }

    ans / 2
}

fn part2_process(input: &str, map: &mut Sol) -> u64 {
    part1_process(input, map);
    let mut ans = 0;
    for row in map.iter() {
        let mut cross = 0;
        for pipe in row.iter() {
            let visited = pipe.visited;
            let drw = pipe.drawing;
            if visited && (drw == '|' || drw == 'F' || drw == '7') {
                cross += 1;
                continue;
            }
            if visited {
                continue;
            }

            if cross % 2 == 1 {
                ans += 1;
            }
        }
    }

    ans
}

mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let mut sol: Sol = vec![];
        assert_eq!(part1_process(input, &mut sol), 4);
    }

    #[test]
    fn example2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let mut sol: Sol = vec![];
        assert_eq!(part1_process(input, &mut sol), 8);
    }

    #[test]
    fn example3() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let mut sol: Sol = vec![];
        assert_eq!(part2_process(input, &mut sol), 4);
    }

    #[test]
    fn example4() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let mut sol: Sol = vec![];
        assert_eq!(part2_process(input, &mut sol), 8);
    }

    #[test]
    fn example5() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let mut sol: Sol = vec![];
        assert_eq!(part2_process(input, &mut sol), 10);
    }
}
