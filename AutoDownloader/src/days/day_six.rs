use crate::problem::Problem;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
pub struct DayFive {
    day: i32
}

impl Problem<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>)> for DayFive {
    fn parse_input(&self, filename: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
        let file_path = format!("/Users/mikescriven/Code/AdventOfCode/scrivens_do_adventofcode/AutoDownloader/day{}/{}", self.day, filename);

        if !!!Path::new(&file_path).exists() {
            println!("Day 5 Part One, file doesn't exist: {}", file_path);
        }

        let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut vector: Vec<Vec<i32>> = Vec::new();

        for line in read_to_string(file_path).unwrap().lines()
        {
            if line.contains("|")
            {
                let mut parsed_string: Vec<i32> = line.split("|").map(|c| c.parse::<i32>().unwrap()).collect();
                parsed_string.reverse();
                if hash_map.contains_key(&parsed_string[0]) {
                    hash_map.get_mut(&parsed_string[0]).unwrap().push(parsed_string[1]);
                } else {
                    let mut vec: Vec<i32> = Vec::new();
                    vec.push(parsed_string[1]);
                    hash_map.insert(parsed_string[0], vec);
                }
            }
            else {
                if line.is_empty() {continue;}
                let parsed_string: Vec<i32> = line.split(",").map(|c| c.parse::<i32>().unwrap()).collect();
                vector.push(parsed_string);
            }
        }

        return (hash_map, vector)

    }

    fn part_one(&self, input: (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>)) -> String {
        String::new()
    }

    fn part_two(&self, input: (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>)) -> String {
        let pages_list = input.1;
        let rules = input.0;

        let mut sum = 0;

        for pages in pages_list {
            let mut sorted_pages = pages.clone();
            let mut needs_sorting = true;
            let mut ever_sorted = false;

            while needs_sorting {
                let (sorted, new_pages) = sort_page(&rules, sorted_pages);
                sorted_pages = new_pages;
                needs_sorting = sorted;
                if needs_sorting {
                    println!("p {:?}", pages);
                    println!("s {:?}", sorted_pages);
                    ever_sorted = true;
                }
            }

            if (ever_sorted) {
                let middle_num = sorted_pages[sorted_pages.len() / 2];
                sum += middle_num;
            }
        }

        sum.to_string()
    }
}

fn sort_page(rules: &HashMap<i32, Vec<i32>>, pages: Vec<i32>) -> (bool, Vec<i32>) {
    for i in 0..pages.len() {
        let page = &pages[i];

        if rules.contains_key(page) {
            let matching_rules = rules.get(page).unwrap();
            for j in i..pages.len() { // Check this isn't overlapping somehow later
                if matching_rules.contains(&pages[j]) {
                    println!("matching_rules {:?}", matching_rules);
                    let mut new_vec = pages.clone();
                    // move wrong value
                    move_element(&mut new_vec, j, i);
                    return (true, new_vec)
                }
            }
        }
        // find something with a rule entry
        // Search the rest of the string
        // Move the first match before the element
        // start again

    }

    return (false, pages)
}

fn move_element(vec: &mut Vec<i32>, from: usize, to: usize) {
    // Check if the indices are valid
    if from >= vec.len() || to >= vec.len() {
        println!("Invalid indices");
        return;
    }

    // Remove the element from the source index
    let element = vec.remove(from);

    // Insert the element at the destination index
    vec.insert(to, element);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let filename: &str = "demo";
        let day1 = DayFive { day: 5 };
        let output: (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) = day1.parse_input(filename);
        assert_eq!(output.0[&47], Vec::from([53, 13, 61, 29]));
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
        let day1 = DayFive { day: 5 };

        let input = day1.parse_input(filename);
        let result = day1.part_two(input);
        // println!("Result of part one: {}", result);
        assert_eq!(result, "123");
    }

    #[test]
    fn test_part_one_real_mike() {
        let filename: &str = "input5";
        let day1 = DayFive { day: 5 };

        let input = day1.parse_input(filename);
        let result = day1.part_two(input);

        assert_eq!(result, "blah");
    }

    #[test]
    fn test_part_one_real_jo() {
        let filename: &str = "input3jo";
        let day1 = DayFive { day: 3 };

        let input = day1.parse_input(filename);
        let result = day1.part_two(input);

        assert_eq!(result, "blah");
    }

    #[test]
    fn test_part_two_demo() {
        let filename: &str = "demo2";
        let day1 = DayFive { day: 3 };

        let input = day1.parse_input(filename);
        let result = day1.part_two(input);

        assert_eq!(result, "48");
    }
}