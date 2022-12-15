use std::{cmp::Ordering, collections::HashSet, str::Lines};

use aoc_2022::DailyProblem;

pub struct RopeBridge;

impl DailyProblem for RopeBridge {
    fn name(&self) -> &str {
        "Day 9: Rope Bridge"
    }
    fn index(&self) -> u8 {
        9
    }
    fn solutions(&self, input: &str) -> (String, String) {
        let mut rope = Rope::new(2);
        let move_iterator = MoveIterator::new(input);
        move_iterator.for_each(|dir| rope.mv(dir));

        let mut long_rope = Rope::new(10);
        let move_iterator = MoveIterator::new(input);
        move_iterator.for_each(|dir| long_rope.mv(dir));

        (
            rope.tail_visited.len().to_string(),
            long_rope.tail_visited.len().to_string(),
        )
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Position>,
    tail_visited: HashSet<Position>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let mut tail_visited = HashSet::new();
        tail_visited.insert(Position::default());
        let knots = vec![Position::default(); length];
        Rope {
            knots,
            tail_visited,
        }
    }

    fn mv(&mut self, dir: Direction) {
        self.knots.first_mut().unwrap().mv(dir);
        for knot_index in 0..self.knots.len() - 1 {
            let directions = self
                .knots
                .get(knot_index)
                .unwrap()
                .follow_dir(self.knots.get(knot_index + 1).unwrap());
            for dir in directions.iter() {
                self.knots.get_mut(knot_index + 1).unwrap().mv(*dir);
            }
        }
        self.tail_visited.insert(*self.knots.last().unwrap());
    }
}

#[derive(PartialEq, Debug, Eq, Hash, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn mv(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                self.y -= 1;
            }
            Direction::Down => {
                self.y += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }
    }

    fn default() -> Self {
        Self { x: 0, y: 0 }
    }

    fn follow_dir(&self, other: &Self) -> Vec<Direction> {
        let mut directions = vec![];
        let dist = self.distance(other);
        if dist.x == 2 {
            directions.push(Direction::Right);
            match dist.y.cmp(&0) {
                Ordering::Greater => directions.push(Direction::Down),
                Ordering::Less => directions.push(Direction::Up),
                _ => {}
            }
        } else if dist.x == -2 {
            directions.push(Direction::Left);
            match dist.y.cmp(&0) {
                Ordering::Greater => directions.push(Direction::Down),
                Ordering::Less => directions.push(Direction::Up),
                _ => {}
            }
        } else if dist.y == 2 {
            directions.push(Direction::Down);
            match dist.x.cmp(&0) {
                Ordering::Greater => directions.push(Direction::Right),
                Ordering::Less => directions.push(Direction::Left),
                _ => {}
            }
        } else if dist.y == -2 {
            directions.push(Direction::Up);
            match dist.x.cmp(&0) {
                Ordering::Greater => directions.push(Direction::Right),
                Ordering::Less => directions.push(Direction::Left),
                _ => {}
            }
        }
        directions
    }

    fn distance(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(input: &str) -> Result<Self, String> {
        match input {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            x => Err(format!("Invalid direction {}", x)),
        }
    }
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Self::Up => Self::Up,
            Self::Down => Self::Down,
            Self::Left => Self::Left,
            Self::Right => Self::Right,
        }
    }
}

struct Move {
    dir: Direction,
    times: u32,
}

impl Move {
    fn new(input: &str) -> Self {
        let mut spl = input.split(' ');
        Self {
            dir: Direction::new(spl.next().unwrap()).unwrap(),
            times: spl.next().unwrap().parse().unwrap(),
        }
    }
}

struct MoveIterator<'a> {
    lines: Lines<'a>,
    current_move: Move,
    index_in_move: u32,
}

impl<'a> MoveIterator<'a> {
    fn new(input: &'a str) -> Self {
        let mut lines = input.lines();
        let current_move = Move::new(lines.next().unwrap());
        Self {
            lines,
            current_move,
            index_in_move: 0,
        }
    }
}

impl<'a> Iterator for MoveIterator<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        match self.index_in_move < self.current_move.times {
            true => {
                self.index_in_move += 1;
                Some(self.current_move.dir)
            }
            false => match self.lines.next() {
                Some(line) => {
                    self.current_move = Move::new(line);
                    self.index_in_move = 1;
                    Some(self.current_move.dir)
                }
                None => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, MoveIterator, Position, Rope};

    #[test]
    fn move_iterator() {
        let mut mi = MoveIterator::new(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next().unwrap(), Direction::Up);
        assert_eq!(mi.next().unwrap(), Direction::Up);
        assert_eq!(mi.next().unwrap(), Direction::Up);
        assert_eq!(mi.next().unwrap(), Direction::Up);
        assert_eq!(mi.next().unwrap(), Direction::Left);
        assert_eq!(mi.next().unwrap(), Direction::Left);
        assert_eq!(mi.next().unwrap(), Direction::Left);
        assert_eq!(mi.next().unwrap(), Direction::Down);
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next().unwrap(), Direction::Down);
        assert_eq!(mi.next().unwrap(), Direction::Left);
        assert_eq!(mi.next().unwrap(), Direction::Left);
        assert_eq!(mi.next().unwrap(), Direction::Left);
        assert_eq!(mi.next().unwrap(), Direction::Left);
        assert_eq!(mi.next().unwrap(), Direction::Left);
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next().unwrap(), Direction::Right);
        assert_eq!(mi.next(), None);
    }

    #[test]
    fn mv() {
        let mut pos = Position { x: 0, y: 0 };
        pos.mv(Direction::Up);
        assert_eq!(pos, Position { x: 0, y: -1 });
        pos.mv(Direction::Right);
        assert_eq!(pos, Position { x: 1, y: -1 });
        pos.mv(Direction::Down);
        assert_eq!(pos, Position { x: 1, y: 0 });
        pos.mv(Direction::Left);
        assert_eq!(pos, Position { x: 0, y: 0 });
    }

    #[test]
    fn main() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let mut rope = Rope::new(2);
        let move_iterator = MoveIterator::new(input);
        move_iterator.for_each(|dir| rope.mv(dir));
        assert_eq!(rope.tail_visited.len(), 13);

        let mut long_rope = Rope::new(10);
        let move_iterator = MoveIterator::new(input);
        move_iterator.for_each(|dir| long_rope.mv(dir));
        assert_eq!(long_rope.tail_visited.len(), 1);

        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let mut long_rope = Rope::new(10);
        let move_iterator = MoveIterator::new(input);
        move_iterator.for_each(|dir| long_rope.mv(dir));
        assert_eq!(long_rope.tail_visited.len(), 36);
    }
}
