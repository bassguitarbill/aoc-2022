use regex::Regex;
use aoc_2022::DailyProblem;
use lazy_static::lazy_static;
use std::{str::Lines, collections::VecDeque};

pub struct SupplyStacks;
impl DailyProblem for SupplyStacks {
    fn name(&self) -> &str { "Day 5: Supply Stacks" }
    fn index(&self) -> &str { "05" }
    fn solutions(&self) -> (String, String) {
	let data = include_str!("supply_stacks/data.txt");
	let mut part_1 = parse_input(data, false);
	while !part_1.step() {}
	let mut part_2 = parse_input(data, true);
	while !part_2.step() {}
	(part_1.crates_on_top(), part_2.crates_on_top())
    }
}

#[derive(Debug)]
struct PuzzleState<'a> {
    crates: Vec<VecDeque<char>>,
    instructions: InstructionIter<'a>,
    can_move_multiple_crates: bool,
}

impl PuzzleState<'_> {
    fn step(&mut self) -> bool {
	match self.instructions.next() {
	    Some(instruction) => { self.apply_instruction(instruction); false },
	    None => true,
	}
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
	match self.can_move_multiple_crates {
	    false => {
		for _ in 0..instruction.to_move {
		    let crates = &mut self.crates;
		    let c = crates.get_mut(instruction.from - 1).unwrap().pop_front().unwrap();
		    crates.get_mut(instruction.to - 1).unwrap().push_front(c);
		}
	    },
	    true => {
		let mut crates_in_crane = vec![];
		for _ in 0..instruction.to_move {
		    let c = self.crates.get_mut(instruction.from - 1).unwrap().pop_front().unwrap();
		    crates_in_crane.push(c);
		}

		for _ in 0..instruction.to_move {
		    let c = crates_in_crane.pop().unwrap();
		    self.crates.get_mut(instruction.to - 1).unwrap().push_front(c);
		}
	    }
	}
    }

    fn crates_on_top(&self) -> String {
	let mut string = "".to_string();
	for c in &self.crates {
	    string.push(*c.iter().next().unwrap());
	}
	string
    }
}

#[derive(Debug)]
struct Instruction {
    to_move: u32,
    from: usize,
    to: usize,
}

fn parse_input(input: &str, can_move_multiple_crates: bool) -> PuzzleState {
    let mut chunks = input.split("\n\n");
    PuzzleState{
	crates: parse_crates(chunks.next().unwrap()),
	instructions: InstructionIter{ lines: chunks.next().unwrap().lines() },
	can_move_multiple_crates,
    }
}

fn parse_crates(input: &str) -> Vec<VecDeque<char>> {
    let num_stacks = input.lines().last().unwrap().replace(' ', "").len();
    let mut stacks = vec![VecDeque::new() ; num_stacks];
    for slice in input.lines() {
	if !slice.trim().starts_with('[') { break }

	for stack_index in 0..num_stacks {
	    let mut potential_crate = slice[stack_index*4..stack_index*4+3].chars();
	    if potential_crate.next().unwrap() == '[' {
		stacks.get_mut(stack_index).unwrap().push_back(potential_crate.next().unwrap());
	    }
	}
    }
    stacks
}

#[derive(Debug)]
struct InstructionIter<'a> {
    lines: Lines<'a>,
}

impl <'a> Iterator for InstructionIter<'a> {
    type Item = Instruction;
    fn next(&mut self) -> Option<Instruction> {
	lazy_static!{
	    static ref RE: Regex = Regex::new("move (\\d*) from (\\d*) to (\\d*)").unwrap();
	}
	match self.lines.next() {
	    Some(line) => {
		let caps = RE.captures(line).unwrap();
		Some(Instruction{
		    to_move: caps.get(1).unwrap().as_str().parse().unwrap(),
		    from: caps.get(2).unwrap().as_str().parse().unwrap(),
		    to: caps.get(3).unwrap().as_str().parse().unwrap(),
		})
	    },
	    None => None,
	}
    }
}
	
