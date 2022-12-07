use regex::Regex;
use std::cmp::Ordering;
use std::str::FromStr;
use aoc_2022::{DailyProblem, sum_lines};
use lazy_static::lazy_static;

pub struct CampCleanup;

impl DailyProblem for CampCleanup {
    fn name(&self) -> &str { "Day 4: Camp Cleanup" }
    fn index(&self) -> &str { "04" }
    fn solutions(&self) -> (String, String) {
	let data = include_str!("data/camp_cleanup.txt");
	(
	    sum_lines(data, count_full_overlaps).to_string(),
	    sum_lines(data, count_partial_overlaps).to_string(),
	)
    }
}

fn count_full_overlaps(input: &str) -> u32 {
    match AssignmentPair::from_str(input).unwrap().is_full_overlap() { true => 1, false => 0 }
}

fn count_partial_overlaps(input: &str) -> u32 {
    match AssignmentPair::from_str(input).unwrap().is_partial_overlap() { true => 1, false => 0 }
}

#[derive(Debug, PartialEq)]
struct AssignmentPair((u32, u32), (u32, u32));

impl FromStr for AssignmentPair {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(\\d*)-(\\d*),(\\d*)-(\\d*)").unwrap();
        }
        let caps = RE.captures(input).unwrap();
	Ok(
	    Self(
		((caps.get(1).unwrap().as_str().parse().unwrap()),(caps.get(2).unwrap().as_str().parse().unwrap())),
		((caps.get(3).unwrap().as_str().parse().unwrap()),(caps.get(4).unwrap().as_str().parse().unwrap())))
	)
    }
}

impl AssignmentPair {
    fn is_full_overlap(&self) -> bool {
	match self.0.0.cmp(&self.1.0) {
	    Ordering::Greater => self.1.1 >= self.0.1,
	    Ordering::Less => self.0.1 >= self.1.1,
	    Ordering::Equal => true
	}
    }

    fn is_partial_overlap(&self) -> bool {
	match self.0.0.cmp(&self.1.0) {
	    Ordering::Greater => self.1.1 >= self.0.0,
	    Ordering::Less => self.0.1 >= self.1.0,
	    Ordering::Equal => true
	}
    }
}

#[cfg(test)]
mod tests {
    use crate::camp_cleanup::AssignmentPair;
    use std::str::FromStr;
    
    #[test]
    fn from_str() {
	assert_eq!(AssignmentPair::from_str("2-4,6-8").unwrap(), AssignmentPair((2, 4), (6, 8))); 
    }
}
