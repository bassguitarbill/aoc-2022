fn main() {
    println!("Day 1: Calorie Counting");
    println!("=======================");
    let data = include_str!("data.txt");
    let groups = group_by_elf2(data);
    let mut calories: Vec<u32> = groups.iter().map(calculate_calories).collect();
    calories.sort();
    let max_calories = calories.last();
    println!("Part 1: {}", max_calories.unwrap());
    let top_three = vec![max_calories, calories.get(calories.len() - 2), calories.get(calories.len() - 3)];
    println!("Part 2: {}", top_three.iter().fold(0, |acc, x| acc + x.unwrap()));
}

fn calculate_calories(elf: &Vec<&str>) -> u32 {
    elf.iter().fold(0, |acc, x| acc + x.parse::<u32>().unwrap())
}

fn group_by_elf2(input: &str) -> Vec<Vec<&str>> {
    input.lines().fold(vec![vec![]], |mut elves: Vec<Vec<&str>>, line| if line.is_empty() { elves.push(Vec::new()); elves } else { elves.last_mut().unwrap().push(line); elves })
}

fn _group_by_elf3(input: &str) -> Vec<Vec<&str>> {
    let mut elves = vec![vec![]];
    for line in input.lines() {
	if line.is_empty() {
	    elves.push(Vec::new());
	} else {
	    let last_elf = elves.last_mut().unwrap();
	    (*last_elf).push(line);
	}
    }
    elves
}

fn _group_by_elf(input: &str) -> Vec<Vec<&str>> {
    let mut elves = Vec::new();
    let mut current_elf = Vec::new();
    for line in input.lines() {
	if line.is_empty() {
	    elves.push(current_elf);
	    current_elf = Vec::new();
	} else {
	    current_elf.push(line);
	}
    }
    elves.push(current_elf);
    elves
}
