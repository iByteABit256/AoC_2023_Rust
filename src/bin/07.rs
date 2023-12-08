advent_of_code::solution!(7);
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
enum Combination {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

impl Hand {
    fn eval(&self) -> Combination {
        let char_count = count_characters(&self.cards);

        if char_count[0].1 == 5 {
            Combination::FiveOfAKind
        } else if char_count[0].1 == 4 {
            Combination::FourOfAKind
        } else if char_count[0].1 == 3 && char_count[1].1 == 2 {
            Combination::FullHouse
        } else if char_count[0].1 == 3 {
            Combination::ThreeOfAKind
        } else if char_count[0].1 == 2 && char_count[1].1 == 2 {
            Combination::TwoPair
        } else if char_count[0].1 == 2 {
            Combination::OnePair
        } else {
            Combination::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let combination_diff = self.eval() as i32 - other.eval() as i32;

        match combination_diff.cmp(&0) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for it in self.cards.chars().zip(other.cards.chars()) {
                    let (self_it, other_it) = it;
                    let diff = compare_strengths(self_it, other_it);

                    if diff != Ordering::Equal {
                        return diff;
                    }
                }

                Ordering::Equal
            }
        }
    }
}

fn count_characters(s: &str) -> Vec<(char, usize)> {
    let mut char_count = HashMap::new();
    let mut joker_count = 0;

    for ch in s.chars() {
        // Joker
        if ch == 'j' {
            joker_count += 1;
            continue;
        }

        let count = char_count.entry(ch).or_insert(0);
        *count += 1;
    }

    let mut char_count_vec: Vec<_> = char_count.into_iter().collect();

    char_count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    if char_count_vec.is_empty() {
        // If it's empty it means that all cards were jokers
        char_count_vec.push(('j', 5));
    } else {
        char_count_vec[0].1 += joker_count;
    }

    char_count_vec
}

fn compare_strengths(a: char, b: char) -> Ordering {
    strength_score(a).cmp(&strength_score(b))
}

fn strength_score(c: char) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        'j' => 1,
        _ => c.to_digit(10).expect("Incorrect character") as i32,
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();

        let hand = Hand {
            cards: parts[0].to_owned(),
            bid: parts[1].parse::<u32>().expect("Incorrect bid format"),
        };

        hands.push(hand);
    }

    hands.sort_unstable();

    Some(
        hands
            .into_iter()
            .enumerate()
            .map(|(idx, hand)| (idx as u32 + 1) * hand.bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // For the Joker I use a small case j instead so I can make the code generic
    part_one(&input.replace('J', "j"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117608084));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(116776127));
    }
}
