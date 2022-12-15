use std::collections::HashMap;

use aoc_2022::DailyProblem;

pub struct NoSpaceLeftOnDevice;

impl DailyProblem for NoSpaceLeftOnDevice {
    fn name(&self) -> &str {
        "Day 7: No Space Left On Device"
    }
    fn index(&self) -> u8 {
        7
    }
    fn solutions(&self, input: &str) -> (String, String) {
        let dirs = &parse_tree(input);
        (
            directories_under(dirs, 100000).to_string(),
            directory_to_delete(dirs, 70000000, 30000000).to_string(),
        )
    }
}

fn parse_tree<'a>(input: &'a str) -> HashMap<String, u32> {
    let mut directories: HashMap<String, u32> = HashMap::new();
    directories.insert("".to_string(), 0);
    let mut context = vec![""];
    for line in input.lines() {
        match InputLine::new(line).unwrap() {
            InputLine::CdRoot => {
                context.clear();
                context.push("");
            }
            InputLine::CdDir(name) => {
                context.push(name);
            }
            InputLine::CdUp => {
                context.pop();
            }
            InputLine::Ls => {}
            InputLine::File(size, _) => {
                let dir = directories.get_mut(&context.join("/")).unwrap();
                *dir += size;
                for index in 0..context.len() - 1 {
                    let dir = directories.get_mut(&context[0..=index].join("/")).unwrap();
                    *dir += size;
                }
            }
            InputLine::Dir(name) => {
                let mut path = context.join("/");
                path.push_str("/");
                path.push_str(name);
                directories.insert(path, 0);
            }
        }
    }
    directories
}

fn directories_under(dirs: &HashMap<String, u32>, size_limit: u32) -> u32 {
    dirs.iter()
        .filter(|d| *d.1 <= size_limit)
        .map(|d| d.1)
        .sum()
}

fn directory_to_delete(dirs: &HashMap<String, u32>, total_size: u32, target_size: u32) -> u32 {
    let free_space = total_size - dirs.get("").unwrap();
    let space_to_delete = target_size - free_space;
    *dirs
        .iter()
        .filter(|d| d.1.to_owned() >= space_to_delete)
        .map(|d| d.1)
        .min()
        .unwrap()
}

#[derive(Debug)]
enum InputLine<'a> {
    CdRoot,
    Ls,
    File(u32, &'a str),
    Dir(&'a str),
    CdUp,
    CdDir(&'a str),
}

impl<'a> InputLine<'a> {
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
            file_size => Ok(InputLine::File(
                file_size.parse().unwrap(),
                words.next().unwrap(),
            )),
        }
    }
}
