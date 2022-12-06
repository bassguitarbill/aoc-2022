use std::collections::HashSet;

use aoc_2022::DailyProblem;

pub struct TuningTrouble;

impl DailyProblem for TuningTrouble {
    fn name(&self) -> &str { "Day 6: Tuning Trouble" }
    fn index(&self) -> &str { "06" }
    fn solutions(&self) -> (String, String) {
	let data = include_str!("tuning_trouble/data.txt");
	(
	    find_start_of_packet_index(data, 4).unwrap().to_string(),
	    find_start_of_packet_index(data, 14).unwrap().to_string()
	)
    }
}

fn find_start_of_packet_index(input: &str, length: usize) -> Option<usize> {
    let mut index = length;
    while index < input.len() {
	if is_start_of_packet(&input[(index - length)..index]) {
	    return Some(index);
	}
	index += 1;
    }
    None
}

pub fn is_start_of_packet(input: &str) -> bool {
    let mut set = HashSet::new();
    input.chars().for_each(|c| { set.insert(c);});
    set.len() == input.len()
}

#[cfg(test)]
mod tests {
    use crate::tuning_trouble::*;
    #[test]
    fn is_start_of_packet_test() {
	assert!(!is_start_of_packet("mjqj"));
	assert!(!is_start_of_packet("jqjp"));
	assert!(!is_start_of_packet("qjpq"));
	assert!(is_start_of_packet("jpqm"));
    }

    #[test]
    fn find_start_of_packet_index_test() {
	assert_eq!(find_start_of_packet_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4).unwrap(), 7);
	assert_eq!(find_start_of_packet_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(), 5);
	assert_eq!(find_start_of_packet_index("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(), 6);
	assert_eq!(find_start_of_packet_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(), 10);
	assert_eq!(find_start_of_packet_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(), 11);

	assert_eq!(find_start_of_packet_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(), 19);
	assert_eq!(find_start_of_packet_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(), 23);
	assert_eq!(find_start_of_packet_index("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(), 23);
	assert_eq!(find_start_of_packet_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(), 29);
	assert_eq!(find_start_of_packet_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(), 26);
    }
}
