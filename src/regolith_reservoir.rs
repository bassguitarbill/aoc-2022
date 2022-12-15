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
        (
            GameState::new(input, false).run().to_string(),
            GameState::new(input, true).run().to_string(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Position(usize, usize);

impl Position {
    fn new(input: &str) -> Self {
        let mut coords = input.split(',');
        Self(
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        )
    }
}

#[derive(Debug)]
struct Path(Vec<Position>);

impl Path {
    fn new(input: &str) -> Self {
        let mut all_positions = vec![];
        let mut positions = input.split(" -> ").map(Position::new);
        let mut position = positions.next().unwrap();
        all_positions.push(position.clone());
        for next_position in positions {
            while position != next_position {
                if position.1 == next_position.1 {
                    if position.0 > next_position.0 {
                        position.0 -= 1;
                    } else {
                        position.0 += 1;
                    }
                } else if position.1 > next_position.1 {
                    position.1 -= 1;
                } else {
                    position.1 += 1;
                }
                all_positions.push(position.clone());
            }
        }
        Self(all_positions)
        // Self(positions.map(Position::new).collect())
    }
}

#[derive(Debug)]
struct Paths(Vec<Path>);

impl Paths {
    fn new(input: &str) -> Self {
        Paths(input.lines().map(Path::new).collect())
    }
}

#[derive(Debug)]
enum Cell {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct GameState {
    cells: Vec<Vec<Cell>>,
    active_sand_position: Position,
    sand_units: u32,
    max_y: usize,
    has_floor: bool,
    entrance_blocked: bool,
}

impl GameState {
    fn new(input: &str, has_floor: bool) -> Self {
        let paths = Paths::new(input);
        let mut max_x = 0;
        let mut max_y = 0;
        for path in &paths.0 {
            for position in &path.0 {
                let x = position.0;
                if x > max_x {
                    max_x = x
                }
                let y = position.1;
                if y > max_y {
                    max_y = y
                }
            }
        }

        let mut cells = vec![];
        for _ in 0..=max_y + 1 {
            cells.push(vec![]);
            for _ in 0..=max_x * 2 {
                cells.last_mut().unwrap().push(Cell::Air);
            }
        }

        if has_floor {
            cells.push(vec![]);
            for _ in 0..=max_x * 2 {
                cells.last_mut().unwrap().push(Cell::Rock);
            }
        }

        for path in &paths.0 {
            for position in &path.0 {
                cells.get_mut(position.1).unwrap().push(Cell::Rock);
                cells.get_mut(position.1).unwrap().swap_remove(position.0);
            }
        }

        cells.get_mut(0).unwrap().push(Cell::Sand);
        cells.get_mut(0).unwrap().swap_remove(500);

        Self {
            cells,
            active_sand_position: Position(0, 500),
            sand_units: 0,
            max_y,
            has_floor,
            entrance_blocked: false,
        }
    }
}

impl GameState {
    fn print(&self) {
        for row in &self.cells {
            for cell in &row[450..550] {
                match cell {
                    Cell::Air => print!("."),
                    Cell::Rock => print!("#"),
                    Cell::Sand => print!("o"),
                }
            }
            println!();
        }
        println!("max y: {}", self.max_y);
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells.get_mut(row).unwrap().push(cell);
        self.cells.get_mut(row).unwrap().swap_remove(col);
    }

    fn get_cell(&mut self, row: usize, col: usize) -> &Cell {
        self.cells.get(row).unwrap().get(col).unwrap()
    }

    fn step(&mut self) {
        if let Cell::Air =
            self.get_cell(self.active_sand_position.0 + 1, self.active_sand_position.1)
        {
            self.set_cell(
                self.active_sand_position.0,
                self.active_sand_position.1,
                Cell::Air,
            );
            self.set_cell(
                self.active_sand_position.0 + 1,
                self.active_sand_position.1,
                Cell::Sand,
            );
            self.active_sand_position =
                Position(self.active_sand_position.0 + 1, self.active_sand_position.1);
        } else if let Cell::Air = self.get_cell(
            self.active_sand_position.0 + 1,
            self.active_sand_position.1 - 1,
        ) {
            self.set_cell(
                self.active_sand_position.0,
                self.active_sand_position.1,
                Cell::Air,
            );
            self.set_cell(
                self.active_sand_position.0 + 1,
                self.active_sand_position.1 - 1,
                Cell::Sand,
            );
            self.active_sand_position = Position(
                self.active_sand_position.0 + 1,
                self.active_sand_position.1 - 1,
            );
        } else if let Cell::Air = self.get_cell(
            self.active_sand_position.0 + 1,
            self.active_sand_position.1 + 1,
        ) {
            self.set_cell(
                self.active_sand_position.0,
                self.active_sand_position.1,
                Cell::Air,
            );
            self.set_cell(
                self.active_sand_position.0 + 1,
                self.active_sand_position.1 + 1,
                Cell::Sand,
            );
            self.active_sand_position = Position(
                self.active_sand_position.0 + 1,
                self.active_sand_position.1 + 1,
            );
        } else {
            if self.has_floor {
                if let Cell::Sand = self.get_cell(0, 500) {
                    self.entrance_blocked = true;
                }
            }
            self.set_cell(0, 500, Cell::Sand);
            self.active_sand_position = Position(0, 500);
            self.sand_units += 1;
        }
    }

    fn game_over(&self) -> bool {
        if self.has_floor {
            self.entrance_blocked
        } else {
            self.active_sand_position.0 > self.max_y
        }
    }

    fn run(&mut self) -> u32 {
        while !self.game_over() {
            self.step();
        }
        self.sand_units
    }
}

impl Iterator for GameState {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        self.step();
        match self.game_over() {
            true => None,
            false => Some(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::regolith_reservoir::GameState;

    #[test]
    fn init() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let mut gs = GameState::new(input, false);
        assert_eq!(gs.run(), 24);
        let mut gs = GameState::new(input, true);
        gs.run();
        gs.print();

        assert_eq!(gs.run(), 93);
    }
}
