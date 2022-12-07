use std::{cell::RefCell, fmt::Debug, fs, ops::Deref, rc::Rc};

#[derive(Clone)]
enum FsEntry {
    Dir {
        parent: Option<Rc<FsEntry>>,
        children: RefCell<Vec<Rc<FsEntry>>>,
        name: String,
    },
    File {
        parent: Rc<FsEntry>,
        size: usize,
        name: String,
    },
}

impl Debug for FsEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parent = self.get_parent().map(|e| e.get_name().to_owned());
        match self {
            Self::Dir {
                parent: _,
                children,
                name,
            } => f
                .debug_struct("Dir")
                .field("parent", &parent)
                .field("children", children)
                .field("name", name)
                .finish(),
            Self::File {
                parent: _,
                size,
                name,
            } => f
                .debug_struct("File")
                .field("parent", &parent)
                .field("size", size)
                .field("name", name)
                .finish(),
        }
    }
}

impl FsEntry {
    fn get_parent(&self) -> Option<Rc<FsEntry>> {
        match self {
            FsEntry::Dir {
                parent,
                name: _,
                children: _,
            } => parent.clone(),
            FsEntry::File {
                parent,
                name: _,
                size: _,
            } => Some(parent.clone()),
        }
    }

    fn get_children(&self) -> Vec<Rc<FsEntry>> {
        match self {
            FsEntry::Dir {
                parent: _,
                name: _,
                children,
            } => children.borrow().deref().clone(),
            FsEntry::File {
                parent: _,
                name: _,
                size: _,
            } => Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        match self {
            FsEntry::Dir {
                parent: _,
                children: _,
                name,
            } => name,
            FsEntry::File {
                parent: _,
                size: _,
                name,
            } => name,
        }
    }

    fn add_child(&self, child: Rc<FsEntry>) {
        match self {
            FsEntry::Dir {
                parent: _,
                children,
                name: _,
            } => children.borrow_mut().push(child),
            FsEntry::File {
                parent: _,
                size: _,
                name: _,
            } => (),
        }
    }

    fn get_size(&self) -> usize {
        match self {
            FsEntry::Dir {
                parent: _,
                children,
                name: _,
            } => children.borrow().iter().map(|c| c.get_size()).sum(),
            FsEntry::File {
                parent: _,
                size,
                name: _,
            } => *size,
        }
    }

    fn get_dir_sizes(&self) -> Vec<(String, usize)> {
        let mut ret = vec![(self.get_name().to_owned(), self.get_size())];
        for child in self.get_children() {
            if let FsEntry::Dir {
                parent: _,
                children: _,
                name: _,
            } = child.clone().deref()
            {
                ret.extend(child.get_dir_sizes());
            }
        }
        ret
    }
}

#[derive(Debug)]
enum InputLine {
    Command(Command),
    Dir(String),
    File(String, usize),
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

fn build_root_dir(data: String) -> Rc<FsEntry> {
    let input_lines = data
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
                .nth(1)
                .map(|dest| InputLine::Dir(dest.into())),
            Some(_) => {
                let parts = line.split(' ').collect::<Vec<_>>();
                match parts[0].parse::<usize>() {
                    Ok(val) => Some(InputLine::File(parts[1].into(), val)),
                    Err(_) => None,
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let root_dir = Rc::new(FsEntry::Dir {
        parent: None,
        children: RefCell::new(Vec::new()),
        name: "/".into(),
    });
    let mut current_dir = root_dir.clone();

    for line in input_lines.iter() {
        match line {
            InputLine::Command(cmd) => match cmd {
                Command::Cd(dir) => {
                    if dir == ".." {
                        current_dir = current_dir.get_parent().unwrap();
                    }
                    if dir == "/" {
                        current_dir = root_dir.clone();
                    } else {
                        for c in current_dir.get_children() {
                            if c.get_name() == dir {
                                current_dir = c;
                                break;
                            }
                        }
                    }
                }
                Command::Ls => (),
            },
            InputLine::Dir(dir_name) => {
                let new_dir = Rc::new(FsEntry::Dir {
                    parent: Some(current_dir.clone()),
                    children: RefCell::new(Vec::new()),
                    name: dir_name.to_owned(),
                });
                current_dir.add_child(new_dir);
            }
            InputLine::File(file_name, size) => {
                let new_file = Rc::new(FsEntry::File {
                    parent: current_dir.clone(),
                    size: *size,
                    name: file_name.clone(),
                });
                current_dir.add_child(new_file);
            }
        }
    }

    root_dir
}

pub fn day7_1(file_name: &str) -> impl crate::AnsType {
    const AT_MOST_SIZE: usize = 100000;

    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let root_dir = build_root_dir(data);

    root_dir
        .get_dir_sizes()
        .into_iter()
        .filter_map(|(_, v)| if v <= AT_MOST_SIZE { Some(v) } else { None })
        .sum::<usize>()
}

pub fn day7_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );

    const TOTAL_SPACE: usize = 70000000;
    const NEEDED_SPACE: usize = 30_000_000;

    let data = fs::read_to_string(input_file).unwrap();
    let root_dir = build_root_dir(data);
    let need_to_delete = NEEDED_SPACE - (TOTAL_SPACE - root_dir.get_size());

    let mut dir_sizes = root_dir
        .get_dir_sizes()
        .into_iter()
        .filter_map(|(_, size)| {
            if size >= need_to_delete {
                Some(size)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    dir_sizes.sort();
    dir_sizes[0]
}
