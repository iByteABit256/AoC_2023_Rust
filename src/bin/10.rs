advent_of_code::solution!(10);
use std::io::{Error, ErrorKind};

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn starting_point(board: &Vec<Vec<char>>) -> Result<(usize, usize), Error> {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'S' {
                return Ok((i, j))
            }
        }
    }

    Err(Error::new(ErrorKind::InvalidData, "Board has no starting point"))
}

fn farthest_tile(board: &Vec<Vec<char>>) {
    let s = starting_point(&board).unwrap();

}

pub fn part_one(input: &str) -> Option<u32> {
    let board = parse(input);

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!("{}", board[i][j]);
        }
        println!("");
    }

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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
