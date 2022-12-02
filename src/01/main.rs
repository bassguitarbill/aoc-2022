fn main() {
    println!("Day 1: Calorie Counting");
    println!("=======================");
    let mut calories = group_by_elf(include_str!("data.txt"));
    calories.sort();
    let max_calories = calories.last();
    println!("Part 1: {}", max_calories.unwrap());
    let top_three = &calories[calories.len() - 3..calories.len()];
    println!("Part 2: {}", top_three.iter().fold(0, |acc, x| acc + x));
}

fn group_by_elf(input: &str) -> Vec<u32> {
    input.split("\n\n").map(|elf| elf.lines().fold(0, |calories, snack| calories + snack.parse::<u32>().unwrap())).collect()
}
