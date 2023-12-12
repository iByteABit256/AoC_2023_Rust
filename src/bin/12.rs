advent_of_code::solution!(12);

struct SpringLine {
    springs: String,
    groups: Vec<u32>
}

fn parse(input: &str) -> Vec<SpringLine> {
    let mut spring_lines = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        let springs = parts.next().unwrap();
        let groups_str = parts.next().unwrap();

        let groups = groups_str.split(',').map(|c| c.parse::<u32>().unwrap()).collect();

        spring_lines.push( SpringLine { springs: springs.to_owned(), groups } )
    }

    spring_lines
}

fn is_broken(spring: char) -> bool {
    spring == '#'
}

fn is_operational(spring: char) -> bool {
    spring == '.'
}

fn is_valid(fixed_springs: SpringLine) -> bool {
    let mut actual_groups = Vec::new();
    let mut curr_count = 0;

    for char in fixed_springs.springs.chars() {
        match char {
            '.' => {
                if curr_count != 0 {
                    actual_groups.push(curr_count);
                }

                curr_count = 0;
                continue;
            },
            '#' => {
                curr_count += 1;
            },
            _ => {
                println!("{} is not a valid character in spring line: {}", char, fixed_springs.springs);
                return false;
            }
        }
    }

    actual_groups.len() == fixed_springs.groups.len()
        &&
    actual_groups.iter().zip(fixed_springs.groups).all(|(a, b)| *a == b)
}

fn combinations(spring_line: SpringLine) -> u32 {
    let total_broken: u32 = spring_line.groups.iter().sum();
    let broken = spring_line.springs.chars().filter(|&c| is_broken(c)).count() as u32;
    let diff = total_broken - broken;
    let springs = spring_line.springs.as_bytes();

    let positions: Vec<u32> = Vec::with_capacity(diff as usize);

    fn place_next(springs: &[u8], start_idx: usize) {
        for i in start_idx + 1..springs.len() {
            if is_operational(springs[i] as char) {

            }
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
