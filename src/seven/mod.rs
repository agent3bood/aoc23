use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Formatter;

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
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card rank"),
        }
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

    fn new_with_cards(cards: &str) -> Hand {
        Hand {
            hand: cards.chars().map(|c| Card::new(c)).collect(),
            bid: 0,
        }
    }

    fn cmp_cards(&self, other: &Self) -> Ordering {
        for (card, card_other) in self.hand.iter().zip(&other.hand) {
            if card > card_other {
                return Ordering::Greater;
            }
            if card < card_other {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            String::from_iter(self.hand.iter().map(|w| w.card))
        )
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

fn partial_cmp_part1(first: &Hand, other: &Hand) -> Ordering {
    let mut counter: BTreeMap<Card, u32> = BTreeMap::default();
    for c in first.hand.iter() {
        *counter.entry(*c).or_insert(0) += 1;
    }
    let mut counter_other: BTreeMap<Card, u32> = BTreeMap::default();
    for c in other.hand.iter() {
        *counter_other.entry(*c).or_insert(0) += 1;
    }

    let five_of_kind = counter.len() == 1;
    let five_of_kind_other = counter_other.len() == 1;
    if five_of_kind {
        return if five_of_kind_other {
            first.cmp_cards(other)
        } else {
            Ordering::Greater
        };
    } else if five_of_kind_other {
        return Ordering::Less;
    }

    let four_of_kind = counter.len() == 2 && counter.iter().any(|e| *e.1 == 4);
    let four_of_kind_other = counter_other.len() == 2 && counter_other.iter().any(|e| *e.1 == 4);
    if four_of_kind {
        return if four_of_kind_other {
            first.cmp_cards(other)
        } else {
            Ordering::Greater
        };
    } else if four_of_kind_other {
        return Ordering::Less;
    }

    let full_house = counter.len() == 2 && counter.iter().any(|e| *e.1 == 3);
    let full_house_other = counter_other.len() == 2 && counter_other.iter().any(|e| *e.1 == 3);
    if full_house {
        return if full_house_other {
            first.cmp_cards(other)
        } else {
            Ordering::Greater
        };
    } else if full_house_other {
        return Ordering::Less;
    }

    let three_of_kind = counter.len() == 3 && counter.iter().any(|e| *e.1 == 3);
    let three_of_kind_other = counter_other.len() == 3 && counter_other.iter().any(|e| *e.1 == 3);
    if three_of_kind {
        return if three_of_kind_other {
            first.cmp_cards(other)
        } else {
            Ordering::Greater
        };
    } else if three_of_kind_other {
        return Ordering::Less;
    }

    let two_pair = !three_of_kind && counter.len() == 3 && counter.iter().any(|e| *e.1 == 1);
    let two_pair_other =
        !three_of_kind_other && counter_other.len() == 3 && counter_other.iter().any(|e| *e.1 == 1);
    if two_pair {
        return if two_pair_other {
            first.cmp_cards(other)
        } else {
            Ordering::Greater
        };
    } else if two_pair_other {
        return Ordering::Less;
    }

    let one_pair = counter.len() == 4 && counter.iter().any(|e| *e.1 == 2);
    let one_pair_other = counter_other.len() == 4 && counter_other.iter().any(|e| *e.1 == 2);
    if one_pair {
        return if one_pair_other {
            first.cmp_cards(other)
        } else {
            Ordering::Greater
        };
    } else if one_pair_other {
        return Ordering::Less;
    }

    first.cmp_cards(other)
}

pub fn part1() {
    let input = include_str!("./input1.txt");
    let mut sol = vec![];
    build_sol(input, &mut sol);
    println!("{}", part1_process(&mut sol));
}

pub fn part2() {
    todo!()
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

fn part1_process(sol: &mut Vec<Hand>) -> u32 {
    sol.sort_unstable_by(partial_cmp_part1);
    let mut ans: u32 = 0;
    for (i, bid) in sol.iter().enumerate() {
        ans += bid.bid * (i + 1) as u32;
    }

    ans
}

mod tests {
    use super::*;

    #[test]
    fn sort1() {
        let mut v1 = vec![
            Hand::new_with_cards("QQQJA"),
            Hand::new_with_cards("T55J5"),
            Hand::new_with_cards("32T3K"),
            Hand::new_with_cards("KK677"),
            Hand::new_with_cards("KTJJT"),
        ];

        let v2 = vec![
            Hand::new_with_cards("32T3K"), // 1
            Hand::new_with_cards("KTJJT"), // 2
            Hand::new_with_cards("KK677"), // 3
            Hand::new_with_cards("T55J5"), // 4
            Hand::new_with_cards("QQQJA"), // 5
        ];
        v1.sort_unstable_by(partial_cmp_part1);

        assert_eq!(v1, v2);
    }

    #[test]
    fn example1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let mut sol = vec![];
        build_sol(input, &mut sol);
        assert_eq!(part1_process(&mut sol), 6440);
    }
}
