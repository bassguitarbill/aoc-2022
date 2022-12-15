use aoc_2022::{sum_lines, DailyProblem};
use std::iter::Iterator;
use std::str::{FromStr, Lines};

pub struct RucksackReorganization;

impl DailyProblem for RucksackReorganization {
    fn name(&self) -> &str {
        "Day 3: Rucksack Reorganization"
    }
    fn index(&self) -> u8 {
        3
    }
    fn solutions(&self, input: &str) -> (String, String) {
        (
            sum_lines(input, calculate_priority).to_string(),
            ElfGroupIter(input.lines())
                .map(|eg| eg.priority().unwrap())
                .sum::<u32>()
                .to_string(),
        )
    }
}

fn calculate_priority(line: &str) -> u32 {
    Rucksack::from_str(line).unwrap().priority().unwrap()
}

struct ElfGroupIter<'a>(Lines<'a>);

impl Iterator for ElfGroupIter<'_> {
    type Item = ElfGroup;

    fn next(&mut self) -> Option<ElfGroup> {
        match self.0.next() {
            Some(a) => match self.0.next() {
                Some(b) => match self.0.next() {
                    Some(c) => Some(ElfGroup(a.to_string(), b.to_string(), c.to_string())),
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
}

fn common_chars(s1: &str, s2: &str) -> String {
    let mut chars = vec![];
    for c in s1.chars() {
        if s2.contains(&(c.to_string())) {
            chars.push(c);
        }
    }
    chars.iter().collect()
}

struct ElfGroup(String, String, String);

impl ElfGroup {
    fn badge_letter(&self) -> Option<char> {
        let badge_candidates = common_chars(&common_chars(&self.0, &self.1), &self.2);
        match badge_candidates.len() {
            0 => None,
            _ => Some(badge_candidates.chars().next().unwrap()),
        }
    }
    fn priority(&self) -> Result<u32, String> {
        match self.badge_letter() {
            Some(letter) => letter_priority(letter),
            None => Err(format!(
                "Common letter not found in {}, {}, and {}",
                self.0, self.1, self.2
            )),
        }
    }
}

struct Rucksack(String, String, String);

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, ()> {
        match input.len() % 2 == 1 {
            true => Err(()),
            false => Ok(Self(
                input.to_string(),
                input[..(input.len() / 2)].to_string(),
                input[(input.len() / 2)..].to_string(),
            )),
        }
    }
}

fn letter_priority(input_char: char) -> Result<u32, String> {
    let input = input_char as u32;
    if input > 64 && input < 91 {
        Ok(input - (65 - 27))
    } else if input > 96 && input < 123 {
        Ok(input - 96)
    } else {
        Err(format!(
            "No priority for input char {}({})",
            input_char, input
        ))
    }
}

impl Rucksack {
    fn common_letter_between_pockets(&self) -> Option<char> {
        self.1.chars().find(|c| self.2.contains(&(c.to_string())))
    }

    fn priority(&self) -> Result<u32, String> {
        match self.common_letter_between_pockets() {
            Some(letter) => letter_priority(letter),
            None => Err(format!(
                "Common letter not found in {} and {}",
                self.0, self.1
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    #[test]
    fn common_letter_between_pockets() {
        assert_eq!(
            super::Rucksack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp")
                .unwrap()
                .common_letter_between_pockets()
                .unwrap(),
            'p'
        );
        assert_eq!(
            super::Rucksack::from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")
                .unwrap()
                .common_letter_between_pockets()
                .unwrap(),
            'L'
        );
        assert_eq!(
            super::Rucksack::from_str("PmmdzqPrVvPwwTWBwg")
                .unwrap()
                .common_letter_between_pockets()
                .unwrap(),
            'P'
        );
        assert_eq!(
            super::Rucksack::from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")
                .unwrap()
                .common_letter_between_pockets()
                .unwrap(),
            'v'
        );
        assert_eq!(
            super::Rucksack::from_str("ttgJtRGJQctTZtZT")
                .unwrap()
                .common_letter_between_pockets()
                .unwrap(),
            't'
        );
        assert_eq!(
            super::Rucksack::from_str("CrZsJsPPZsGzwwsLwLmpwMDw")
                .unwrap()
                .common_letter_between_pockets()
                .unwrap(),
            's'
        );
    }

    #[test]
    fn letter_priority() {
        assert_eq!(super::letter_priority('p').unwrap(), 16);
        assert_eq!(super::letter_priority('L').unwrap(), 38);
        assert_eq!(super::letter_priority('P').unwrap(), 42);
        assert_eq!(super::letter_priority('v').unwrap(), 22);
        assert_eq!(super::letter_priority('t').unwrap(), 20);
        assert_eq!(super::letter_priority('s').unwrap(), 19);
    }

    #[test]
    fn badge_letter() {
        assert_eq!(
            super::ElfGroup(
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                "PmmdzqPrVvPwwTWBwg".to_string()
            )
            .badge_letter()
            .unwrap(),
            'r'
        );
        assert_eq!(
            super::ElfGroup(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                "ttgJtRGJQctTZtZT".to_string(),
                "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
            )
            .badge_letter()
            .unwrap(),
            'Z'
        );
    }
}
