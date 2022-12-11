use std::os::unix::prelude::CommandExt;
use std::slice::Iter;

/// Day 7
use aoc_2022::day7::{Directory, File, FileObject, FileTree, HasSize};

const DISK_SPACE: u32 = 70_000_000;
const FREE_SPACE_REQ: u32 = 30_000_000;

/// Parse a History and build out a FileObject Structure
pub struct HistoryParser {
    pub tree: FileTree,
}

impl HistoryParser {
    pub fn new() -> Self {
        Self {
            tree: FileTree::default(),
        }
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::LS(output) => self.handle_ls(output),
            Command::CD(path) => self.handle_cd(path),
        }
    }

    fn handle_ls(&mut self, output: LSOutput) {
        for fo in output.files() {
            self.tree.add_file(fo);
        }
    }

    fn handle_cd(&mut self, path: CDPath) {
        match path {
            CDPath::Root => self.tree.move_root(),
            CDPath::Path(name) => self.tree.move_dir(name),
            CDPath::Up => self.tree.move_to_parent(),
        }
    }
}

/// The Command
#[derive(Debug)]
pub enum Command {
    CD(CDPath),
    LS(LSOutput),
}

impl Command {
    pub fn from_str(input: &'static str) -> Self {
        match input[..3].as_ref() {
            " cd" => Command::CD(CDPath::new(&input[3..])),
            " ls" => Command::LS(LSOutput::new(&input[3..])),
            _ => panic!("AoC is a Liar"),
        }
    }
}

#[derive(Debug)]
pub enum CDPath {
    Path(&'static str),
    Up,
    Root,
}

impl CDPath {
    fn new(input: &'static str) -> Self {
        let path = if &input[..3] == " cd" {
            input[3..].trim()
        } else {
            input.trim()
        };

        match path {
            ".." => CDPath::Up,
            "/" => CDPath::Root,
            _ => CDPath::Path(path),
        }
    }
}

#[derive(Debug)]
pub struct LSOutput {
    data: Vec<FileObject>,
}

impl LSOutput {
    pub fn new(input: &'static str) -> Self {
        let data = match input[..3].as_ref() {
            " ls" => Self::parse_output(&input[3..]),
            _ => Self::parse_output(input),
        };
        Self { data }
    }

    fn parse_output(output: &'static str) -> Vec<FileObject> {
        //split the strings
        let mut results = vec![];
        for line in output.lines() {
            if line.is_empty() {
                continue;
            }
            match line[..3].as_ref() {
                "dir" => {
                    // Its a directory
                    let fo = FileObject::Dir(Directory::new(&line[4..]));
                    results.push(fo);
                }
                _ => {
                    // Its a  File
                    let (size, name) = line.split_once(' ').unwrap();
                    let fo = FileObject::File(File::new(name, size.parse().unwrap()));
                    results.push(fo);
                }
            }
        }
        results
    }

    pub fn files(self) -> impl Iterator<Item = FileObject> {
        self.data.into_iter()
    }
}

/// Take in a History of Series of Commands (ls and cd) and their output, And determine certain
/// information pertaining the directory structure and size.
fn main() {
    let command_history = include_str!("./data/day_7.prod.txt");

    let mut parser = HistoryParser::new();
    let commands: Vec<Command> = command_history
        .split('$')
        .skip(1)
        .map(|x| Command::from_str(x))
        .collect();

    for command in commands {
        parser.execute(command)
    }

    dbg!(parser
        .tree
        .sizes()
        .iter()
        .filter(|(_name, x)| *x < 100_000)
        .map(|(_, z)| z)
        .sum::<u32>());

    parser.tree.move_root();
    let unused = dbg!(DISK_SPACE - parser.tree.size());
    let needed = dbg!(FREE_SPACE_REQ - unused);

    dbg!(parser
        .tree
        .sizes()
        .iter()
        .filter(|(_name, x)| *x > needed)
        .map(|(_, z)| z)
        .min());
        // .sum::<u32>());

} 
