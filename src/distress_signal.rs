use crate::DailyProblem;

pub struct DistressSignal;

impl DailyProblem for DistressSignal {
    fn name(&self) -> &str {
        "Day 13: Distress Signal"
    }
    fn index(&self) -> u8 {
        13
    }
    fn solutions(&self, input: &str) -> (String, String) {
        ((-1).to_string(), (-1).to_string())
    }
}
