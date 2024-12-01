pub trait Problem<T> {
    fn parse_input(&self, filename: &str) -> T;
    fn part_one(&self, input: T) -> String;
    fn part_two(&self, input: T) -> String;
}