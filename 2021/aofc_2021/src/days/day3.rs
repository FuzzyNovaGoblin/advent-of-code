use std::{fmt::Debug, fs, ops};

#[derive(Default, Clone)]
struct BitSet {
    pub bits: Vec<u8>,
}

impl Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BitSet {{{:?}}}", self.bits)
    }
}

impl ops::Index<usize> for BitSet {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

impl ops::IndexMut<usize> for BitSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        while index + 1 > self.bits.len() {
            self.bits.push(0);
        }
        &mut self.bits[index]
    }
}

impl BitSet {
    fn from_str<T>(data: T) -> Self
    where
        T: AsRef<str>,
    {
        let mut chars = data.as_ref().chars().rev().collect::<Vec<_>>();
        let mut bits = Vec::<u8>::new();
        while let Some(v) = chars.pop() {
            if v == '1' {
                bits.push(1);
            } else {
                bits.push(0);
            }
        }
        Self { bits }
    }

    fn to_usize(&self) -> usize {
        let mut retval = 0;
        for n in &self.bits {
            retval <<= 1;
            retval |= *n as usize;
        }
        retval
    }

    fn print_binary(&self) {
        for i in 0..5 {
            print!("{}", self[i]);
        }
        println!();
    }
}

pub fn day3_1 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",env!("ADVENT_OF_CODE_2021"),file_name);
    let data = fs::read_to_string(input_file).unwrap();
    let data = data
        .split("\n")
        .map(|s| BitSet::from_str(s))
        .collect::<Vec<_>>();
    let length = data.len();
    let width = data[0].bits.len();
    let mut ones_count = vec![0; length];

    for bset in data {
        for i in 0..width {
            if bset[i] == 1 {
                ones_count[i] += 1;
            }
        }
    }
    let mut gama = BitSet::default();
    let mut epsilon = BitSet::default();

    for i in 0..width {
        if ones_count[i] > length / 2 {
            gama[i] = 1;
            epsilon[i] = 0;
        } else {
            gama[i] = 0;
            epsilon[i] = 1;
        }
    }
    gama.print_binary();
    epsilon.print_binary();

    gama.to_usize() * epsilon.to_usize()
}

pub fn day3_2 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",env!("ADVENT_OF_CODE_2021"),file_name);
    let data = fs::read_to_string(input_file).unwrap();
    let data = data
        .split("\n")
        .map(|s| BitSet::from_str(s))
        .collect::<Vec<_>>();

    let mut oxygen_vals = data.clone();
    let mut co2_vals = data.clone();

    let mut count = 0;
    while oxygen_vals.len() > 1 {
        let needed_val = {
            let mut num_count = [0; 2];

            for bset in &oxygen_vals {
                if bset[count] == 1 {
                    num_count[1] += 1;
                } else {
                    num_count[0] += 1;
                }
            }
            if num_count[1] >= num_count[0] {
                1
            } else {
                0
            }
        };

        oxygen_vals = oxygen_vals
            .into_iter()
            .filter(|bit_set| bit_set[count] == needed_val)
            .collect();
        count += 1;
    }
    count = 0;

    while co2_vals.len() > 1 {
        let needed_val = {
            let mut num_count = [0; 2];

            for bset in &co2_vals {
                if bset[count] == 1 {
                    num_count[1] += 1;
                } else {
                    num_count[0] += 1;
                }
            }
            if num_count[1] == 0 {
                0
            } else if num_count[0] == 0 {
                1
            } else if num_count[1] < num_count[0] {
                1
            } else {
                0
            }
        };

        co2_vals = co2_vals
            .into_iter()
            .filter(|bit_set| bit_set[count] == needed_val)
            .collect();
        count += 1;
    }
    dbg!(oxygen_vals[0].to_usize());
    dbg!(co2_vals[0].to_usize());
    oxygen_vals[0].to_usize() * co2_vals[0].to_usize()
}
