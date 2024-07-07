use std::{cmp::Ordering, fmt::Display};

use crate::parser::parse_game;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Card {
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Trubadour,
    Jack,
    Queen,
    King,
    Ace,
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Card::Two => '2',
                Card::Three => '3',
                Card::Four => '4',
                Card::Five => '5',
                Card::Six => '6',
                Card::Seven => '7',
                Card::Eight => '8',
                Card::Nine => '9',
                Card::Trubadour => 'T',
                Card::Jack => 'J',
                Card::Queen => 'Q',
                Card::King => 'K',
                Card::Ace => 'A',
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    OnePair,
    TwoPair,
    HighCard,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug)]
pub struct Hand {
    pub cards: Cards,
    pub bid: u16,
    pub hand_type: HandType,
}
impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: String = self.cards.iter().map(|c| format!("{}", c)).collect();
        write!(f, "{}", cards)
    }
}

type Cards = [Card; 5];

pub fn score(cards: &Cards) -> HandType {
    let mut cards = cards.clone();
    cards.sort();
    // dbg!(&cards);
    let mut cards = cards.into_iter();
    let mut groups = vec![];
    let mut prev = cards.next().unwrap();
    let mut count = 1;
    while let Some(card) = cards.next() {
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
    // dbg!(&groups);
    match groups[..] {
        [5] => HandType::FiveOfAKind,
        [4] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3] => HandType::ThreeOfAKind,
        [2, 2] => HandType::TwoPair,
        [2] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

impl From<([Card; 5], u16)> for Hand {
    fn from(value: ([Card; 5], u16)) -> Self {
        Self {
            cards: value.0,
            bid: value.1,
            hand_type: score(&value.0),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("test.txt");
    let (_, mut game) = parse_game(input).unwrap();
    game.sort_by(|a, b| {
        // println!("comparing {a:?} and {b:?}");
        if let Some(x) = a
            .cards
            .into_iter()
            .zip(&b.cards)
            .map(|(a, b)| a.cmp(b))
            .find(|x| !x.is_eq())
        {
            x
        } else {
            Ordering::Equal
        }
    });
    game.sort_by_key(|h| h.hand_type);

    let total_winnings: usize = game
        .iter()
        .enumerate()
        // .inspect(|(rank, hand)| println!("{:?} {}", hand.hand_type, hand))
        .map(|(rank, hand)| (rank + 1) * usize::from(hand.bid))
        .sum();

    println!("Total winnings: {}", total_winnings);
}

mod parser {
    use nom::bytes::complete::tag;
    use nom::character::complete::{self as cc, one_of};
    use nom::{multi::separated_list1, sequence::separated_pair, IResult};

    use crate::{Card, Cards, Hand};

    pub fn parse_game(game: &str) -> IResult<&str, Vec<Hand>> {
        separated_list1(tag("\n"), hand)(game)
    }
    fn hand(hand: &str) -> IResult<&str, Hand> {
        separated_pair(cards, tag(" "), cc::u16)(hand).map(|(rest, hand)| (rest, hand.into()))
    }

    fn cards(input: &str) -> IResult<&str, Cards> {
        let mut cards = [Card::default(); 5];
        let mut rest = input;
        for count in 0..5 {
            let drawn: Card;
            (rest, drawn) = card(rest)?;
            cards[count] = drawn;
        }

        Ok((rest, cards))
    }

    fn card(card: &str) -> IResult<&str, Card> {
        one_of("AKQJT98765432")(card).map(|(rest, drawn)| {
            use Card::*;
            (
                rest,
                match drawn {
                    '2' => Two,
                    '3' => Three,
                    '4' => Four,
                    '5' => Five,
                    '6' => Six,
                    '7' => Seven,
                    '8' => Eight,
                    '9' => Nine,
                    'T' => Trubadour,
                    'J' => Jack,
                    'Q' => Queen,
                    'K' => King,
                    'A' => Ace,
                    _ => unreachable!(),
                },
            )
        })
    }
}
