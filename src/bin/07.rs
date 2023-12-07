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
    fn eval(&self) -> (Combination, Vec<(char, usize)>) {
        let char_count = count_characters(&self.cards);

        if char_count[0].1 == 5 {
            (Combination::FiveOfAKind, char_count)
        } else if char_count[0].1 == 4 {
            (Combination::FourOfAKind, char_count)
        } else if char_count[0].1 == 3 && char_count[1].1 == 2 {
            (Combination::FullHouse, char_count)
        } else if char_count[0].1 == 3 {
            (Combination::ThreeOfAKind, char_count)
        } else if char_count[0].1 == 2 && char_count[1].1 == 2 {
            (Combination::TwoPair, char_count)
        } else if char_count[0].1 == 2 {
            (Combination::OnePair, char_count)
        } else {
            (Combination::HighCard, char_count)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let (self_combination, self_counts) = self.eval();
        let (other_combination, other_counts) = other.eval();
        let combination_diff = self_combination as i32 - other_combination as i32;

        match combination_diff.cmp(&0) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for it in self_counts.iter().zip(other_counts.iter()) {
                    let (self_it, other_it) = it;
                    let diff = compare_strengths(self_it.0, other_it.0);

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

    for ch in s.chars() {
        let count = char_count.entry(ch).or_insert(0);
        *count += 1;
    }

    let mut char_count_vec: Vec<_> = char_count.into_iter().collect();

    char_count_vec.sort_by(|a, b| {
        // First, compare by count in descending order
        let count_ordering = b.1.cmp(&a.1);

        // If counts are equal, compare alphabetically
        if count_ordering == std::cmp::Ordering::Equal {
            compare_strengths(b.0, a.0)
        } else {
            count_ordering
        }
    });

    char_count_vec
}

fn compare_strengths(a: char, b: char) -> Ordering {
    strength_score(a).cmp(&strength_score(b))
}

fn strength_score(c: char) -> i32 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
