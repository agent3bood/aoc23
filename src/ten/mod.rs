pub fn part1() {
    let input = include_str!("./input.txt");
    println!("{}", part1_process(input));
}

pub fn part2() {
    todo!()
}

type Sol = Vec<Vec<Pipe>>;

fn is_monster(map: &Sol, pos: (usize, usize)) -> Option<bool> {
    Some(map.get(pos.0)?.get(pos.1)?.drawing == 'S')
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
    match node.drawing {
        '|' => {
            let n = south(map, pos);
            return if n.is_some() && !is_visited(map, n.unwrap()).unwrap_or(true) {
                n
            } else {
                north(map, pos)
            };
        }
        '-' => {
            let n = east(map, pos);
            return if n.is_some() && !is_visited(map, n.unwrap()).unwrap_or(true) {
                n
            } else {
                west(&map, pos)
            };
        }
        'L' => {
            let n = east(map, pos);
            return if n.is_some() && !is_visited(map, n.unwrap()).unwrap_or(true) {
                n
            } else {
                north(map, pos)
            };
        }
        'J' => {
            let n = west(map, pos);
            return if n.is_some() && !is_visited(map, n.unwrap()).unwrap_or(true) {
                n
            } else {
                north(&map, pos)
            };
        }
        '7' => {
            let n = west(map, pos);
            return if n.is_some() && !is_visited(map, n.unwrap()).unwrap_or(true) {
                n
            } else {
                south(&map, pos)
            };
        }
        'F' => {
            let n = east(map, pos);
            return if n.is_some() && !is_visited(map, n.unwrap()).unwrap_or(true) {
                n
            } else {
                south(&map, pos)
            };
        }
        _ => None,
    }
}

struct Pipe {
    drawing: char,
    x: usize,
    y: usize,
    visited: bool,
}

impl Pipe {
    fn new(p: char, x: usize, y: usize) -> Pipe {
        Pipe {
            drawing: p,
            x,
            y,
            visited: false,
        }
    }

    // fn is_monster(&self) -> bool {
    //     self.drawing == 'S'
    // }
    //
    // fn south<'a>(&'a self, map: &'a Vec<Vec<Pipe>>) -> Option<&Pipe> {
    //     map.get(self.x).unwrap().get(self.y - 1)
    // }
    // fn north<'a>(&'a self, map: &'a Vec<Vec<Pipe>>) -> Option<&Pipe> {
    //     map.get(self.x).unwrap().get(self.y + 1)
    // }
    // fn east<'a>(&'a self, map: &'a Vec<Vec<Pipe>>) -> Option<&Pipe> {
    //     map.get(self.x + 1).unwrap().get(self.y)
    // }
    // fn west<'a>(&'a self, map: &'a Vec<Vec<Pipe>>) -> Option<&Pipe> {
    //     map.get(self.x - 1).unwrap().get(self.y)
    // }

    // fn right<'a>(&'a self, map: &'a Vec<Vec<Pipe>>) -> Option<&Pipe> {
    //     // | is a vertical pipe connecting north and south.
    //     // - is a horizontal pipe connecting east and west.
    //     // L is a 90-degree bend connecting north and east.
    //     // J is a 90-degree bend connecting north and west.
    //     // 7 is a 90-degree bend connecting south and west.
    //     // F is a 90-degree bend connecting south and east.
    //     // . is ground; there is no pipe in this tile.
    //     // S is the starting position of the animal; there is a pipe on this
    //     match self.drawing {
    //         '|' => {
    //             let n = self.south(map);
    //             return if n.is_some() && !is_visited(map, n.unwrap())  {
    //                 n
    //             } else {
    //                 self.north(map)
    //             };
    //         }
    //         '-' => {
    //             let n = self.east(map);
    //             return if n.is_some() && !is_visited(map, n.unwrap())  {
    //                 n
    //             } else {
    //                 self.west(&map)
    //             };
    //         }
    //         'L' => {
    //             let n = self.east(map);
    //             return if n.is_some() && !is_visited(map, n.unwrap())  {
    //                 n
    //             } else {
    //                 self.north(map)
    //             };
    //         }
    //         'J' => {
    //             let n = self.west(map);
    //             return if n.is_some() && !is_visited(map, n.unwrap())  {
    //                 n
    //             } else {
    //                 self.north(&map)
    //             };
    //         }
    //         '7' => {
    //             let n = self.west(map);
    //             return if n.is_some() && !is_visited(map, n.unwrap())  {
    //                 n
    //             } else {
    //                 self.south(&map)
    //             };
    //         }
    //         'F' => {
    //             let n = self.east(map);
    //             return if n.is_some() && !is_visited(map, n.unwrap())  {
    //                 n
    //             } else {
    //                 self.south(&map)
    //             };
    //         }
    //         _ => None,
    //     }
    // }

    // fn left<'a>(&'a self, map: &'a Vec<Vec<Pipe>>) -> Option<&Pipe> {
    //     match self.drawing {
    //         '|' => self.north(map),
    //         '-' => self.west(map),
    //         'L' => self.west(map),
    //         'J' => self.east(map),
    //         '7' => self.east(map),
    //         'F' => self.west(map),
    //         _ => None,
    //     }
    // }
}

fn map_get(map: &Sol, pos: (usize, usize)) -> Option<&Pipe> {
    map.get(pos.0)?.get(pos.1)
}

fn part1_process(input: &str) -> u64 {
    let mut ans = 0;
    let mut map: Sol = vec![];
    let mut monster = (0, 0);
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, p) in line.chars().enumerate() {
            let pipe = Pipe::new(p, i, j);
            if p == 'S' {
                monster = (i, j);
            }
            row.push(pipe);
        }
        map.push(row);
    }

    let mut pos = monster;
    loop {
        if is_monster(&map, pos).unwrap_or(false) {
            map.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap().visited = true;
            let mut pos_next: Option<(usize, usize)> = None;
            match north(&map, pos) {
                Some(p) => {
                    let pipe = map_get(&map, p);
                    match pipe {
                        Some(pipe) => match pipe.drawing {
                            '|' | '7' | 'F' => {
                                pos_next = Some(p);
                            }
                            _ => {}
                        },
                        None => {}
                    };
                }
                _ => {}
            };
            if pos_next.is_none() {
                match south(&map, pos) {
                    Some(p) => {
                        let pipe = map_get(&map, p);
                        match pipe {
                            Some(pipe) => match pipe.drawing {
                                '|' | 'L' | 'J' => {
                                    pos_next = Some(p);
                                }
                                _ => {}
                            },
                            None => {}
                        };
                    }
                    _ => {}
                };
            }
            if pos_next.is_none() {
                match east(&map, pos) {
                    Some(p) => {
                        let pipe = map_get(&map, p);
                        match pipe {
                            Some(pipe) => match pipe.drawing {
                                '-' | 'J' | '7' => {
                                    pos_next = Some(p);
                                }
                                _ => {}
                            },
                            None => {}
                        };
                    }
                    _ => {}
                };
            }
            if pos_next.is_none() {
                match west(&map, pos) {
                    Some(p) => {
                        let pipe = map_get(&map, p);
                        match pipe {
                            Some(pipe) => match pipe.drawing {
                                '-' | 'L' | 'F' => {
                                    pos_next = Some(p);
                                }
                                _ => {}
                            },
                            None => {}
                        };
                    }
                    _ => {}
                };
            }
            println!("{:?} -> {:?}", pos, pos_next);
            pos = pos_next.unwrap();
        } else {
            let pos_x = right(&map, pos).unwrap();
            println!("{:?} -> {:?}", pos, pos_x);
            pos = pos_x;
        }
        ans += 1;
        map.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap().visited = true;
        if is_monster(&map, pos).unwrap_or(false) {
            break;
        }
    }

    ans / 2
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
        assert_eq!(part1_process(input), 4);
    }

    #[test]
    fn example2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part1_process(input), 8);
    }
}
