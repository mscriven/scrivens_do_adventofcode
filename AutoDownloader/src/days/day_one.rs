use crate::problem::Problem;
use std::fs::read_to_string;

pub struct DayOne {}

impl DayOne {
    fn read_lines(filename: &str) -> Vec<String> {
        let mut result = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }

        result
    }
}

impl Problem for DayOne {
    fn parse_input(&self, input: &str) -> String {
        return String::new()
    }

    fn part_one(&self, input: &str) -> String {
        //solution: String = "";
        //println!("Day 1 Part 2 {}", solution);
        return String::new()
    }

    fn part_two(&self, input: &str) -> String {
        //solution: String = "";
        //println!("Day 1 Part 2 {}", solution);
        return String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let filename: &str = "/Users/mikescriven/Code/AdventOfCode/scrivens_do_adventofcode/AutoDownloader/day1/input1";
        let output: Vec<String> = DayOne::read_lines(filename);
        println!("{:#?}", output);
    }
    
    #[test]
    fn test_part_one() {
        let input = "";
        assert_eq!(DayOne {}.part_one(input), "");
    }
}