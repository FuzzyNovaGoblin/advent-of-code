use std::{collections::HashMap, fs, rc::Rc};

#[derive(Debug, Clone, Hash)]
enum FsEntry {
    Dir {
        parent: String,
        children: Vec<FsEntry>,
    },
    File {
        parent: String,
        size: usize,
    },
}

impl FsEntry {
    fn get_parent(&self) -> String {
        match self {
            FsEntry::Dir {
                parent,
                children: _,
            } => parent.clone(),
            FsEntry::File { parent, size: _ } => parent.clone(),
        }
    }
}

enum InputLine {
    Command(Command),
    Dir(String),
    File(String, usize),
}

enum Command {
    Ls,
    Cd(String),
}

pub fn day7_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let inputLines = data
        .lines()
        .filter_map(|line| match line.chars().next() {
            Some('$') => {
                let parts = line.split(' ').skip(1).collect::<Vec<_>>();
                match parts[0] {
                    "cd" => Some(InputLine::Command(Command::Cd(parts[1].into()))),
                    "ls" => Some(InputLine::Command(Command::Ls)),
                    _ => None,
                }
            }
            Some('d') => line
                .split(' ')
                .skip(1)
                .next()
                .map(|dest| InputLine::Dir(dest.into())),
            Some(c) => {
                let parts = line.split(' ').collect::<Vec<_>>();
                match parts[0].parse::<usize>() {
                    Ok(val) => Some(InputLine::File(parts[1].into(), val)),
                    Err(_) => None,
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut current_dir = String::new();
    let mut file_map = HashMap::<String, FsEntry>::new();

    let mut i = 0;
    while i < inputLines.len() {
        match &inputLines[i] {
            InputLine::Command(cmd) => match cmd {
                Command::Cd(dir) => {
                    if dir == ".." {
                        current_dir = file_map.get(&current_dir).unwrap().get_parent();
                    } else if file_map.contains_key(dir) {
                        current_dir = dir.to_owned();
                    } else {
                        let new_dir = FsEntry::Dir {
                            parent: current_dir.clone(),
                            children: Vec::new(),
                        };
                        file_map.insert(dir.into(), new_dir);
                        current_dir = dir.to_owned();
                    }
                }
                Command::Ls => (),
            },
            InputLine::Dir(dir_name) => {
                let new_dir = FsEntry::Dir {
                    parent: current_dir.clone(),
                    children: Vec::new(),
                };
                file_map.insert(dir_name.into(), new_dir);
            }
            InputLine::File(file_name, size) => {
                let new_file = FsEntry::File {
                    parent: current_dir.clone(),
                    size: *size,
                };
                file_map.insert(file_name.into(), new_file);
            }
        }
        i += 1;
    }

    println!("{:?}", file_map);

    todo!()
}

pub fn day7_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    todo!()
}
