fn main() {
    println!("Day 1: Calorie Counting");
    println!("=======================");
    let mut calories = group_by_elf(include_str!("data.txt"));
    calories.sort();
    let max_calories = calories.last();
    println!("Part 1: {}", max_calories.unwrap());
    let top_three = &calories[calories.len() - 3..calories.len()];
    println!("Part 2: {}", top_three.iter().sum::<u32>());
}

fn group_by_elf(input: &str) -> Vec<u32> {
    input.split("\n\n").map(|elf| elf.lines().fold(0, |calories, snack| calories + snack.parse::<u32>().unwrap())).collect()
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
