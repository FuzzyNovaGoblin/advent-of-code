use std::{
    collections::{HashMap, HashSet},
    fs, fmt::Debug,
};

struct Data {
    pub input: Vec<HashSet<char>>,
    pub output: Vec<HashSet<char>>,
    pub num_map: HashMap<u32, HashSet<char>>,
    pub wire_map: HashMap<char, char>,
    pub debug_output: Vec<String>,
}

const SETUP_0: [char; 6] = ['a', 'b', 'c', 'e', 'f', 'g'];
const SETUP_1: [char; 2] = ['c', 'f'];
const SETUP_2: [char; 5] = ['a', 'c', 'd', 'e', 'g'];
const SETUP_3: [char; 5] = ['a', 'c', 'd', 'f', 'g'];
const SETUP_4: [char; 4] = ['b', 'c', 'd', 'f'];
const SETUP_5: [char; 5] = ['a', 'b', 'd', 'f', 'g'];
const SETUP_6: [char; 6] = ['a', 'b', 'd', 'e', 'f', 'g'];
const SETUP_7: [char; 3] = ['a', 'c', 'f'];
const SETUP_8: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
const SETUP_9: [char; 6] = ['a', 'b', 'c', 'd', 'f', 'g'];

impl Data {
    fn new(input: Vec<String>, output: Vec<String>) -> Self {
        let debug_output = output.clone();
        Self {
            input: input
                .into_iter()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .collect(),
            output: output
                .into_iter()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .collect(),
            debug_output,
            num_map: Default::default(),
            wire_map: Default::default(),
        }
    }

    fn string_to_num(&self, input: &HashSet<char>) -> u32 {
        for (val, num_conversion) in &self.num_map {
            if input.symmetric_difference(num_conversion).next() == None {
                return *val;
            }
        }
        return 10;
    }
}

pub fn day8_1() -> impl Debug {
    let input_file = format!("{}/aofc_2021/input/day8", env!("ADVENT_OF_CODE_2021"));
    let _data = fs::read_to_string(input_file);
    let data = _data
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut i_and_o = line
                .split(" | ")
                .map(|part| part.split(" ").map(|s| s.to_owned()).collect())
                .collect::<Vec<_>>();
            let (o, i) = (i_and_o.pop().unwrap(), i_and_o.pop().unwrap());
            Data::new(i, o)
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for data_line in data {
        for o in data_line.output {
            if o.len() == 4 || o.len() == 2 || o.len() == 3 || o.len() == 7 {
                count += 1;
            }
        }
    }
    count
}

pub fn day8_2() -> u32 {
    let input_file = format!("{}/aofc_2021/input/day8", env!("ADVENT_OF_CODE_2021"));
    let _data = fs::read_to_string(input_file);
    let mut data = _data
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut i_and_o = line
                .split(" | ")
                .map(|part| part.split(" ").map(|s| s.to_owned()).collect())
                .collect::<Vec<_>>();
            let (o, i) = (i_and_o.pop().unwrap(), i_and_o.pop().unwrap());
            Data::new(i, o)
        })
        .collect::<Vec<_>>();

    let mut out_sum = 0;
    for data_line in &mut data {

        for i in &mut data_line.input {
            let key = match i.len() {
                4 => 4,
                2 => 1,
                3 => 7,
                7 => 8,
                _ => continue,
            };

            for c in i.iter() {
                data_line.num_map.entry(key).or_insert(Default::default());
                data_line.num_map.get_mut(&key).unwrap().insert(*c);
            }
        }

        data_line.wire_map.insert(
            'a',
            *data_line.num_map[&7]
                .difference(&data_line.num_map[&1])
                .next()
                .unwrap(),
        );

        {
            let mut a_count = 0;
            let mut b_count = 0;
            let c_and_f = data_line.num_map[&1].iter().collect::<Vec<_>>();
            for num_set in &mut data_line.input {
                if num_set.contains(c_and_f[0]) {
                    a_count += 1;
                }
                if num_set.contains(c_and_f[1]) {
                    b_count += 1;
                }
            }
            if a_count > b_count {
                data_line.wire_map.insert('f', *c_and_f[0]);
                data_line.wire_map.insert('c', *c_and_f[1]);
            } else {
                data_line.wire_map.insert('c', *c_and_f[0]);
                data_line.wire_map.insert('f', *c_and_f[1]);
            }
        }

        {
            let b_and_d = data_line.num_map[&4]
                .iter()
                .filter(|&c| *c != data_line.wire_map[&'c'] && *c != data_line.wire_map[&'f'])
                .collect::<Vec<_>>();
            let mut a_count = 0;
            let mut b_count = 0;
            for num_set in &mut data_line.input {
                if num_set.contains(b_and_d[0]) {
                    a_count += 1;
                }
                if num_set.contains(b_and_d[1]) {
                    b_count += 1;
                }
            }
            if a_count > b_count {
                data_line.wire_map.insert('d', *b_and_d[0]);
                data_line.wire_map.insert('b', *b_and_d[1]);
            } else {
                data_line.wire_map.insert('b', *b_and_d[0]);
                data_line.wire_map.insert('d', *b_and_d[1]);
            }
        }

        {
            let tmp = data_line
                .wire_map
                .values()
                .into_iter()
                .map(|c| *c)
                .collect::<HashSet<_>>();

            let e_and_g = data_line.num_map[&8]
                .symmetric_difference(&tmp)
                .collect::<Vec<_>>();

            let mut a_count = 0;
            let mut b_count = 0;
            for num_set in &mut data_line.input {
                if num_set.contains(e_and_g[0]) {
                    a_count += 1;
                }
                if num_set.contains(e_and_g[1]) {
                    b_count += 1;
                }
            }
            if a_count > b_count {
                data_line.wire_map.insert('g', *e_and_g[0]);
                data_line.wire_map.insert('e', *e_and_g[1]);
            } else {
                data_line.wire_map.insert('e', *e_and_g[0]);
                data_line.wire_map.insert('g', *e_and_g[1]);
            }
        }

        data_line
            .num_map
            .insert(0, SETUP_0.iter().map(|c| data_line.wire_map[c]).collect());
        data_line
            .num_map
            .insert(1, SETUP_1.iter().map(|c| data_line.wire_map[c]).collect());
        data_line
            .num_map
            .insert(2, SETUP_2.iter().map(|c| data_line.wire_map[c]).collect());
        data_line
            .num_map
            .insert(3, SETUP_3.iter().map(|c| data_line.wire_map[c]).collect());
        data_line
            .num_map
            .insert(4, SETUP_4.iter().map(|c| data_line.wire_map[c]).collect());
        data_line
            .num_map
            .insert(5, SETUP_5.iter().map(|c| data_line.wire_map[c]).collect());
        data_line
            .num_map
            .insert(6, SETUP_6.iter().map(|c| data_line.wire_map[c]).collect());
        data_line
            .num_map
            .insert(7, SETUP_7.iter().map(|c| data_line.wire_map[c]).collect());
        data_line
            .num_map
            .insert(8, SETUP_8.iter().map(|c| data_line.wire_map[c]).collect());
        data_line
            .num_map
            .insert(9, SETUP_9.iter().map(|c| data_line.wire_map[c]).collect());

        let mut out_num = 0;
        for out_digit in &data_line.output {
            out_num *= 10;
            out_num += data_line.string_to_num(out_digit);
        }
        out_sum += out_num;
    }

    out_sum
}
