mod calorie_counting;
mod rock_paper_scissors;
mod rucksack_reorganization;
mod camp_cleanup;
mod supply_stacks;
mod tuning_trouble;
mod no_space_left_on_device;
mod treetop_tree_house;
mod rope_bridge;

use std::env;
use aoc_2022::DailyProblem;
use crate::calorie_counting::CalorieCounting;
use crate::rock_paper_scissors::RockPaperScissors;
use crate::rucksack_reorganization::RucksackReorganization;
use crate::camp_cleanup::CampCleanup;
use crate::supply_stacks::SupplyStacks;
use crate::tuning_trouble::TuningTrouble;
use crate::no_space_left_on_device::NoSpaceLeftOnDevice;
use crate::treetop_tree_house::TreetopTreeHouse;
use crate::rope_bridge::RopeBridge;

fn main() {
    let problems: Vec<Box<dyn DailyProblem>> = vec![
	Box::new(CalorieCounting{}),
	Box::new(RockPaperScissors{}),
	Box::new(RucksackReorganization{}),
	Box::new(CampCleanup{}),
	Box::new(SupplyStacks{}),
	Box::new(TuningTrouble{}),
	Box::new(NoSpaceLeftOnDevice{}),
	Box::new(TreetopTreeHouse{}),
	Box::new(RopeBridge{}),
	];

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    if args.is_empty() {
	for p in problems.iter() {
	    p.print_solutions();
	    println!();
	}
    } else {
	let arg = args.first().unwrap().parse::<usize>();
	match arg {
	    Ok(num) => {
		if num >= 1 && num <= problems.len() {
		    problems.get(num - 1).unwrap().print_solutions();
		} else {
		    panic!("Invalid problem number {}", num);
		}
	    },
	    Err(_) => { panic!("Arg must be numeric") }
	}
    }
}
