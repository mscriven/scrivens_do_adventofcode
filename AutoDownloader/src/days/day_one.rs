use crate::problem::Problem;
use std::fs::read_to_string;
use std::path::Path;
use dict::{Dict, DictIface};
pub struct DayOne {
    day: i32
}

impl DayOne {
}

impl Problem<(Vec<i32>, Vec<i32>)> for DayOne {
    fn parse_input(&self, filename: &str) -> (Vec<i32>, Vec<i32>) {
        let file_path = format!("/Users/mikescriven/Code/AdventOfCode/scrivens_do_adventofcode/AutoDownloader/day{}/{}", self.day, filename);

        let mut left_side = Vec::new();
        let mut right_side = Vec::new();

        if !!!Path::new(&file_path).exists() {
            println!("Day 1 Part One, file doesn't exist: {}", file_path);
        }

        for line in read_to_string(file_path).unwrap().lines() {
            let split_line : Vec<&str> = line.split("   ").collect();
            left_side.push(split_line[0].parse().unwrap());
            right_side.push(split_line[1].parse().unwrap());
        }

        (left_side, right_side)
    }

    fn part_one(&self, input: (Vec<i32>, Vec<i32>)) -> String {

        let (mut left, mut right) = input;
        left.sort();
        right.sort();
        let mut i = 0;
        let mut sum = 0;
        while i < left.len() && i < right.len() {
            sum += (left[i] - right[i]).abs();
            i += 1;
        }
        println!("Day 1 Part One: {}", sum);
        sum.to_string()
    }

    fn part_two(&self, input: (Vec<i32>, Vec<i32>)) -> String {
        let (mut left, mut right) = input;
        left.sort();
        right.sort();

        let mut i = 0;
        let mut dict = Dict::<i32>::new();
        while i < left.len() {
            dict.add(left[i].to_string(), 0);
            i += 1;
        }

        i = 0;
        while i < right.len() {
            let key = right[i].to_string();
            if dict.contains_key(&key) {
                let current = *dict.get(&key).unwrap() + 1;
                dict.remove_key(&key);
                dict.add(key, current);
            }
            i += 1;
        }

        let mut sum = 0;
        for o in &left {
            let num = *o * dict.get(&o.to_string()).unwrap();
            sum += num;
        }

        println!("Day 1 Part Two: {}", sum);
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let filename: &str = "demo1";
        let day1 = DayOne { day: 1 };
        let output: (Vec<i32>, Vec<i32>) = day1.parse_input(filename);
        assert_eq!(output.0, Vec::from([3, 4, 2, 1, 3, 3]));
        assert_eq!(output.1, Vec::from([4, 3, 5, 3, 9, 3]));
    }

    #[test]
    fn test_sort_list() {
        let mut left = Vec::from([3, 4, 2, 1, 3, 3]);
        let mut right = Vec::from([4, 3, 5, 3, 9, 3]);
        left.sort();
        right.sort();
        assert_eq!(left, Vec::from([1, 2, 3, 3, 3, 4]));
        assert_eq!(right, Vec::from([3, 3, 3, 4, 5, 9]));
    }

    #[test]
    fn test_part_one() {
        let filename: &str = "demo1";
        let day1 = DayOne { day: 1 };

        let input = day1.parse_input(filename);
        let result = day1.part_one(input);

        assert_eq!(result, "11");
    }

    #[test]
    fn test_part_one_real() {
        let filename: &str = "input1";
        let day1 = DayOne { day: 1 };

        let input = day1.parse_input(filename);
        let result = day1.part_one(input);

        assert_eq!(result, "blah");
    }

    #[test]
    fn test_part_two_demo() {
        let filename: &str = "demo1";
        let day1 = DayOne { day: 1 };

        let input = day1.parse_input(filename);
        let result = day1.part_two(input);

        assert_eq!(result, "31");
    }
}