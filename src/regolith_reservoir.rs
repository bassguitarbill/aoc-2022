use crate::DailyProblem;

pub struct RegolithReservoir;

impl DailyProblem for RegolithReservoir {
    fn name(&self) -> &str {
        "Day 14: Regolith Reservoir"
    }
    fn index(&self) -> u8 {
        14
    }
    fn solutions(&self, input: &str) -> (String, String) {
        ((-1).to_string(), (-1).to_string())
    }
}
