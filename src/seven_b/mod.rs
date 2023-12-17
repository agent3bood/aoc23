use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug)]
struct Card {
    card: char,
}

impl Card {
    fn new(c: char) -> Card {
        Card { card: c }
    }

    fn value(&self) -> u8 {
        match self.card {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card rank"),
        }
    }

    fn is_joker(&self) -> bool {
        self.card == 'J'
    }
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(&other.value())
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

struct Hand {
    hand: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new() -> Hand {
        Hand {
            hand: vec![],
            bid: 0,
        }
    }

    fn cmp_cards(&self, other: &Self) -> Option<Ordering> {
        for (card, card_other) in self.hand.iter().zip(&other.hand) {
            if card.value() > card_other.value() {
                return Some(Ordering::Greater);
            }
            if card.value() < card_other.value() {
                return Some(Ordering::Less);
            }
        }

        Some(Ordering::Equal)
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let mut counter: BTreeMap<Card, u32> = BTreeMap::default();
        for c in self.hand.iter() {
            *counter.entry(*c).or_insert(0) += 1;
        }
        let jokers = match counter.get(&Card::new('J')) {
            Some(v) => *v,
            None => 0,
        };
        if let Some((&max_key, _)) = counter
            .iter()
            .filter(|(card, _)| !card.is_joker())
            .max_by_key(|&(_, v)| v)
        {
            if let Some(value) = counter.get_mut(&max_key) {
                *value += jokers;
            }
        }
        counter.remove(&Card::new('J'));
        let len = if jokers == 5 { 1 } else { counter.len() };

        let mut counter_other: BTreeMap<Card, u32> = BTreeMap::default();
        for c in other.hand.iter() {
            *counter_other.entry(*c).or_insert(0) += 1;
        }
        let jokers_other = match counter_other.get(&Card::new('J')) {
            Some(v) => *v,
            None => 0,
        };
        if let Some((&max_key, _)) = counter_other
            .iter()
            .filter(|(card, _)| !card.is_joker())
            .max_by_key(|&(_, v)| v)
        {
            if let Some(value) = counter_other.get_mut(&max_key) {
                *value += jokers_other;
            }
        }
        counter_other.remove(&Card::new('J'));
        let len_other = if jokers_other == 5 {
            1
        } else {
            counter_other.len()
        };

        let five_of_kind = len == 1;
        let five_of_kind_other = len_other == 1;
        if five_of_kind {
            return if five_of_kind_other {
                self.cmp_cards(other)
            } else {
                Some(Ordering::Greater)
            };
        } else if five_of_kind_other {
            return Some(Ordering::Less);
        }

        let four_of_kind = len == 2 && counter.iter().any(|e| *e.1 == 4);
        let four_of_kind_other = len_other == 2 && counter_other.iter().any(|e| *e.1 == 4);
        if four_of_kind {
            return if four_of_kind_other {
                self.cmp_cards(other)
            } else {
                Some(Ordering::Greater)
            };
        } else if four_of_kind_other {
            return Some(Ordering::Less);
        }

        let full_house = len == 2 && counter.iter().any(|e| *e.1 == 3);
        let full_house_other = len_other == 2 && counter_other.iter().any(|e| *e.1 == 3);
        if full_house {
            return if full_house_other {
                self.cmp_cards(other)
            } else {
                Some(Ordering::Greater)
            };
        } else if full_house_other {
            return Some(Ordering::Less);
        }

        let three_of_kind = len == 3 && counter.iter().any(|e| *e.1 == 3);
        let three_of_kind_other = len_other == 3 && counter_other.iter().any(|e| *e.1 == 3);
        if three_of_kind {
            return if three_of_kind_other {
                self.cmp_cards(other)
            } else {
                Some(Ordering::Greater)
            };
        } else if three_of_kind_other {
            return Some(Ordering::Less);
        }

        let two_pair = len == 3 && counter.iter().any(|e| *e.1 == 1);
        let two_pair_other = len_other == 3 && counter_other.iter().any(|e| *e.1 == 1);
        if two_pair {
            return if two_pair_other {
                self.cmp_cards(other)
            } else {
                Some(Ordering::Greater)
            };
        } else if two_pair_other {
            return Some(Ordering::Less);
        }

        let one_pair = len == 4;
        let one_pair_other = len_other == 4;
        if one_pair {
            return if one_pair_other {
                self.cmp_cards(other)
            } else {
                Some(Ordering::Greater)
            };
        } else if one_pair_other {
            return Some(Ordering::Less);
        }

        self.cmp_cards(other)
    }
}

pub fn part2() {
    let input = include_str!("./input1.txt");
    let mut sol = vec![];
    build_sol(input, &mut sol);
    println!("{}", part2_process(&mut sol));
}

fn build_sol(input: &str, sol: &mut Vec<Hand>) {
    for line in input.lines() {
        let mut bid = Hand::new();
        let mut parts = line.split_whitespace();
        for c in parts.next().unwrap().chars() {
            bid.hand.push(Card::new(c));
        }
        bid.bid = parts.next().unwrap().parse::<u32>().unwrap();
        sol.push(bid);
    }
}

fn part2_process(sol: &mut Vec<Hand>) -> u32 {
    sol.sort_unstable();
    let mut ans: u32 = 0;
    for (i, hand) in sol.iter().enumerate() {
        ans += hand.bid * (i + 1) as u32;
    }

    ans
}

mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let mut sol = vec![];
        build_sol(input, &mut sol);
        assert_eq!(part2_process(&mut sol), 5905);
    }
}
