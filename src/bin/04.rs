advent_of_code::solution!(4);
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut won = 0;

        let all_numbers: Vec<_> = line.split(": ").collect();
        let all_numbers = all_numbers.last().expect("Wrong input");

        let parts: Vec<_> = all_numbers.split('|').collect();
        let (winning_num_str, owned_num_str) = match parts.as_slice() {
            [first, second] => (first.trim(), second.trim()),
            _ => ("", ""), // Provide default values or handle the case where there are not exactly two parts
        };

        let winning_numbers = parse_winning_numbers(winning_num_str);
        let owned_numbers = parse_owned_numbers(owned_num_str);

        for num in owned_numbers {
            if winning_numbers.contains(&num) {
                won = if won == 0 { 1 } else { won << 1 }
            }
        }

        sum += won;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input.lines().count();
    let mut scratchcards: HashMap<usize, u32> = HashMap::with_capacity(count);

    for i in 0..count {
        scratchcards.insert(i, 1);
    }

    for (line_num, line) in input.lines().enumerate() {
        let all_numbers: Vec<_> = line.split(": ").collect();
        let all_numbers = all_numbers.last().expect("Wrong input");

        let parts: Vec<_> = all_numbers.split('|').collect();
        let (winning_num_str, owned_num_str) = match parts.as_slice() {
            [first, second] => (first.trim(), second.trim()),
            _ => ("", ""), // Provide default values or handle the case where there are not exactly two parts
        };

        let winning_numbers = parse_winning_numbers(winning_num_str);
        let owned_numbers = parse_owned_numbers(owned_num_str);

        let won_count = owned_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();

        for i in line_num + 1..cmp::min(line_num + won_count + 1, count) {
            *scratchcards.get_mut(&i).unwrap() += scratchcards[&line_num];
        }
    }

    Some(scratchcards.values().sum())
}

fn parse_winning_numbers(num_str: &str) -> HashSet<i32> {
    HashSet::from_iter(
        num_str
            .split_whitespace()
            .map(|x| x.trim().parse().expect("Not an integer!")),
    )
}

fn parse_owned_numbers(num_str: &str) -> Vec<i32> {
    num_str
        .split_whitespace()
        .map(|x| x.trim().parse().expect("Not an integer!"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23847));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8570000));
    }
}
