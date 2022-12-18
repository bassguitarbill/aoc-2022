use crate::DailyProblem;

pub struct ProboscideaVolcanium;

impl DailyProblem for ProboscideaVolcanium {
    fn name(&self) -> &str {
        "Day 16: Proboscidea Volcanium"
    }
    fn index(&self) -> u8 {
        16
    }
    fn solutions(&self, _input: &str) -> (String, String) {
        (
            "This one looked too challenging".to_string(),
            "I'll look into it later".to_string(),
        )
    }
}
