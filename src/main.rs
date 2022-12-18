mod beacon_exclusion_zone;
mod calorie_counting;
mod camp_cleanup;
mod cathode_ray_tube;
mod distress_signal;
mod hill_climbing_algorithm;
mod monkey_in_the_middle;
mod no_space_left_on_device;
mod proboscidea_volcanium;
mod pyroclastic_flow;
mod regolith_reservoir;
mod rock_paper_scissors;
mod rope_bridge;
mod rucksack_reorganization;
mod supply_stacks;
mod treetop_tree_house;
mod tuning_trouble;

use crate::beacon_exclusion_zone::BeaconExclusionZone;
use crate::calorie_counting::CalorieCounting;
use crate::camp_cleanup::CampCleanup;
use crate::cathode_ray_tube::CathodeRayTube;
use crate::distress_signal::DistressSignal;
use crate::hill_climbing_algorithm::HillClimbingAlgorithm;
use crate::monkey_in_the_middle::MonkeyInTheMiddle;
use crate::no_space_left_on_device::NoSpaceLeftOnDevice;
use crate::proboscidea_volcanium::ProboscideaVolcanium;
use crate::pyroclastic_flow::PyroclasticFlow;
use crate::regolith_reservoir::RegolithReservoir;
use crate::rock_paper_scissors::RockPaperScissors;
use crate::rope_bridge::RopeBridge;
use crate::rucksack_reorganization::RucksackReorganization;
use crate::supply_stacks::SupplyStacks;
use crate::treetop_tree_house::TreetopTreeHouse;
use crate::tuning_trouble::TuningTrouble;
use aoc_2022::DailyProblem;
use std::env;

fn main() {
    let problems: Vec<Box<dyn DailyProblem>> = vec![
        Box::new(CalorieCounting {}),
        Box::new(RockPaperScissors {}),
        Box::new(RucksackReorganization {}),
        Box::new(CampCleanup {}),
        Box::new(SupplyStacks {}),
        Box::new(TuningTrouble {}),
        Box::new(NoSpaceLeftOnDevice {}),
        Box::new(TreetopTreeHouse {}),
        Box::new(RopeBridge {}),
        Box::new(CathodeRayTube {}),
        Box::new(MonkeyInTheMiddle {}),
        Box::new(HillClimbingAlgorithm {}),
        Box::new(DistressSignal {}),
        Box::new(RegolithReservoir {}),
        Box::new(BeaconExclusionZone {}),
        Box::new(ProboscideaVolcanium {}),
        Box::new(PyroclasticFlow {}),
    ];

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    if let Some(last_arg) = args.last() {
        match last_arg.parse::<usize>() {
            Ok(to_run) => {
                let to_run = to_run;
                if to_run >= 1 && to_run <= problems.len() {
                    problems.get(to_run - 1).unwrap().print_solutions();
                } else {
                    panic!("Invalid problem number {}", to_run);
                }
            }
            Err(_) => {
                for p in problems.iter() {
                    p.print_solutions();
                    println!();
                }
            }
        }
    } else {
        for p in problems.iter() {
            p.print_solutions();
            println!();
        }
    }
}
