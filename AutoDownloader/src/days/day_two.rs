use crate::problem::Problem;
use std::fs::read_to_string;
use std::path::Path;
use dict::{Dict, DictIface};
pub struct DayTwo {
    day: i32
}

impl Problem<(Vec<Vec<i32>>)> for DayTwo {
    fn parse_input(&self, filename: &str) -> (Vec<Vec<i32>>) {
        let file_path = format!("/Users/mikescriven/Code/AdventOfCode/scrivens_do_adventofcode/AutoDownloader/day{}/{}", self.day, filename);

        if !!!Path::new(&file_path).exists() {
            println!("Day 1 Part One, file doesn't exist: {}", file_path);
        }

        let mut reports = Vec::new();

        for line in read_to_string(file_path).unwrap().lines() {
            let mut report = Vec::new();
            let splits = line.split(" ").collect::<Vec<&str>>();
            println!("{:?}", splits);
            let split_line: Vec<&str> = line.split(" ").collect();
            println!("{:?}", split_line);
            for item in split_line {
                report.push(item.parse().unwrap());
            }
            reports.push(report);
        }

        (reports)
    }

    fn part_one(&self, input: (Vec<Vec<i32>>)) -> String {
        let mut sum = 0;
        for report in input {
            let mut index = 0;
            while index < report.len() {
                let mut copy = report.clone();
                copy.remove(index);
                let passed = check_report(copy);
                if passed {
                    sum += 1;
                    break;
                }
                index += 1;
            }
        }

        sum.to_string()
    }

    fn part_two(&self, input: (Vec<Vec<i32>>)) -> String {
        String::from("empty question two")
    }
}

fn check_report(report: Vec<i32>) -> bool
{
    println!("START {:?}", report);
    let mut copy = report.clone();
    copy.sort();

    let mut passed = 0;
    if (report.eq(&copy)) {
        passed += 1;
    }

    if ((passed == 0)) {
        copy.reverse();
        if (report.eq(&copy)) {
            passed += 1;
        }
    }

    if (passed == 0) {
        println!("Failed order checks");
        return false;
    }

    let mut index = 0;
    let mut count = 0;
    while index < (report.len() - 1) {
        let num = report[index];
        let next_num = report[index + 1];
        let bigger_than_zero = (next_num - num).abs() > 0;
        let smaller_than = (next_num - num).abs() < 4;

        if (bigger_than_zero) {
            if (smaller_than) {
                count += 1;
            } else {
                println!("9");
                return false;
            }
        } else {
            println!("9");
            return false;
        }


        index += 1;
    }

    if (count == report.len() - 1) {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let filename: &str = "demo";
        let day1 = DayTwo { day: 2 };
        let output: (Vec<Vec<i32>>) = day1.parse_input(filename);
        assert_eq!(output[0], Vec::from([7, 6, 4, 2, 1]));
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
    fn test_part_one() {
        let filename: &str = "demo";
        let day1 = DayTwo { day: 2 };

        let input = day1.parse_input(filename);
        let result = day1.part_one(input);

        assert_eq!(result, "4");
    }

    #[test]
    fn test_part_one_real_mike() {
        let filename: &str = "input2";
        let day1 = DayTwo { day: 2 };

        let input = day1.parse_input(filename);
        let result = day1.part_one(input);

        assert_eq!(result, "blah");
    }

    #[test]
    fn test_part_one_real_jo() {
        let filename: &str = "input2jo";
        let day1 = DayTwo { day: 2 };

        let input = day1.parse_input(filename);
        let result = day1.part_one(input);

        assert_eq!(result, "blah");
    }

    #[test]
    fn test_part_two_demo() {
        let filename: &str = "demo";
        let day1 = DayTwo { day: 1 };

        let input = day1.parse_input(filename);
        let result = day1.part_two(input);

        assert_eq!(result, "31");
    }
}