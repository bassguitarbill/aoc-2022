use std::str::FromStr;

fn main() {
    println!("Day 2: Rock Paper Scissors");
    println!("==========================");
    let data = include_str!("data.txt");

    println!("Part 1: {}", sum_lines(data, score_round_part_1));
    println!("Part 2: {}", sum_lines(data, score_round_part_2));
}

fn sum_lines(input: &str, f: fn(&str) -> u32) -> u32 {
    input.lines().map(f).sum()
}

fn score_round_part_1(input: &str) -> u32 {
    let mut signs = input.split(' ');
    let their_sign = Sign::from_str(signs.next().unwrap()).unwrap();
    let my_sign = Sign::from_str(signs.next().unwrap()).unwrap();
    let result = my_sign.result(their_sign);

    my_sign.score() + result.score()
}

fn score_round_part_2(input: &str) -> u32 {
    let mut signs = input.split(' ');
    let their_sign = Sign::from_str(signs.next().unwrap()).unwrap();
    let result = RoundResult::from_str(signs.next().unwrap()).unwrap();
    let my_sign = result.sign_to_throw(their_sign);

    my_sign.score() + result.score()
}

#[derive(PartialEq)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl Sign {
    fn score(&self) -> u32 {
	match self {
	    Sign::Rock => 1,
	    Sign::Paper => 2,
	    Sign::Scissors => 3,
	}
    }

    fn beats(&self) -> Sign {
	match self {
	    Sign::Rock => Sign::Scissors,
	    Sign::Paper => Sign::Rock,
	    Sign::Scissors => Sign::Paper,
	}
    }

    fn is_beaten_by(&self) -> Sign {
	match self {
	    Sign::Rock => Sign::Paper,
	    Sign::Paper => Sign::Scissors,
	    Sign::Scissors => Sign::Rock,
	}
    }

    fn result(&self, other: Self) -> RoundResult {
	if self.beats() == other { RoundResult::Victory }
	else if self.is_beaten_by() == other { RoundResult::Loss }
	else { RoundResult::Tie }
    }
}

enum RoundResult {
    Victory,
    Loss,
    Tie,
}

impl RoundResult {
    fn score(&self) -> u32 {
	match self {
	    RoundResult::Victory => 6,
	    RoundResult::Tie => 3,
	    RoundResult::Loss => 0,
	}
    }

    fn sign_to_throw(&self, opponent_sign: Sign) -> Sign {
	match self {
	    RoundResult::Victory => opponent_sign.is_beaten_by(),
	    RoundResult::Tie => opponent_sign,
	    RoundResult::Loss => opponent_sign.beats(),
	}
    }
}

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, ()> {
	match input {
	    "X" => Ok(RoundResult::Loss),
	    "Y" => Ok(RoundResult::Tie),
	    "Z" => Ok(RoundResult::Victory),
	    _ => Err(()),
	}
    }
}


impl FromStr for Sign {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
	match input {
	    "A" => Ok(Sign::Rock),
	    "X" => Ok(Sign::Rock),
	    "B" => Ok(Sign::Paper),
	    "Y" => Ok(Sign::Paper),
	    "C" => Ok(Sign::Scissors),
	    "Z" => Ok(Sign::Scissors),
	    _ => Err(()),
	}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn score_round_part_1() {
	assert_eq!(super::score_round_part_1("A Y"), 8);
	assert_eq!(super::score_round_part_1("B X"), 1);
	assert_eq!(super::score_round_part_1("C Z"), 6);
    }

    #[test]
    fn score_round_part_2() {
	assert_eq!(super::score_round_part_2("A Y"), 4);
	assert_eq!(super::score_round_part_2("B X"), 1);
	assert_eq!(super::score_round_part_2("C Z"), 7);
    }
}
