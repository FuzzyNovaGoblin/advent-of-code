use std::{collections::HashMap, fs};

pub fn day14_1(file_name: &str) -> impl std::fmt::Debug {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    let (mut data_code, pair_conversion) = {
        let both = _data
            .unwrap()
            .split("\n\n")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        (
            both[0].chars().collect::<Vec<char>>(),
            both[1]
                .split("\n")
                .map(|line| {
                    let parts = line.split(" -> ").collect::<Vec<_>>();
                    let f_part = parts[0].chars().collect::<Vec<_>>();
                    ((f_part[0], f_part[1]), parts[1].chars().next().unwrap())
                })
                .collect::<HashMap<(char, char), char>>(),
        )
    };

    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for c in data_code.iter() {
        let e = char_counts.entry(*c).or_default();
        *e += 1;
    }

    for _ in 0..10 {
        let mut next_code = Vec::new();
        next_code.push(data_code[0]);
        for c in data_code
            .iter()
            .map(|c| *c)
            .zip(data_code.iter().skip(1).map(|c| *c))
        {
            next_code.push(pair_conversion[&c]);
            let e = char_counts.entry(pair_conversion[&c]).or_default();
            *e += 1;
            next_code.push(c.1);
        }
        data_code = next_code;
    }

    char_counts.iter().reduce(|largest, current| if current.1 > largest.1{current}else{largest}).unwrap().1-
    char_counts.iter().reduce(|smallest, current| if current.1 < smallest.1{current}else{smallest}).unwrap().1

}

pub fn day14_2(file_name: &str) -> impl std::fmt::Debug {
  let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    let (mut data_code, pair_conversion) = {
        let both = _data
            .unwrap()
            .split("\n\n")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        (
            both[0].chars().collect::<Vec<char>>(),
            both[1]
                .split("\n")
                .map(|line| {
                    let parts = line.split(" -> ").collect::<Vec<_>>();
                    let f_part = parts[0].chars().collect::<Vec<_>>();
                    ((f_part[0], f_part[1]), parts[1].chars().next().unwrap())
                })
                .collect::<HashMap<(char, char), char>>(),
        )
    };

    let mut char_counts: HashMap<char, u32> = HashMap::new();

    for c in data_code.iter() {
        let e = char_counts.entry(*c).or_default();
        *e += 1;
    }

    for i in 0..40 {
dbg!(i);
        let mut next_code = Vec::new();
        next_code.push(data_code[0]);
        for c in data_code
            .iter()
            .map(|c| *c)
            .zip(data_code.iter().skip(1).map(|c| *c))
        {
            next_code.push(pair_conversion[&c]);
            let e = char_counts.entry(pair_conversion[&c]).or_default();
            *e += 1;
            next_code.push(c.1);
        }
        data_code = next_code;
    }

    char_counts.iter().reduce(|largest, current| if current.1 > largest.1{current}else{largest}).unwrap().1-
    char_counts.iter().reduce(|smallest, current| if current.1 < smallest.1{current}else{smallest}).unwrap().1
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_dbgfmt;

    #[test]
    fn t1() {
        assert_eq_dbgfmt!(1588, day14_1("test"));
    }
    #[test]
    fn t2() {
        assert_eq_dbgfmt!(2188189693529_u64, day14_2("test"));
    }
}
