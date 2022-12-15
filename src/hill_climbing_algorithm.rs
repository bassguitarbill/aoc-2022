use std::collections::HashMap;

use priority_queue::DoublePriorityQueue;

use crate::DailyProblem;

pub struct HillClimbingAlgorithm;

impl DailyProblem for HillClimbingAlgorithm {
    fn name(&self) -> &str {
        "Day 12: Hill Climbing Algorithm"
    }
    fn index(&self) -> u8 {
        12
    }
    fn solutions(&self, input: &str) -> (String, String) {
        //println!("{:?}", );
        (
            dijkstra(heights(input), true).to_string(),
            dijkstra(heights(input), false).to_string(),
        )
    }
}

fn dijkstra(hts: Heights, part_1: bool) -> u32 {
    let mut unvisited_nodes_pqueue: DoublePriorityQueue<Position, u32> = DoublePriorityQueue::new();
    let mut visited_nodes: HashMap<Position, u32> = HashMap::new();
    if part_1 {
        unvisited_nodes_pqueue.push(hts.start_position, 0);
    } else {
        for (row_num, row) in hts.heights.iter().enumerate() {
            for (col_num, col) in row.iter().enumerate() {
                if *col == 0 {
                    unvisited_nodes_pqueue.push((row_num, col_num), 0);
                }
            }
        }
    }

    loop {
        let closest = unvisited_nodes_pqueue.pop_min().unwrap();
        if closest.0 == hts.end_position {
            return closest.1;
        };
        visited_nodes.insert(closest.0, closest.1);
        let closest_cost = closest.1;
        let closest_path = hts.get(closest.0 .0, closest.0 .1).unwrap();
        if let Some(cost) = closest_path.up_cost {
            if !visited_nodes.contains_key(&(closest.0 .0 - 1, closest.0 .1)) {
                unvisited_nodes_pqueue
                    .push_decrease((closest.0 .0 - 1, closest.0 .1), closest_cost + cost);
            }
        }
        if let Some(cost) = closest_path.down_cost {
            if !visited_nodes.contains_key(&(closest.0 .0 + 1, closest.0 .1)) {
                unvisited_nodes_pqueue
                    .push_decrease((closest.0 .0 + 1, closest.0 .1), closest_cost + cost);
            }
        }
        if let Some(cost) = closest_path.left_cost {
            if !visited_nodes.contains_key(&(closest.0 .0, closest.0 .1 - 1)) {
                unvisited_nodes_pqueue
                    .push_decrease((closest.0 .0, closest.0 .1 - 1), closest_cost + cost);
            }
        }
        if let Some(cost) = closest_path.right_cost {
            if !visited_nodes.contains_key(&(closest.0 .0, closest.0 .1 + 1)) {
                unvisited_nodes_pqueue
                    .push_decrease((closest.0 .0, closest.0 .1 + 1), closest_cost + cost);
            }
        }
    }
}

type Position = (usize, usize);

#[derive(Debug)]
struct Heights {
    heights: Vec<Vec<u32>>,
    paths: Vec<Vec<Path>>,
    start_position: Position,
    end_position: Position,
}

impl Heights {
    fn get(&self, row: usize, col: usize) -> Option<Path> {
        match self.paths.get(row) {
            Some(r) => r.get(col).copied(),
            None => None,
        }
    }
}

fn heights(input: &str) -> Heights {
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);

    let mut hts = vec![];
    // let mut line_num = 0;
    for (line_num, line) in input.lines().enumerate() {
        hts.push(vec![]);
        // let mut char_num = 0;
        for (char_num, char) in line.chars().enumerate() {
            hts.get_mut(line_num).unwrap().push(match char {
                'S' => {
                    start_position = (line_num, char_num);
                    0
                }
                'E' => {
                    end_position = (line_num, char_num);
                    25
                }
                c => c as u32 - 'a' as u32,
            });
        }
    }
    // (c as u32) - ('a' as i32)}).collect())
    let pts = paths(&hts);
    Heights {
        heights: hts,
        paths: pts,
        start_position,
        end_position,
    }
}

#[derive(Clone, Debug)]
struct Path {
    up_cost: Option<u32>,
    down_cost: Option<u32>,
    left_cost: Option<u32>,
    right_cost: Option<u32>,
}

impl Copy for Path {}

impl Path {
    fn default() -> Self {
        Self {
            up_cost: None,
            down_cost: None,
            left_cost: None,
            right_cost: None,
        }
    }
}

fn paths(hts: &Vec<Vec<u32>>) -> Vec<Vec<Path>> {
    let num_rows = hts.len();
    let num_cols = hts.get(0).unwrap().len();
    let mut paths = vec![vec![Path::default(); num_cols]; num_rows];
    for row in 0..num_rows {
        for col in 0..num_cols {
            let height = hts.get(row).unwrap().get(col).unwrap();
            let can_go_up = row > 0 && *hts.get(row - 1).unwrap().get(col).unwrap() < height + 2;
            let can_go_down =
                row < num_rows - 1 && *hts.get(row + 1).unwrap().get(col).unwrap() < height + 2;
            let can_go_left = col > 0 && *hts.get(row).unwrap().get(col - 1).unwrap() < height + 2;
            let can_go_right =
                col < num_cols - 1 && *hts.get(row).unwrap().get(col + 1).unwrap() < height + 2;
            let path = paths.get_mut(row).unwrap().get_mut(col).unwrap();
            if can_go_up {
                path.up_cost = Some(1)
            }
            if can_go_down {
                path.down_cost = Some(1)
            }
            if can_go_left {
                path.left_cost = Some(1)
            }
            if can_go_right {
                path.right_cost = Some(1)
            }
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use crate::hill_climbing_algorithm::{dijkstra, heights};

    #[test]
    fn t() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(dijkstra(heights(input), true), 31);
        assert_eq!(dijkstra(heights(input), false), 29);
    }
}
