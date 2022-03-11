use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Default, Debug)]
struct Graph<'a> {
    connections: HashMap<Node<'a>, Vec<Node<'a>>>,
}

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node<'a>(&'a str);
impl<'a> Graph<'a> {
    fn add_connection(&mut self, a: Node<'a>, b: Node<'a>) {
        self.connections.entry(a).or_insert(vec![]);
        self.connections.get_mut(&a).unwrap().push(b);
    }

    fn find_paths(
        &self,
        next_node: Node<'a>,
        mut path_string: String,
        mut been_to: HashMap<Node<'a>, u8>,
        mut used_double: bool,
    ) -> Vec<String> {
        if next_node.0 == "end" {
            path_string.push_str("end");
            return vec![path_string];
        } else {
            path_string.push_str(next_node.0);
            path_string.push(',');
        }
        if next_node.0.chars().next().unwrap().is_lowercase() {
            let b_to = been_to.entry(next_node).or_insert(0);
            *b_to += 1;
            if let Some(_tmp @ 2) = been_to.get(&next_node) {
                used_double = true;
            }
        }

        let mut ret_vec = Vec::new();

        for new_node in self.connections.get(&next_node).unwrap() {
            if new_node.0 == "start" {
                continue;
            }
            match been_to.get(new_node) {
                Some(&v) if v == 1 && used_double => continue,
                Some(&v) if v == 2 => continue,
                Some(&v) if v > 2 => panic!("should not be larger than 2. node: {:?}", new_node),
                _ => (),
            }

            self.find_paths(
                *new_node,
                path_string.clone(),
                been_to.clone(),
                used_double,
            )
            .into_iter()
            .for_each(|v| ret_vec.push(v));
        }

        ret_vec
    }

    fn find_paths_part1(
        &self,
        next_node: Node<'a>,
        mut path_string: String,
        mut been_to: HashSet<Node<'a>>,
    ) -> Vec<String> {
        if next_node.0 == "end" {
            path_string.push_str("end");
            return vec![path_string];
        } else {
            path_string.push_str(next_node.0);
            path_string.push(',');
        }
        if next_node.0.chars().next().unwrap().is_lowercase() {
            been_to.insert(next_node);
        }

        let mut ret_vec = Vec::new();

        for new_node in self.connections.get(&next_node).unwrap() {
            if been_to.contains(new_node) {
                continue;
            }

            self.find_paths_part1(*new_node, path_string.clone(), been_to.clone())
                .into_iter()
                .for_each(|v| ret_vec.push(v));
        }

        ret_vec
    }
}

pub fn day12_1(file_name: &str) -> usize {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        std::env::var("ADVENT_OF_CODE_2021").unwrap(),
        file_name
    );
    let _data = fs::read_to_string(input_file).unwrap();
    let mut data = Graph::default();
    for line in _data.split('\n') {
        let split_line = line.split('-').collect::<Vec<&str>>();
        data.add_connection(Node(split_line[0]), Node(split_line[1]));
        data.add_connection(Node(split_line[1]), Node(split_line[0]));
    }
    let paths = data.find_paths_part1(Node("start"), "".into(), Default::default());
    paths.len()
}

pub fn day12_2(file_name: &str) -> usize {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        std::env::var("ADVENT_OF_CODE_2021").unwrap(),
        file_name
    );
    let _data = fs::read_to_string(input_file).unwrap();
    let mut data = Graph::default();
    for line in _data.split('\n') {
        let split_line = line.split('-').collect::<Vec<&str>>();
        data.add_connection(Node(split_line[0]), Node(split_line[1]));
        data.add_connection(Node(split_line[1]), Node(split_line[0]));
    }
    let mut been_to = HashMap::new();
    been_to.insert(Node("start"), 2);
    let paths = data.find_paths(Node("start"), "".into(), been_to, false);
    paths.len()
}
