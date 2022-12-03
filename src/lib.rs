pub trait DailyProblem {
    fn name(&self) -> &str;
    fn index(&self) -> &str;
    fn solutions(&self) -> (String, String);
}

impl dyn DailyProblem {
    pub fn print_solutions(&self) {
	println!("{}", self.name());
	println!("{}", "=".repeat(self.name().len()));
	let (first_solution, second_solution) = self.solutions();
	println!("Part 1: {}", first_solution);
	println!("Part 2: {}", second_solution);
    }
}

pub fn sum_lines(input: &str, f: fn(&str) -> u32) -> u32 {
    input.lines().map(f).sum()
}
