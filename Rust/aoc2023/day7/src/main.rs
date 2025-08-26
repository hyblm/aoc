use std::fmt::Write;
use std::{cmp::Ordering, fmt::Display};

fn main() {
    let input = include_str!("input.txt");
    let (_, mut game) = parse::game(input).unwrap();
    let total_winnings: usize = eval_winnings::<true>(&mut game);

    println!("Total winnings: {}", total_winnings);
}

fn eval_winnings<const JOKERS: bool>(game: &mut [Hand]) -> usize {
    if JOKERS {
        for hand in game.iter_mut() {
            apply_jokers(hand);
        }
    }

    game.sort_by(|a, b| {
        let mut ordering = Ordering::Equal;
        for (a, b) in a.cards.iter().zip(&b.cards) {
            ordering = a.cmp(b);
            if !ordering.is_eq() {
                break;
            }
        }
        ordering
    });
    game.sort_by_key(|h| h.kind);

    game.iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * usize::from(hand.bid))
        .sum()
}

fn apply_jokers(hand: &mut Hand) {
    let joker_count = hand
        .cards
        .iter_mut()
        .filter(|card| **card == Card::Jack)
        .map(|card| *card = Card::Joker)
        .count();

    if joker_count == 0 {
        return;
    }
    eprint!("{hand}\t{:?}", hand.kind);
    eprint!("  -{joker_count}->  ");

    use HandKind::*;
    hand.kind = match (joker_count, hand.kind) {
        (0 | 5, _) => hand.kind,
        (1, HighCard) => OnePair,
        (1 | 2, OnePair) => ThreeOfAKind,
        (1, TwoPair) => FullHouse,
        (1, FullHouse) | (2, TwoPair) | (1 | 3, ThreeOfAKind) => FourOfAKind,
        (_, FourOfAKind) | (2 | 3, FullHouse) => FiveOfAKind,
        _ => unreachable!(),
    };
    eprintln!("{:?}", hand.kind);
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Card {
    Joker = b'1',
    Two, Three, Four, Five, Six, Seven, Eight, Nine,
    Trubadour, Jack, Queen, King, Ace,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl From<Card> for char {
    fn from(value: Card) -> Self {
        use Card::*;
        match value {
            Joker => 'J',
            Two | Three | Four | Five | Six | Seven | Eight | Nine => char::from(value as u8),
            Trubadour => 'T',
            Jack => 'J',
            Queen => 'Q',
            King => 'K',
            Ace => 'A',
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug)]
pub struct Hand {
    pub cards: [Card; 5],
    pub bid: u16,
    pub kind: HandKind,
}
impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: String = self
            .cards
            .iter()
            .fold(String::with_capacity(5), |mut hand_str, c| {
                let _ = write!(hand_str, "{c}");
                hand_str
            });
        write!(f, "{}", cards)
    }
}

pub fn score(cards: &[Card; 5]) -> HandKind {
    let mut cards = *cards;
    cards.sort();
    let mut cards = cards.into_iter();
    let mut groups = vec![];
    let mut prev = cards.next().unwrap();
    let mut count = 1;
    for card in cards {
        if card.eq(&prev) {
            count += 1;
        } else if count > 1 {
            groups.push(count);
            count = 1;
        };
        prev = card;
    }
    if count > 1 {
        groups.push(count);
    }
    groups.sort_unstable();
    match groups[..] {
        [5] => HandKind::FiveOfAKind,
        [4] => HandKind::FourOfAKind,
        [3, 2] => HandKind::FullHouse,
        [2, 3] => HandKind::FullHouse,
        [3] => HandKind::ThreeOfAKind,
        [2, 2] => HandKind::TwoPair,
        [2] => HandKind::OnePair,
        _ => HandKind::HighCard,
    }
}

impl From<([Card; 5], u16)> for Hand {
    fn from(value: ([Card; 5], u16)) -> Self {
        Self {
            cards: value.0,
            bid: value.1,
            kind: score(&value.0),
        }
    }
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{self as cc, one_of};
    use nom::{multi::separated_list1, sequence::separated_pair, IResult};

    use crate::{Card, Hand};

    pub fn game(game: &str) -> IResult<&str, Vec<Hand>> {
        separated_list1(tag("\n"), hand)(game)
    }

    fn hand(hand: &str) -> IResult<&str, Hand> {
        separated_pair(cards, tag(" "), cc::u16)(hand).map(|(rest, hand)| (rest, hand.into()))
    }

    fn cards(input: &str) -> IResult<&str, [Card; 5]> {
        let mut cards = [Card::Joker; 5];
        let mut rest = input;
        for drawn in &mut cards {
            (rest, *drawn) = card(rest)?;
        }
        Ok((rest, cards))
    }

    fn card(card: &str) -> IResult<&str, Card> {
        one_of("AKQJT98765432")(card).map(|(rest, drawn)| {
            use Card::*;
            (
                rest,
                match drawn {
                    '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => unsafe {
                        std::mem::transmute::<u8, Card>(drawn as u8)
                    },
                    'T' => Trubadour,
                    'J' => Jack,
                    'Q' => Queen,
                    'K' => King,
                    'A' => Ace,
                    _ => unreachable!("invalid input"),
                },
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{eval_winnings, parse::game};
    use std::fmt::Write;

    #[test]
    fn example() {
        let input = include_str!("test.txt");
        let (_, mut game) = game(input).unwrap();
        assert_eq!(eval_winnings::<false>(&mut game), 6440);
    }

    #[test]
    fn example_joker_rules() {
        let input = include_str!("test.txt");
        let (_, mut game) = game(input).unwrap();
        assert_eq!(eval_winnings::<true>(&mut game), 5905);
    }

    #[test]
    fn test_parsing() {
        let input = include_str!("test.txt");
        let (_, game) = game(input).unwrap();

        let mut game_str = String::with_capacity(input.len());
        for hand in game {
            let _ = writeln!(game_str, "{hand} {}", hand.bid);
        }

        assert_eq!(game_str, input);
    }

    #[test]
    fn input_parsing() {
        let input = include_str!("test.txt");
        let (_, game) = game(input).unwrap();

        let mut game_str = String::with_capacity(input.len());
        for hand in game {
            let _ = writeln!(game_str, "{hand} {}", hand.bid);
        }

        assert_eq!(game_str, input);
    }
}
