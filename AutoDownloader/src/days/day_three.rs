use crate::problem::Problem;
use std::fs::read_to_string;
use std::path::Path;
use dict::{Dict, DictIface};
use regex::Regex;
pub struct DayThree {
    day: i32
}

impl Problem<String> for DayThree {
    fn parse_input(&self, filename: &str) -> String {
        let file_path = format!("/Users/mikescriven/Code/AdventOfCode/scrivens_do_adventofcode/AutoDownloader/day{}/{}", self.day, filename);

        if !!!Path::new(&file_path).exists() {
            println!("Day 1 Part One, file doesn't exist: {}", file_path);
        }


        // return read_to_string(file_path).unwrap().lines();
        return read_to_string(file_path).unwrap();
    }

    fn part_one(&self, input: String) -> String {
        let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();

        println!("got here");
        // let mut results = Vec::new();
        let results : Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();

        // for (_, [path, lineno, line]) in re.captures_iter(&input).map(|c| c.extract()) {
        //     results.push(line);
        //     println!("got here");
        // }

        println!("{:?}", results);

        let mut sum = 0;
        let re2 = Regex::new(r"[0-9]+").unwrap();
        for result in results {
            println!("{:?}", result);
            let nums : Vec<&str> = re2.find_iter(result).map(|m| m.as_str()).collect();
            // let mut nums = Vec::new();
            // for (_, [path, lineno, line]) in re2.captures_iter(&result).map(|c| c.extract()) {
            //     nums.push(line.parse::<i32>().unwrap());
            // }
            println!("{:?}", nums);
            sum += &nums[0].parse::<i32>().unwrap() * &nums[1].parse::<i32>().unwrap();

        }


        sum.to_string()
    }

    fn part_two(&self, input: String) -> String {

        let re = Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").unwrap();
        let results : Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
        println!("{:?}", results);

        let mut sum = 0;
        let mut on = true;
        for result in results {
            if (result == "do()"){
                on = true;
                continue;
            }
            if (result == "don't()"){
                on = false;
                continue;
            }
            if (!on) {
                continue;
            }

            let re2 = Regex::new(r"[0-9]+").unwrap();
            let nums : Vec<&str> = re2.find_iter(result).map(|m| m.as_str()).collect();
            sum += &nums[0].parse::<i32>().unwrap() * &nums[1].parse::<i32>().unwrap();
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let filename: &str = "demo";
        let day1 = DayThree { day: 3 };
        let output: (String) = day1.parse_input(filename);
        assert_eq!(output, String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"));
        // assert_eq!(output.1, Vec::from([4, 3, 5, 3, 9, 3]));
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
    fn test_part_one_demo() {
        let filename: &str = "demo";
        let day1 = DayThree { day: 3 };

        let input = day1.parse_input(filename);
        let result = day1.part_one(input);
        // println!("Result of part one: {}", result);
        // assert_eq!("161", "161");
    }

    #[test]
    fn test_part_one_real_mike() {
        let filename: &str = "input3";
        let day1 = DayThree { day: 3 };

        let input = day1.parse_input(filename);
        let result = day1.part_two(input);

        assert_eq!(result, "blah");
    }

    #[test]
    fn test_part_one_real_jo() {
        let filename: &str = "input3jo";
        let day1 = DayThree { day: 3 };

        let input = day1.parse_input(filename);
        let result = day1.part_two(input);

        assert_eq!(result, "blah");
    }

    #[test]
    fn test_part_two_demo() {
        let filename: &str = "demo2";
        let day1 = DayThree { day: 3 };

        let input = day1.parse_input(filename);
        let result = day1.part_two(input);

        assert_eq!(result, "48");
    }
}