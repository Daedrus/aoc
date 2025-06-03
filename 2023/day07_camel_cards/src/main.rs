use log::{debug, info};
use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1},
    error::Error,
    Parser,
};
use std::{cmp::Ordering, collections::HashMap};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<(char, bool)> for Card {
    fn from((input, j_is_joker): (char, bool)) -> Self {
        match input {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => {
                if j_is_joker {
                    Card::Joker
                } else {
                    Card::J
                }
            }
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    category: Category,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.category == other.category {
            match self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find(|(card1, card2)| card1 != card2)
            {
                Some((card1, card2)) => card1.cmp(card2),
                None => Ordering::Equal,
            }
        } else {
            self.category.cmp(&other.category)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.category == other.category {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .any(|(card1, card2)| card1 != card2)
        } else {
            false
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(&str, bool)> for Hand {
    fn from((input, j_is_joker): (&str, bool)) -> Self {
        let (_, (card_str, _, bid)) = (alphanumeric1::<&str, Error<&str>>, tag(" "), complete::u32)
            .parse(input)
            .unwrap();

        // Convert the input string to our Card enum
        let cards: Vec<Card> = card_str.chars().map(|c| (c, j_is_joker).into()).collect();

        // Get the frequency of each Card in the hand
        let mut card_frequency_map: HashMap<&Card, u32> = HashMap::new();
        cards.iter().for_each(|card| {
            let frequency = card_frequency_map.entry(card).or_insert(0);
            *frequency += 1;
        });

        // Sort the frequencies so that we don't have to write all permutations
        // in the match arms below
        let mut card_frequency_values: Vec<&u32> = card_frequency_map.values().collect();
        card_frequency_values.sort();

        let category = match card_frequency_values[..] {
            [5] => Category::FiveOfAKind,
            [1, 4] => {
                if card_frequency_map.contains_key(&Card::Joker) {
                    Category::FiveOfAKind
                } else {
                    Category::FourOfAKind
                }
            }
            [2, 3] => {
                if card_frequency_map.contains_key(&Card::Joker) {
                    Category::FiveOfAKind
                } else {
                    Category::FullHouse
                }
            }
            [1, 1, 3] => {
                if card_frequency_map.contains_key(&Card::Joker) {
                    Category::FourOfAKind
                } else {
                    Category::ThreeOfAKind
                }
            }
            [1, 2, 2] => match card_frequency_map.get(&Card::Joker) {
                Some(frequency) if *frequency == 1 => Category::FullHouse,
                Some(frequency) if *frequency == 2 => Category::FourOfAKind,
                _ => Category::TwoPair,
            },
            [1, 1, 1, 2] => {
                if card_frequency_map.contains_key(&Card::Joker) {
                    Category::ThreeOfAKind
                } else {
                    Category::OnePair
                }
            }
            [1, 1, 1, 1, 1] => {
                if card_frequency_map.contains_key(&Card::Joker) {
                    Category::OnePair
                } else {
                    Category::HighCard
                }
            }
            _ => unreachable!(),
        };

        Hand {
            cards,
            bid,
            category,
        }
    }
}

fn parse_input(input: &mut impl BufRead, j_is_joker: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|line| (line.unwrap().as_str(), j_is_joker).into())
        .collect()
}

fn compute_winnings(hands: &mut [Hand]) -> u32 {
    hands.sort();

    debug!("{:?}", hands);

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1) as u32)
        .sum::<u32>()
}

fn part1(input: &mut impl BufRead) -> String {
    let mut hands = parse_input(input, false);

    compute_winnings(&mut hands).to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let mut hands = parse_input(input, true);

    compute_winnings(&mut hands).to_string()
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader));

    reader.rewind().unwrap();

    info!("Part 2 answer: {}", part2(&mut reader));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "253933213");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "253473930");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "6440");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "5905");
    }
}
