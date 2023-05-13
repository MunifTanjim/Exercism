use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    FiveOfAKind,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    hand: &'a str,
    category: Category,
    ranks: Vec<u8>,
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.category.cmp(&self.category) {
            Ordering::Equal => Some(self.ranks.cmp(&other.ranks)),
            order => Some(order),
        }
    }
}

impl<'a> Hand<'a> {
    fn new(hand: &'a str) -> Self {
        let (suits, mut ranks): (Vec<u8>, Vec<u8>) = hand
            .split_ascii_whitespace()
            .map(|card| {
                let (rank, suit) = card.split_at(card.len() - 1);
                (
                    suit.as_bytes()[0],
                    match rank.parse::<u8>() {
                        Ok(n) => n,
                        _ => "JQKA".find(rank).unwrap() as u8 + 11,
                    },
                )
            })
            .unzip();

        let mut count_by_rank = ranks.iter().fold(HashMap::new(), |mut map, &rank| {
            *map.entry(rank).or_insert(0) += 1;
            map
        });

        let mut sorted_rank_count = count_by_rank
            .values()
            .map(|count| *count as u8)
            .collect::<Vec<u8>>();
        sorted_rank_count.sort_unstable_by(|a, b| b.cmp(a));

        let category = match &sorted_rank_count[..] {
            [4, 1] => {
                if ranks[0] == 11 || ranks[4] == 11 {
                    Category::FiveOfAKind
                } else {
                    Category::FourOfAKind
                }
            }
            [3, 2] => Category::FullHouse,
            [3, 1, 1] => Category::ThreeOfAKind,
            [2, 2, 1] => Category::TwoPair,
            [2, 1, 1, 1] => Category::OnePair,
            _ => {
                ranks.sort_unstable_by(|a, b| b.cmp(&a));
                if ranks == [14, 5, 4, 3, 2] {
                    ranks = vec![5, 4, 3, 2, 1];
                    count_by_rank.remove(&14);
                    count_by_rank.insert(1, 1);
                }

                let is_straight = ranks[0] - ranks[4] == 4;
                let is_flush = suits[1..].iter().all(|&suit| suit == suits[0]);

                match (is_flush, is_straight) {
                    (true, true) => Category::StraightFlush,
                    (true, false) => Category::Flush,
                    (false, true) => Category::Straight,
                    (false, false) => Category::HighCard,
                }
            }
        };
        let mut ranks: Vec<u8> = count_by_rank.keys().map(|&c| c).collect();
        ranks.sort_unstable_by(|a, b| {
            match count_by_rank
                .get(b)
                .unwrap()
                .cmp(&count_by_rank.get(a).unwrap())
            {
                Ordering::Equal => b.cmp(a),
                order => order,
            }
        });

        Self {
            hand,
            category,
            ranks,
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: Vec<Hand> = hands.iter().map(|&hand| Hand::new(hand)).collect();

    hands.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));

    hands
        .iter()
        .take_while(|&a| a.partial_cmp(&hands[0]).unwrap_or(Ordering::Equal) == Ordering::Equal)
        .map(|hand| hand.hand)
        .collect()
}
