use std::collections::BTreeMap;
use std::io::BufRead;
use crate::utils;


struct Cards {
    cards: BTreeMap<u32, Card>,
    counter: u32,
    // how many cards - part 2
    score: u32, // total score - part 1
}

impl Cards {
    fn new() -> Cards {
        Cards {
            cards: BTreeMap::new(),
            counter: 0,
            score: 0,
        }
    }
    pub fn add(&mut self, id: u32, winning: Option<Vec<u32>>, numbers: Option<Vec<u32>>) {
        let mut card = self.cards.entry(id).or_insert_with(|| {
            Card {
                _id: id,
                winning: vec![],
                numbers: vec![],
                score: 0,
                machs: 0,
                copies: 0,
            }
        });
        card.copies += 1;
        self.counter += 1;
        match (winning, numbers) {
            (Some(winning), Some(numbers)) => {
                card.winning = winning;
                card.numbers = numbers;
                for num in card.numbers.iter() {
                    if card.winning.contains(&num) {
                        card.machs += 1;
                        if card.score == 0 {
                            card.score = 1;
                        } else {
                            card.score *= 2;
                        }
                    }
                }
                self.score += card.score;
                let copies = card.copies;
                let machs = card.machs;
                for _ in 0..copies {
                    for i in 1..=machs {
                        self.add(id + i, None, None);
                    }
                }
            }
            _ => {}
        }
    }
}

struct Card {
    _id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
    score: u32,
    machs: u32,
    copies: u32,
}


pub fn part1() {
    let mut reader = utils::read_file("src/four/input1.txt");
    println!("{}", part1_process(&mut reader));
}

pub fn part2() {
    let mut reader = utils::read_file("src/four/input1.txt");
    println!("{}", part2_process(&mut reader));
}

fn part1_process<R: BufRead>(reader: &mut R) -> u32 {
    let mut cards = Cards::new();
    build_cards_table(reader, &mut cards);
    cards.score
}

fn part2_process<R: BufRead>(reader: &mut R) -> u32 {
    let mut cards = Cards::new();
    build_cards_table(reader, &mut cards);
    cards.counter
}

fn build_cards_table<R: BufRead>(reader: &mut R, cards: &mut Cards) {
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens = line.split(" ");
        let mut section = 1;
        let mut id: Option<u32> = None;
        let mut numbers = vec![];
        let mut winning = vec![];
        for token in tokens {
            if token.is_empty() {
                continue;
            }
            if token == "Card" {
                continue;
            }
            if token.ends_with(':') {
                section += 1;
                id = Some(token.replace(":", "").parse().unwrap());
                continue;
            }
            if token == "|" {
                section += 1;
                continue;
            }
            let n = token.parse().unwrap();
            if section == 2 {
                winning.push(n);
            } else if section == 3 {
                numbers.push(n);
            }
        }
        match id {
            None => {
                panic!("Could not parse game id {}", line);
            }
            Some(id) => {
                cards.add(id, Some(winning), Some(numbers));
            }
        }
    }
}

mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut reader = Cursor::new(input);
        assert_eq!(part1_process(&mut reader), 13);
    }

    #[test]
    fn part2_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut reader = Cursor::new(input);
        assert_eq!(part2_process(&mut reader), 30);
    }
}
