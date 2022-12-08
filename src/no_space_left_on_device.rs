// use std::collections::HashMap;

use aoc_2022::DailyProblem;

pub struct NoSpaceLeftOnDevice;

impl DailyProblem for NoSpaceLeftOnDevice {
    fn name(&self) -> &str { "Day 7: No Space Left On Device" }
    fn index(&self) -> u8 { 7 }
    fn solutions(&self, _input: &str) -> (String, String) {
	// println!("{:?}", Directory::new(input).unwrap());
	((-1).to_string(), (-1).to_string())
    }
}

/* #[derive(Debug)]
struct Directory <'a> {
    parent: Option<&'a Directory<'a>>,
    contents: HashMap<&'a str, DirectoryListing<'a>>,
}

impl <'a> Directory<'a> {
    fn new(input: &'a str) -> Result<Self, String> {
	let mut root_directory = Self{ parent: None, contents: HashMap::new() };
	let mut current_directory = &mut root_directory;
	for line in input.lines() {
	    match InputLine::new(line).unwrap() {
		InputLine::CdRoot => { current_directory = &mut root_directory }
		InputLine::CdDir(name) => {
		    current_directory = match current_directory.contents.get_mut(name).unwrap() {
			DirectoryListing::File(_) => panic!(),
			DirectoryListing::Directory(d) => d,
		    };
		}
		InputLine::Ls => { },
		InputLine::File(size, name) => {
		    let cd = current_directory;
		    current_directory.contents.insert(name, DirectoryListing::File(File{ parent: cd, size }));
		},
		InputLine::Dir(name) => {
		    let cd = &mut (*current_directory).contents;
		    cd.insert(name, DirectoryListing::Directory(Directory{ parent: Some(current_directory), contents: HashMap::new() }));
		},
		e => todo!("{:?}",e)
	    }
	}
	Ok(root_directory)
    }
}

#[derive(Debug)]
struct File <'a> {
    parent: &'a Directory<'a>,
    size: u32,
}

#[derive(Debug)]
enum DirectoryListing<'a> {
    File(File<'a>),
    Directory(Directory<'a>),
}*/
/*
impl FromStr for DirectoryListing {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, ()> {
	let root_directory = DirectoryListing::Directory(HashMap::new(), None);
	let mut current_directory = &root_directory;
	for line in input.lines() {
	    match InputLine::from_str(line).unwrap() {
		InputLine::CdRoot => { current_directory = &root_directory; }
		InputLine::CdDir(dir_name) => {
		    match current_directory {
			DirectoryListing::File(_, _) => panic!("Can't cd into a file"),
			DirectoryListing::Directory(map, _) => { current_directory = map.get(&dir_name).expect(&format!("No directory named {}", dir_name)); },
		    }
		},
		InputLine::CdUp => {
		    match current_directory {
			DirectoryListing::File(_, parent) => { current_directory = &parent.as_ref().unwrap() },
			DirectoryListing::Directory(_, parent) => { current_directory = &parent.as_ref().unwrap() },
		    }
		},
		InputLine::Ls => { },
		InputLine::File(size, name) => {
		    match current_directory {
			DirectoryListing::File(_, _) => panic!("Can't list a file in a file"),
			DirectoryListing::Directory(map, _) => { map.insert(name, DirectoryListing::File(size, Some(Box::new(current_directory)))); },
		    }
		},
		InputLine::Dir(name) => { },
	    }
	}
	Ok(root_directory)
    }
}*/

/*
#[derive(Debug)]
enum InputLine<'a> {
    CdRoot,
    Ls,
    File(u32, &'a str),
    Dir(&'a str),
    CdUp,
    CdDir(&'a str),
}

impl <'a> InputLine<'a> {
    fn new(input: &'a str) -> Result<Self, String> {
	let mut words = input.split(' ');
	match words.next().unwrap() {
	    "$" => match words.next().unwrap() {
		"cd" => match words.next().unwrap() {
		    ".." => Ok(InputLine::CdUp),
		    "/" => Ok(InputLine::CdRoot),
		    dir_name => Ok(InputLine::CdDir(dir_name)),
		},
		"ls" => Ok(InputLine::Ls),
		command => Err(format!("Invalid command {}", command)),
	    },
	    "dir" => Ok(InputLine::Dir(words.next().unwrap())),
	    file_size => Ok(InputLine::File(file_size.parse().unwrap(), words.next().unwrap())),
	}
    }
}
*/
