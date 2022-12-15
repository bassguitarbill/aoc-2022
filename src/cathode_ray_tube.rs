use std::str::{FromStr, Lines};

use aoc_2022::DailyProblem;

pub struct CathodeRayTube;

impl DailyProblem for CathodeRayTube {
    fn name(&self) -> &str {
        "Day 10: Cathode-Ray Tube"
    }
    fn index(&self) -> u8 {
        10
    }
    fn solutions(&self, input: &str) -> (String, String) {
        let mut crt = Crt::new(input);
        (0..240).for_each(|_| crt.next_cycle());
        (
            signal_strengths(&mut Cpu::new(input), vec![20, 60, 100, 140, 180, 220])
                .iter()
                .sum::<i32>()
                .to_string(),
            crt.pixel_string(),
        )
    }
}

struct Crt<'a> {
    cpu: Cpu<'a>,
    cycle: u32,
    pixels: [[char; 40]; 6],
}

impl<'a> Crt<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            cpu: Cpu::new(input),
            cycle: 0,
            pixels: [['.'; 40]; 6],
        }
    }

    fn next_cycle(&mut self) {
        match self.should_draw_pixel() {
            true => self.pixels[self.current_row()][self.current_column()] = '#',
            false => self.pixels[self.current_row()][self.current_column()] = '.',
        }
        self.cycle += 1;
        self.cpu.next_cycle();
    }

    fn should_draw_pixel(&self) -> bool {
        (self.current_column() as i32).abs_diff(self.cpu.x as i32) < 2
    }

    fn current_column(&self) -> usize {
        self.cycle as usize % 40
    }

    fn current_row(&self) -> usize {
        self.cycle as usize / 40
    }

    fn pixel_string(&self) -> String {
        let rows = self.pixels.map(|row| row.iter().collect::<String>());
        let mut output = "\n".to_string();
        output.push_str(&rows.join("\n"));
        output
    }
}

fn signal_strengths(cpu: &mut Cpu, cycles: Vec<u32>) -> Vec<i32> {
    let mut strengths = vec![];
    for cycle in cycles.iter() {
        (cpu.cycle..*cycle).for_each(|_| cpu.next_cycle());
        strengths.push(cpu.signal_strength);
    }
    strengths
}

struct Cpu<'a> {
    x: i32,
    cycle: u32,
    adding: bool,
    instructions: InstructionIterator<'a>,
    to_add: i32,
    signal_strength: i32,
}

impl<'a> Cpu<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            instructions: InstructionIterator {
                lines: input.lines(),
            },
            x: 1,
            cycle: 0,
            adding: false,
            to_add: 0,
            signal_strength: 0,
        }
    }

    fn next_cycle(&mut self) {
        self.cycle += 1;
        self.signal_strength = self.cycle as i32 * self.x;
        if self.adding {
            self.adding = false;
            self.x += self.to_add;
            self.to_add = 0;
        } else {
            match self.instructions.next() {
                Some(Instruction::Noop) => {}
                Some(Instruction::AddX(to_add)) => {
                    self.adding = true;
                    self.to_add = to_add;
                }
                None => {}
            }
        }
    }
}

struct InstructionIterator<'a> {
    lines: Lines<'a>,
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .next()
            .map(|line| Instruction::from_str(line).unwrap())
    }
}

enum Instruction {
    AddX(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else {
            Ok(Self::AddX(s.split(' ').nth(1).unwrap().parse().unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{signal_strengths, Cpu, Crt};

    #[test]
    fn cycle() {
        let input = "noop
addx 3
addx -5";
        let mut cpu = Cpu::new(input);
        assert_eq!(cpu.x, 1);
        cpu.next_cycle();
        assert_eq!(cpu.x, 1);
        cpu.next_cycle();
        assert_eq!(cpu.x, 1);
        cpu.next_cycle();
        assert_eq!(cpu.x, 4);
        cpu.next_cycle();
        assert_eq!(cpu.x, 4);
        cpu.next_cycle();
        assert_eq!(cpu.x, -1);
    }

    #[test]
    fn signal_strength() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let mut cpu = Cpu::new(input);
        (0..20).for_each(|_| cpu.next_cycle());
        assert_eq!(cpu.signal_strength, 420);
        (0..40).for_each(|_| cpu.next_cycle());
        assert_eq!(cpu.signal_strength, 1140);
        (0..40).for_each(|_| cpu.next_cycle());
        assert_eq!(cpu.signal_strength, 1800);
        (0..40).for_each(|_| cpu.next_cycle());
        assert_eq!(cpu.signal_strength, 2940);
        (0..40).for_each(|_| cpu.next_cycle());
        assert_eq!(cpu.signal_strength, 2880);
        (0..40).for_each(|_| cpu.next_cycle());
        assert_eq!(cpu.signal_strength, 3960);
    }

    #[test]
    fn check_signal_strengths() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let mut cpu = Cpu::new(input);
        assert_eq!(
            signal_strengths(&mut cpu, vec![20, 60, 100, 140, 180, 220]),
            vec![420, 1140, 1800, 2940, 2880, 3960]
        );
    }

    #[test]
    fn pixel_string() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let mut crt = Crt::new(input);
        assert_eq!(
            crt.pixel_string(),
            "
........................................
........................................
........................................
........................................
........................................
........................................"
        );
        (0..240).for_each(|_| crt.next_cycle());
        assert_eq!(
            crt.pixel_string(),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
