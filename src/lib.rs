use std::fs;

use curl::easy::Easy;

pub trait DailyProblem {
    fn name(&self) -> &str;
    fn index(&self) -> u8;
    fn solutions(&self, input: &str) -> (String, String);
}

impl dyn DailyProblem {
    pub fn print_solutions(&self) {
	let _input = self.get_input();
	println!("{}", self.name());
	println!("{}", "=".repeat(self.name().len()));
	let (first_solution, second_solution) = self.solutions(&self.get_input());
	println!("Part 1: {}", first_solution);
	println!("Part 2: {}", second_solution);
    }

    fn get_input(&self) -> String {
	match fs::read_to_string(format!("data/{}", self.index())) {
	    Ok(s) => s,
	    Err(_) => self.download_and_return_input().unwrap(),
	}
    }

    fn download_and_return_input(&self) -> Result<String, ()> {
	println!("Downloading file for problem {}", self.index());

	let mut cookie_file_path = project_root::get_project_root().unwrap();
	cookie_file_path.push("src");
	cookie_file_path.push("data");
	cookie_file_path.push("cookie");
	let cookie_string = match fs::read_to_string(cookie_file_path.to_str().unwrap()) {
	    Ok(cs) => cs,
	    Err(e) => panic!("Set your cookie file at \"src/data/cookie\": {}", e)
	};

	let mut handle = Easy::new();
	handle.cookie(&cookie_string).unwrap();
	handle.url(&format!("https://adventofcode.com/2022/day/{}/input", self.index())).unwrap();

	let mut input: String = String::new();
	{
	    let mut transfer = handle.transfer();
	    transfer.write_function(|data| {
		input.push_str(std::str::from_utf8(data).unwrap());
		Ok(data.len())
	    }).unwrap();

	    transfer.perform().unwrap();
	}
	fs::write(format!("data/{}", self.index()), &input).unwrap();
	Ok(input)
    }
}

pub fn sum_lines(input: &str, f: fn(&str) -> u32) -> u32 {
    input.lines().map(f).sum()
}
