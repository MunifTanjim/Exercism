use std::collections::HashMap;

const DIGITS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn permutations<T: Clone + Eq>(items: Vec<T>, n: usize) -> impl Iterator<Item = Vec<T>> {
    if n == 0 {
        return vec![vec![]].into_iter();
    }

    items
        .clone()
        .iter()
        .flat_map(move |item| {
            let mut items = items.clone();
            items.retain(|x| x != item);
            permutations(items, n - 1).map(move |mut perm| {
                perm.push(item.clone());
                perm
            })
        })
        .collect::<Vec<Vec<T>>>()
        .into_iter()
}

struct Letter {
    c: char,
    v: u8,
}

struct Alphametics {
    letters: Vec<Letter>,
    words: Vec<Vec<usize>>,
}

impl From<&str> for Alphametics {
    fn from(input: &str) -> Self {
        let input = input.replace("==", "+");
        let mut map = HashMap::new();

        let mut letters = vec![];
        let words = input
            .split("+")
            .collect::<Vec<&str>>()
            .iter()
            .map(|part| {
                part.trim()
                    .chars()
                    .map(|c| {
                        *map.entry(c).or_insert_with(|| {
                            letters.push(Letter { c, v: 0 });
                            letters.len() - 1
                        })
                    })
                    .collect()
            })
            .collect();

        Self { letters, words }
    }
}

impl Alphametics {
    fn try_solve(&mut self, permutation: Vec<u8>) -> Option<HashMap<char, u8>> {
        for (i, letter) in self.letters.iter_mut().enumerate() {
            let value = permutation[i];
            letter.v = value;
        }

        match self.is_valid() {
            false => None,
            _ => Some(HashMap::from_iter(
                self.letters.iter().map(|letter| (letter.c, letter.v)),
            )),
        }
    }

    fn is_valid(&self) -> bool {
        let word_count = self.words.len();

        if (0..word_count).any(|idx| self.letters[self.words[idx][0]].v == 0) {
            return false;
        }

        return (0..word_count - 1)
            .map(|idx| self.word_value(idx))
            .sum::<u64>()
            == self.word_value(word_count - 1);
    }

    fn word_value(&self, idx: usize) -> u64 {
        let mut value: u64 = 0;
        for (i, &index) in self.words[idx].iter().rev().enumerate() {
            value += 10_u64.pow(i as u32) * self.letters[index].v as u64;
        }
        value
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut game = Alphametics::from(input);

    if game.words.len() < 3 {
        return None;
    }

    for permutation in permutations(DIGITS.to_vec(), game.letters.len()) {
        if let Some(map) = game.try_solve(permutation) {
            return Some(map);
        }
    }

    None
}
