mod calorie_counting;
mod rock_paper_scissors;

use aoc_2022::DailyProblem;
use crate::calorie_counting::CalorieCounting;
use crate::rock_paper_scissors::RockPaperScissors;

fn main() {
    let cc: Box<dyn DailyProblem> = Box::new(CalorieCounting{});
    cc.print_solutions();
    println!();
    let rps: Box<dyn DailyProblem> = Box::new(RockPaperScissors{});
    rps.print_solutions();
}
