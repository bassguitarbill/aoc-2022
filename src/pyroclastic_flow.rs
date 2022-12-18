use std::{collections::HashSet, fmt::Display};

use crate::DailyProblem;

pub struct PyroclasticFlow;

impl DailyProblem for PyroclasticFlow {
    fn name(&self) -> &str {
        "Day 17: Pyroclastic Flow"
    }
    fn index(&self) -> u8 {
        17
    }
    fn solutions(&self, input: &str) -> (String, String) {
        (
            highest_rock_after_x_pieces(input, 2022).to_string(),
            (-1).to_string(),
        )
    }
}

fn highest_rock_after_x_pieces(input: &str, pieces: u32) -> usize {
    let mut cave = Cave::new(input);
    cave.spawn_piece();
    while cave.dropped_pieces <= pieces {
        cave.step();
    }
    cave.highest_rock()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position(usize, usize);

#[derive(Debug)]
struct Cave {
    cells: Vec<Vec<bool>>,
    pieces: PieceTemplateIterator,
    jets: JetIterator,
    active_piece: Option<Piece>,
    dropped_pieces: u32,
}

const CAVE_WIDTH: usize = 7;

impl Cave {
    fn new(input: &str) -> Self {
        Self {
            cells: vec![vec![true; CAVE_WIDTH]],
            pieces: PieceTemplateIterator::new(PIECE_TEMPLATE_STRING),
            jets: JetIterator::new(input),
            active_piece: None,
            dropped_pieces: 0,
        }
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn highest_rock(&self) -> usize {
        let mut row_index = 0;
        while !self.is_empty_row(row_index) {
            row_index += 1;
        }
        row_index
    }

    fn is_empty_row(&self, row_index: usize) -> bool {
        //dbg!("rows: {}, row_index: {}", self.cells.len(), row_index);
        if row_index > 0 && self.cells.len() < row_index + 1 {
            return true;
        }
        self.cells
            .get(row_index)
            .unwrap()
            .iter()
            .filter(|c| **c)
            .count()
            == 0
    }

    fn add_row(&mut self) {
        self.cells.push(vec![false; CAVE_WIDTH]);
    }

    fn spawn_piece(&mut self) {
        while self.height() < self.highest_rock() + MAX_PIECE_HEIGHT + HEIGHT_ABOVE_BOARD {
            self.add_row();
        }
        self.active_piece = Some(Piece {
            template: self.pieces.next().unwrap(),
            position: Position(self.highest_rock() + HEIGHT_ABOVE_BOARD, 2),
        });
        self.dropped_pieces += 1;
        // println!("{}", self.as_string());
    }

    fn as_string(&self) -> String {
        let mut active_piece_cells = HashSet::new();
        let active_piece = self.active_piece.as_ref().unwrap();
        for cell in active_piece.cells() {
            active_piece_cells.insert(cell);
        }
        let mut s = String::new();
        for (row_index, row) in self.cells.iter().rev().enumerate() {
            s.push('|');
            for (col_index, cell) in row.iter().enumerate() {
                if self.active_piece.as_ref().unwrap().position
                    == Position(self.height() - row_index - 1, col_index)
                {
                    s.push('*');
                } else if active_piece_cells
                    .contains(&Position(self.height() - row_index - 1, col_index))
                {
                    s.push('o');
                } else if *cell {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('|');
            s.push('\n');
        }
        s
    }

    fn step(&mut self) {
        let jet = self.jets.next().unwrap();
        self.move_active_piece_laterally(jet);
        if self.can_drop_piece() {
            self.drop_active_piece();
        } else {
            self.solidify_active_piece();
        }
    }

    fn move_active_piece_laterally(&mut self, dir: JetDirection) {
        if self.can_move_active_piece_laterally(&dir) {
            let active_piece = self.active_piece.as_mut().unwrap();
            match dir {
                JetDirection::Left => active_piece.move_left(),
                JetDirection::Right => active_piece.move_right(),
            }
        }
    }

    fn can_move_active_piece_laterally(&self, dir: &JetDirection) -> bool {
        let active_piece = self.active_piece.as_ref().unwrap();
        match dir {
            JetDirection::Left => {
                if active_piece.is_on_left_wall() {
                    return false;
                }
                let left_piece = Piece {
                    template: active_piece.template.clone(),
                    position: Position(active_piece.position.0, active_piece.position.1 - 1),
                };
                !self.piece_intersects_with_rock(&left_piece)
            }
            JetDirection::Right => {
                if active_piece.is_on_right_wall() {
                    return false;
                }
                let right_piece = Piece {
                    template: active_piece.template.clone(),
                    position: Position(active_piece.position.0, active_piece.position.1 + 1),
                };
                !self.piece_intersects_with_rock(&right_piece)
            }
        }
    }

    fn solidify_active_piece(&mut self) {
        for cell in self.active_piece.as_ref().unwrap().cells() {
            self.cells.get_mut(cell.0).unwrap().push(true);
            self.cells.get_mut(cell.0).unwrap().swap_remove(cell.1);
        }
        self.spawn_piece();
    }

    fn drop_active_piece(&mut self) {
        self.active_piece.as_mut().unwrap().drop();
    }

    fn can_drop_piece(&self) -> bool {
        let active_piece = self.active_piece.as_ref().unwrap();
        let dropped_piece = Piece {
            template: active_piece.template.clone(),
            position: Position(active_piece.position.0 - 1, active_piece.position.1),
        };
        !self.piece_intersects_with_rock(&dropped_piece)
    }

    fn piece_intersects_with_rock(&self, piece: &Piece) -> bool {
        let mut active_piece_cells = HashSet::new();
        for cell in piece.cells() {
            active_piece_cells.insert(cell);
        }
        // println!("piece to check: {:?}", &piece);
        for (row_index, row) in self.cells.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                if *cell && active_piece_cells.contains(&Position(row_index, col_index)) {
                    return true;
                }
            }
        }
        false
    }
}

const MAX_PIECE_HEIGHT: usize = 3;
const HEIGHT_ABOVE_BOARD: usize = 3;

#[derive(Debug)]
struct Piece {
    template: PieceTemplate,
    position: Position,
}

impl Piece {
    fn cells(&self) -> Vec<Position> {
        let mut cells = vec![];
        for (row_index, row) in self.template.cells.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                if *cell {
                    cells.push(Position(
                        row_index + self.position.0,
                        col_index + self.position.1,
                    ));
                }
            }
        }
        cells
    }

    fn drop(&mut self) {
        self.position.0 -= 1;
    }

    fn is_on_right_wall(&self) -> bool {
        self.position.1 + self.template.width >= CAVE_WIDTH
    }

    fn is_on_left_wall(&self) -> bool {
        self.position.1 == 0
    }

    fn move_left(&mut self) {
        self.position.1 -= 1;
    }

    fn move_right(&mut self) {
        self.position.1 += 1;
    }
}

#[derive(Clone, Debug, PartialEq)]
enum JetDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct JetIterator {
    jet_directions: Vec<JetDirection>,
    index: usize,
}

impl JetIterator {
    fn new(input: &str) -> Self {
        let mut jet_directions = vec![];
        for c in input.lines().next().unwrap().chars() {
            jet_directions.push(match c {
                '>' => JetDirection::Right,
                '<' => JetDirection::Left,
                invalid_char => panic!("Invalid jet direction {}", invalid_char),
            });
        }
        Self {
            jet_directions,
            index: 0,
        }
    }
}

impl Iterator for JetIterator {
    type Item = JetDirection;
    fn next(&mut self) -> Option<Self::Item> {
        let len = self.jet_directions.len();
        let dir = self.jet_directions.get(self.index % len).unwrap();
        self.index += 1;
        Some(dir.clone())
    }
}

#[derive(Clone, Debug)]
struct PieceTemplate {
    height: usize,
    width: usize,
    cells: Vec<Vec<bool>>,
}

impl PieceTemplate {
    fn new(input: &str) -> Self {
        let mut cells = vec![];
        for row in input.lines() {
            cells.push(vec![]);
            for c in row.chars() {
                cells.last_mut().unwrap().push(match c {
                    '#' => true,
                    '.' => false,
                    invalid_char => panic!("Invalid character {}", invalid_char),
                });
            }
        }
        let height = cells.len();
        let width = cells.get(0).unwrap().len();
        Self {
            cells,
            height,
            width,
        }
    }

    fn as_string(&self) -> String {
        let mut s = "".to_string();
        for line in self.cells.iter() {
            for cell in line {
                match cell {
                    true => s.push('#'),
                    false => s.push(' '),
                }
            }
            s.push('\n');
        }
        s
    }
}

#[derive(Debug)]
struct PieceTemplateIterator {
    piece_templates: Vec<PieceTemplate>,
    index: usize,
}

impl Iterator for PieceTemplateIterator {
    type Item = PieceTemplate;

    fn next(&mut self) -> Option<Self::Item> {
        let length = self.piece_templates.len();
        self.index += 1;
        Some(
            self.piece_templates
                .get(self.index % length)
                .unwrap()
                .clone(),
        )
    }
}

impl PieceTemplateIterator {
    fn new(input: &str) -> Self {
        let piece_templates: Vec<PieceTemplate> =
            input.split("\n\n").map(PieceTemplate::new).collect();
        let length = &piece_templates.len();
        Self {
            piece_templates,
            index: length - 1,
        }
    }
}

const PIECE_TEMPLATE_STRING: &str = "####

.#.
###
.#.

###
..#
..#

#
#
#
#

##
##";

#[cfg(test)]
mod tests {
    use crate::pyroclastic_flow::{JetDirection, JetIterator};

    use super::{Cave, PieceTemplateIterator, PIECE_TEMPLATE_STRING};

    #[test]
    fn piece_template_iterator() {
        let mut pti = PieceTemplateIterator::new(PIECE_TEMPLATE_STRING);
        assert_eq!(pti.next().unwrap().as_string(), "####\n");
        assert_eq!(pti.next().unwrap().as_string(), " # \n###\n # \n");
        assert_eq!(pti.next().unwrap().as_string(), "  #\n  #\n###\n");
        assert_eq!(pti.next().unwrap().as_string(), "#\n#\n#\n#\n");
        assert_eq!(pti.next().unwrap().as_string(), "##\n##\n");
        assert_eq!(pti.next().unwrap().as_string(), "####\n");
    }

    #[test]
    fn jet_iterator() {
        let mut ji = JetIterator::new("><>");
        assert_eq!(ji.next().unwrap(), JetDirection::Right);
        assert_eq!(ji.next().unwrap(), JetDirection::Left);
        assert_eq!(ji.next().unwrap(), JetDirection::Right);
        assert_eq!(ji.next().unwrap(), JetDirection::Right);
        assert_eq!(ji.next().unwrap(), JetDirection::Left);
    }

    #[test]
    fn run() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut cave = Cave::new(input);
        cave.spawn_piece();
        while cave.dropped_pieces < 11 {
            cave.step();
        }
        panic!()
    }
}
