use std::collections::HashSet;

use crate::DailyProblem;

pub struct BoilingBoulders;

impl DailyProblem for BoilingBoulders {
    fn name(&self) -> &str {
        "Day 18: Boiling Boulders"
    }
    fn index(&self) -> u8 {
        18
    }
    fn solutions(&self, input: &str) -> (String, String) {
        (
            total_surface_area(cubes_from_string(input)).to_string(),
            total_external_surface_area(input).to_string(),
        )
    }
}

// part 1
// a cube at 1,1,1 has six sides
// let's assume it spans from point 1,1,1 to 2,2,2
// it has two sides with a normal in the x direction (x sides),
// one at 1,1,1 to 1,2,2, the other at 2,1,1 to 2,2,2
// it has two y sides (1,1,1 to 2,1,2 and 1,2,1 to 2,2,2)
// and two z sides (1,1,1 to 2,2,1 and 1,1,2 to 2,2,2)
// adding a fresh cube should add all six of these sides
// if one of these sides already exists, though, it means
// that it's no longer on the surface and should be removed

fn cubes_from_string(input: &str) -> Vec<Cube> {
    input.lines().map(Cube::new).collect()
}

fn total_surface_area(cubes: Vec<Cube>) -> usize {
    let mut x_sides = HashSet::new();
    let mut y_sides = HashSet::new();
    let mut z_sides = HashSet::new();

    for cube in cubes {
        for xs in cube.x_sides() {
            if x_sides.contains(&xs) {
                x_sides.remove(&xs);
            } else {
                x_sides.insert(xs);
            }
        }
        for ys in cube.y_sides() {
            if y_sides.contains(&ys) {
                y_sides.remove(&ys);
            } else {
                y_sides.insert(ys);
            }
        }
        for zs in cube.z_sides() {
            if z_sides.contains(&zs) {
                z_sides.remove(&zs);
            } else {
                z_sides.insert(zs);
            }
        }
    }
    x_sides.len() + y_sides.len() + z_sides.len()
}

// part 2
// i think i need to do a 3d cellular automata
// a cell can be in one of three states: rock, out, or in
// all edges of the bounding box start as out, all cubes
// start as rock, and everything else starts as in
// every step, out -> out, rock -> rock, and
// in -> out if it has at least 1 orthogonal out neighbor,
// otherwise it remains in
// keep stepping until the number of in and out no longer
// change, then run the total surface area algorithm on
// the initial cubes, then subtract the result of the total
// surface area of all of the in cubes

fn total_external_surface_area(input: &str) -> usize {
    let rocks = cubes_from_string(input);
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut min_z = 0;
    let mut max_z = 0;
    let mut rocks_set = HashSet::new();
    for rock in rocks.iter() {
        if rock.0 < min_x {
            min_x = rock.0
        }
        if rock.0 > max_x {
            max_x = rock.0
        }
        if rock.1 < min_y {
            min_y = rock.1
        }
        if rock.1 > max_y {
            max_y = rock.1
        }
        if rock.2 < min_z {
            min_z = rock.2
        }
        if rock.2 > max_z {
            max_z = rock.2
        }
        rocks_set.insert(rock);
    }

    let mut grid = vec![];
    for x in min_x..=max_x {
        grid.push(vec![]);
        for y in min_y..=max_y {
            grid.last_mut().unwrap().push(vec![]);
            for z in min_z..=max_z {
                grid.last_mut().unwrap().last_mut().unwrap().push(
                    match rocks_set.contains(&Cube(x, y, z)) {
                        true => Cell::Rock,
                        false => Cell::In,
                    },
                );
            }
        }
    }

    for y in (min_y)..=max_y {
        for z in (min_z)..=max_z {
            if let Cell::In = grid.get(min_x).unwrap().get(y).unwrap().get(z).unwrap() {
                grid.get_mut(min_x)
                    .unwrap()
                    .get_mut(y)
                    .unwrap()
                    .push(Cell::Out);
                grid.get_mut(min_x)
                    .unwrap()
                    .get_mut(y)
                    .unwrap()
                    .swap_remove(z);
            }
            if let Cell::In = grid.get(max_x).unwrap().get(y).unwrap().get(z).unwrap() {
                grid.get_mut(max_x)
                    .unwrap()
                    .get_mut(y)
                    .unwrap()
                    .push(Cell::Out);
                grid.get_mut(max_x)
                    .unwrap()
                    .get_mut(y)
                    .unwrap()
                    .swap_remove(z);
            }
        }
    }

    for x in (min_x)..=(max_x) {
        for z in (min_z)..=(max_z) {
            if let Cell::In = grid.get(x).unwrap().get(min_y).unwrap().get(z).unwrap() {
                grid.get_mut(x)
                    .unwrap()
                    .get_mut(min_y)
                    .unwrap()
                    .push(Cell::Out);
                grid.get_mut(x)
                    .unwrap()
                    .get_mut(min_y)
                    .unwrap()
                    .swap_remove(z);
            }
            if let Cell::In = grid.get(x).unwrap().get(max_y).unwrap().get(z).unwrap() {
                grid.get_mut(x)
                    .unwrap()
                    .get_mut(max_y)
                    .unwrap()
                    .push(Cell::Out);
                grid.get_mut(x)
                    .unwrap()
                    .get_mut(max_y)
                    .unwrap()
                    .swap_remove(z);
            }
        }
    }

    for x in (min_x)..=max_x {
        for y in (min_y)..=max_y {
            if let Cell::In = grid.get(x).unwrap().get(y).unwrap().get(min_z).unwrap() {
                grid.get_mut(x).unwrap().get_mut(y).unwrap().push(Cell::Out);
                grid.get_mut(x)
                    .unwrap()
                    .get_mut(y)
                    .unwrap()
                    .swap_remove(min_z);
            }
            if let Cell::In = grid.get(x).unwrap().get(y).unwrap().get(max_z).unwrap() {
                grid.get_mut(x).unwrap().get_mut(y).unwrap().push(Cell::Out);
                grid.get_mut(x)
                    .unwrap()
                    .get_mut(y)
                    .unwrap()
                    .swap_remove(max_z);
            }
        }
    }

    let mut change = true;
    while change {
        change = false;

        for x in min_x..max_x {
            for y in min_y..max_y {
                for z in min_z..max_z {
                    if let Cell::In = grid.get(x).unwrap().get(y).unwrap().get(z).unwrap() {
                        if grid
                            .get(x - 1)
                            .unwrap()
                            .get(y)
                            .unwrap()
                            .get(z)
                            .unwrap()
                            .out()
                            || grid
                                .get(x + 1)
                                .unwrap()
                                .get(y)
                                .unwrap()
                                .get(z)
                                .unwrap()
                                .out()
                            || grid
                                .get(x)
                                .unwrap()
                                .get(y - 1)
                                .unwrap()
                                .get(z)
                                .unwrap()
                                .out()
                            || grid
                                .get(x)
                                .unwrap()
                                .get(y + 1)
                                .unwrap()
                                .get(z)
                                .unwrap()
                                .out()
                            || grid
                                .get(x)
                                .unwrap()
                                .get(y)
                                .unwrap()
                                .get(z - 1)
                                .unwrap()
                                .out()
                            || grid
                                .get(x)
                                .unwrap()
                                .get(y)
                                .unwrap()
                                .get(z + 1)
                                .unwrap()
                                .out()
                        {
                            change = true;
                            grid.get_mut(x).unwrap().get_mut(y).unwrap().push(Cell::Out);
                            grid.get_mut(x).unwrap().get_mut(y).unwrap().swap_remove(z);
                        }
                    }
                }
            }
        }
    }

    let mut in_set = HashSet::new();
    for x in min_x..max_x {
        for y in min_y..max_y {
            for z in min_z..max_z {
                if let Cell::In = grid.get(x).unwrap().get(y).unwrap().get(z).unwrap() {
                    in_set.insert(Cube(x, y, z));
                }
            }
        }
    }

    total_surface_area(cubes_from_string(input)) - total_surface_area(in_set.drain().collect())
}

#[derive(PartialEq, Debug)]
enum Cell {
    Out,
    In,
    Rock,
}

impl Cell {
    fn out(&self) -> bool {
        *self == Self::Out
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Cube(usize, usize, usize);

#[derive(Eq, PartialEq, Hash)]
struct Side(usize, usize, usize);

impl Cube {
    fn new(input: &str) -> Self {
        let mut chunks = input.split(',').map(|c| c.parse::<usize>().unwrap());
        Cube(
            chunks.next().unwrap(),
            chunks.next().unwrap(),
            chunks.next().unwrap(),
        )
    }

    fn x_sides(&self) -> Vec<Side> {
        vec![
            Side(self.0, self.1, self.2),
            Side(self.0 + 1, self.1, self.2),
        ]
    }

    fn y_sides(&self) -> Vec<Side> {
        vec![
            Side(self.0, self.1, self.2),
            Side(self.0, self.1 + 1, self.2),
        ]
    }
    fn z_sides(&self) -> Vec<Side> {
        vec![
            Side(self.0, self.1, self.2),
            Side(self.0, self.1, self.2 + 1),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::boiling_boulders::{
        cubes_from_string, total_external_surface_area, total_surface_area,
    };

    #[test]
    fn smol() {
        let input = "1,1,1
2,1,1";
        assert_eq!(total_surface_area(cubes_from_string(input)), 10);
    }

    #[test]
    fn larg() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(total_surface_area(cubes_from_string(input)), 64);
    }
    #[test]
    fn in_set() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(total_external_surface_area(input), 58);
    }
}
