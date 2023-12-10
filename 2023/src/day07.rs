use std::{
    cmp::Ordering,
    collections::HashMap,
    fs,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Card(char);

impl Card {
    fn to_value(&self) -> u8 {
        match &self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            n => n.to_digit(10).unwrap() as u8,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_value().cmp(&other.to_value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAkind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    value: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type(false).cmp(&other.hand_type(false)).then_with(|| {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(x, y)| x.cmp(y))
                .find(|&v| v != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn hand_type(&self, count_jokers: bool) -> HandType {
        let mut card_set: HashMap<Card, u8> = HashMap::new();
        card_set.insert(Card('J'), 0);

        let mut jokers = 0;
        for &card in &self.cards {
            if card.0 == 'J' && count_jokers {
                jokers += 1;
            } else {
                card_set.entry(card).and_modify(|x| *x += 1).or_insert(1);
            }
        }

        if card_set.iter().any(|(_, &n)| n + jokers == 5) {
            HandType::FiveOfAkind
        } else if card_set.iter().any(|(_, &n)| n + jokers == 4) {
            HandType::FourOfAKind
        } else if (card_set.iter().any(|(_, &n)| n == 3) && card_set.iter().any(|(_, &n)| n == 2)) || (card_set.iter().filter(|(_, &n)| n == 2).count() == 2 && jokers == 1) {
            HandType::FullHouse
        } else if card_set.iter().any(|(_, &n)| n + jokers == 3) {
            HandType::ThreeOfAKind
        } else if card_set.iter().filter(|(_, &n)| n == 2).count() == 2 || (card_set.iter().any(|(_, &n)| n == 2) && jokers == 1) {
            HandType::TwoPair
        } else if card_set.iter().any(|(_, &n)| n + jokers == 2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn parse(inp: &str) -> Self {
        let (left, right) = inp.split_once(' ').unwrap();
        let cards: Vec<Card> = left.chars().map(|x| Card(x)).collect();
        Hand {
            cards,
            value: right.parse().unwrap()
        }
    }

    fn sort_part_two(&self, other: &Hand) -> Ordering {
        self.hand_type(true).cmp(&other.hand_type(true)).then_with(|| {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(mut x, mut y)| {
                    if x.0 == 'J' {
                        x = &Card('0')
                    }

                    if y.0 == 'J' {
                        y = &Card('0')
                    }

                    x.cmp(y)
                })
                .find(|&v| v != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
    }
}

pub fn main() {
    let file: String = fs::read_to_string("puzzles/7.txt").unwrap();
    let mut hands: Vec<Hand> = file
        .lines()
        .map(|hand| Hand::parse(hand))
        .collect();

    hands.sort();
    println!("Exercise 1: {}", hands.iter().enumerate().map(|(i, hand)| (i + 1) * hand.value).sum::<usize>());

    hands.sort_by(Hand::sort_part_two);
    println!("Exercise 2: {}", hands.iter().enumerate().map(|(i, hand)| (i + 1) * hand.value).sum::<usize>());
}
