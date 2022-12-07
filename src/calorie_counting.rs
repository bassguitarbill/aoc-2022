use aoc_2022::DailyProblem;

pub struct CalorieCounting;

impl DailyProblem for CalorieCounting {
    fn name(&self) -> &str { "Day 1: Calorie Counting" }
    fn index(&self) -> &str { "01" }
    fn solutions(&self) -> (String, String) {
	let mut calories = group_by_elf(include_str!("data/calorie_counting.txt"));
	calories.sort();
	let max_calories = calories.last();
	let top_three = &calories[calories.len() - 3..calories.len()];
	(max_calories.unwrap().to_string(), top_three.iter().sum::<u32>().to_string())
    }
}

pub fn group_by_elf(input: &str) -> Vec<u32> {
    input.split("\n\n").map(|elf| elf.lines().map(|l| l.parse::<u32>().unwrap()).sum()).collect()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_group_by_elf() {
	let input =
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
	let calories = vec![6000, 4000, 11000, 24000, 10000];
	assert_eq!(super::group_by_elf(input), calories);
    }
}
