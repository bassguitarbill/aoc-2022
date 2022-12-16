use std::collections::HashSet;

use crate::DailyProblem;
use lazy_static::lazy_static;
use regex::Regex;

pub struct BeaconExclusionZone;

impl DailyProblem for BeaconExclusionZone {
    fn name(&self) -> &str {
        "Day 15: Beacon Exclusion Zone"
    }
    fn index(&self) -> u8 {
        15
    }
    fn solutions(&self, input: &str) -> (String, String) {
        (
            no_beacons_at_row(input, 2000000).to_string(),
            diagonals(input).to_string(),
        )
    }
}

fn diagonals(input: &str) -> i64 {
    let mut diagonals = vec![];
    input
        .lines()
        .map(SensorBeaconPair::new)
        .map(|sbp| sbp.diagonals())
        .for_each(|mut ds| diagonals.append(&mut ds));
    diagonals.sort();
    let mut slopes = vec![];
    for i in 0..(diagonals.len() - 1) {
        if diagonals.get(i).unwrap().y_intercept - diagonals.get(i + 1).unwrap().y_intercept == -2 {
            slopes.push(diagonals.get(i).unwrap().y_intercept + 1);
        }
    }
    // println!("slopes: {:?}", slopes);
    let y = (slopes.get(0).unwrap() + slopes.get(1).unwrap()) / 2;
    let x = (slopes.get(1).unwrap() - slopes.get(0).unwrap()) / 2;
    // println!("x: {}, y: {}", x, y);
    ((x as i64) * 4000000) + (y as i64)
}

#[derive(Debug)]
struct Range(i32, i32);
fn no_beacons_at_row(input: &str, row: i32) -> u32 {
    let ranges: Vec<Range> = input
        .lines()
        .map(SensorBeaconPair::new)
        .filter_map(|sbp| sbp.range_at_row(row))
        .collect();
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for range in &ranges {
        if range.0 < min {
            min = range.0
        }
        if range.1 > max {
            max = range.1
        }
    }
    let mut sum = 0;
    for x in min..=max {
        for range in &ranges {
            if x >= range.0 && x <= range.1 {
                sum += 1;
                break;
            }
        }
    }

    let mut beacons_in_row = HashSet::new();
    input
        .lines()
        .map(SensorBeaconPair::new)
        .map(|sbp| sbp.1)
        .for_each(|pos| {
            beacons_in_row.insert(pos);
        });

    let beacons_in_row_count = beacons_in_row
        .iter()
        .filter(|y| y.1 == row)
        .collect::<Vec<&Position>>()
        .len();
    sum - 1
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position(i32, i32);

#[derive(Debug)]
struct SensorBeaconPair(Position, Position);

impl SensorBeaconPair {
    fn new(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "Sensor at x=(-?\\d*), y=(-?\\d*): closest beacon is at x=(-?\\d*), y=(-?\\d*)"
            )
            .unwrap();
        }
        let caps = RE.captures(input).unwrap();
        Self(
            Position(
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            ),
            Position(
                caps.get(3).unwrap().as_str().parse().unwrap(),
                caps.get(4).unwrap().as_str().parse().unwrap(),
            ),
        )
    }

    fn distance(&self) -> i32 {
        (self.0 .0 - self.1 .0).abs() + (self.0 .1 - self.1 .1).abs()
    }

    fn range_at_row(&self, row: i32) -> Option<Range> {
        let sen_bea_dist = self.distance();
        let sen_y_dist = (self.0 .1 - row).abs();
        let half_range_width = sen_bea_dist - sen_y_dist;
        if half_range_width < 0 {
            None
        } else {
            Some(Range(
                self.0 .0 - half_range_width,
                self.0 .0 + half_range_width,
            ))
        }
    }

    fn diagonals(&self) -> Vec<Diagonal> {
        vec![
            Diagonal {
                positive_slope: false,
                y_intercept: self.0 .1 - self.0 .0 + self.distance(),
            },
            Diagonal {
                positive_slope: true,
                y_intercept: self.0 .1 + self.0 .0 - self.distance(),
            },
            Diagonal {
                positive_slope: false,
                y_intercept: self.0 .1 - self.0 .0 - self.distance(),
            },
            Diagonal {
                positive_slope: true,
                y_intercept: self.0 .1 + self.0 .0 + self.distance(),
            },
        ]
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Diagonal {
    positive_slope: bool,
    y_intercept: i32,
}

#[cfg(test)]
mod tests {
    use super::{no_beacons_at_row, SensorBeaconPair};

    #[test]
    fn parse() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        println!("{}", no_beacons_at_row(input, 10));
    }
    #[test]
    fn diagonals() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let mut diagonals = vec![];
        input
            .lines()
            .map(SensorBeaconPair::new)
            .map(|sbp| sbp.diagonals())
            .for_each(|mut ds| diagonals.append(&mut ds));
        diagonals.sort();
        for d in diagonals {
            println!("{:?}", d);
        }
    }
}
