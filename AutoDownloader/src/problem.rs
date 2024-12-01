pub trait Problem {
    fn parse_input(&self, input: &str) -> String;
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}