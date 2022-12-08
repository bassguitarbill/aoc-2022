use aoc_2022::DailyProblem;

pub struct TreetopTreeHouse;

impl DailyProblem for TreetopTreeHouse {
    fn name(&self) -> &str { "Day 8: Treetop Tree House "}
    fn index(&self) -> u8 { 8 }
    fn solutions(&self, input: &str) -> (String, String) {
	let mut trees = TreeGrid::new(input);
	trees.scan();
	(trees.count_visible_trees().to_string(), trees.highest_scenic_score().to_string())
    }
}

struct Tree {
    height: u8,
    visible_from_top: bool,
    visible_from_left: bool,
    visible_from_right: bool,
    visible_from_bottom: bool,
    scenic_score: u32,
}

impl Tree {
    fn new(input: char) -> Self {
	Tree{
	    height: input.to_string().parse().unwrap(),
	    visible_from_top: false,
	    visible_from_left: false,
	    visible_from_right: false,
	    visible_from_bottom: false,
	    scenic_score: 0,
	}
    }

    fn visible(&self) -> bool {
	self.visible_from_bottom ||
	    self.visible_from_left ||
	    self.visible_from_right ||
	    self.visible_from_top
    }
}

struct TreeGrid {
    trees: Vec<Vec<Tree>>,
}

impl TreeGrid {
    fn new(input: &str) -> Self {
	let mut trees = vec![];
	for line in input.lines() {
	    let mut tree_line = vec![];
	    for c in line.chars() {
		tree_line.push(Tree::new(c));
	    }
	    trees.push(tree_line);
	}
	TreeGrid{ trees }
    }

    fn count_visible_trees(&self) -> u32 {
	let mut count = 0;
	for line in self.trees.iter() {
	    for tree in line {
		if tree.visible() { count += 1 };
	    }
	}
	count
    }

    fn highest_scenic_score(&self) -> u32 {
	self.trees.iter().map(|l| l.iter().map(|t| t.scenic_score).max().unwrap()).max().unwrap()
    }

    fn height_at(&self, x: usize, y: usize) -> u8 {
	self.tree_at(x, y).height
    }
	
    fn tree_at(&self, x: usize, y: usize) -> &Tree {
	self.trees.get(y).unwrap().get(x).unwrap()
    }

    fn scan_scenic_score(&mut self) {
	for y in 0..self.trees.len() {
	    for x in 0..self.trees.first().unwrap().len() {
		let tree_height = self.height_at(x, y);

		let mut view_above = 0;
		if y > 0 {
		    for view_y in (0..y).rev() {
			view_above += 1;
			let target_height = self.height_at(x, view_y);
			if target_height >= tree_height {
			    break;
			}
		    }
		}

		let mut view_below = 0;
		for view_y in y+1..self.trees.len() {
		    view_below += 1;
		    let target_height = self.height_at(x, view_y);
		    if target_height >= tree_height {
			break;
		    }
		}

		let mut view_left = 0;
		if x > 0 {
		    for view_x in (0..x).rev() {
			view_left += 1;
			let target_height = self.height_at(view_x, y);
			if target_height >= tree_height {
			    break;
			}
		    }
		}

		let mut view_right = 0;
		for view_x in x+1..self.trees.first().unwrap().len() {
		    view_right += 1;
		    let target_height = self.height_at(view_x, y);
		    if target_height >= tree_height {
			break;
		    }
		}

		self.trees.get_mut(y).unwrap().get_mut(x).unwrap().scenic_score =
		    view_above * view_below * view_right * view_left;
		//print!("{}, {}, {}, {} ({}) | ", view_above, view_below, view_left, view_right, 
		//self.trees.get_mut(y).unwrap().get_mut(x).unwrap().scenic_score);
	    }
	    //println!();
	}
    }

    fn scan(&mut self) {
	self.scan_top();
	self.scan_bottom();
	self.scan_left();
	self.scan_right();
	self.scan_scenic_score();
    }

    fn scan_top(&mut self) {
	for column_number in 0..self.trees.first().unwrap().len() {
	    let mut height = self.trees.first().unwrap().get(column_number).unwrap().height;
	    self.trees.first_mut().unwrap().get_mut(column_number).unwrap().visible_from_top = true;
	    for line in self.trees.iter_mut() {
		let tree = line.get_mut(column_number).unwrap();
		if tree.height > height {
		    height = tree.height;
		    tree.visible_from_top = true;
		}
	    }
	}
    }

    fn scan_left(&mut self) {
	for line in self.trees.iter_mut() {
	    let mut height = line.first().unwrap().height;
	    line.first_mut().unwrap().visible_from_left = true;
	    for tree in line.iter_mut() {
		if tree.height > height {
		    height = tree.height;
		    tree.visible_from_left = true;
		}
	    }
	}
    }

    fn scan_right(&mut self) {
	for line in self.trees.iter_mut() {
	    let mut height = line.last().unwrap().height;
	    line.last_mut().unwrap().visible_from_right = true;
	    for tree in line.iter_mut().rev() {
		if tree.height > height {
		    height = tree.height;
		    tree.visible_from_right = true;
		}
	    }
	}
    }

    fn scan_bottom(&mut self) {
	for column_number in 0..self.trees.first().unwrap().len() {
	    let mut height = self.trees.last().unwrap().get(column_number).unwrap().height;
	    self.trees.last_mut().unwrap().get_mut(column_number).unwrap().visible_from_bottom = true;
	    for line in self.trees.iter_mut().rev() {
		let tree = line.get_mut(column_number).unwrap();
		if tree.height > height {
		    height = tree.height;
		    tree.visible_from_bottom = true;
		}
	    }
	}
    }
}

#[cfg(test)]
mod tests {
    use super::TreeGrid;
    #[test]
    fn scenic_scores() {
	let mut trees = TreeGrid::new("30373
25512
65332
33549
35390");
	trees.scan();
	assert_eq!(trees.tree_at(0, 0).height, 3);
	assert_eq!(trees.tree_at(3, 0).height, 7);
	assert_eq!(trees.tree_at(2, 1).scenic_score, 4);
	assert_eq!(trees.tree_at(2, 3).scenic_score, 8);
	assert_eq!(2, 1);
    }
}
