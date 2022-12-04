use std::fs;

pub fn day4_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file);
    data.unwrap()
        .lines()
        .map(|line| {
            let mut pair = line.split(',');
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .map(|(rng1, rng2)| {
            let mut rng1 = rng1.split('-').map(|s| s.parse::<u32>());
            let mut rng2 = rng2.split('-').map(|s| s.parse::<u32>());
            (
                (rng1.next().unwrap().unwrap()..=rng1.next().unwrap().unwrap()),
                (rng2.next().unwrap().unwrap()..=rng2.next().unwrap().unwrap()),
            )
        })
        .filter(|(rng1, rng2)| {
            rng2.contains(rng1.start()) && rng2.contains(rng1.end())
                || rng1.contains(rng2.start()) && rng1.contains(rng2.end())
        })
        .count()
}

pub fn day4_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file);
    data.unwrap()
        .lines()
        .map(|line| {
            let mut pair = line.split(',');
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .map(|(rng1, rng2)| {
            let mut rng1 = rng1.split('-').map(|s| s.parse::<u32>());
            let mut rng2 = rng2.split('-').map(|s| s.parse::<u32>());
            (
                (rng1.next().unwrap().unwrap()..=rng1.next().unwrap().unwrap()),
                (rng2.next().unwrap().unwrap()..=rng2.next().unwrap().unwrap()),
            )
        })
        .filter(|(rng1, rng2)| {
            rng2.contains(rng1.start())
                || rng2.contains(rng1.end())
                || rng1.contains(rng2.start())
                || rng1.contains(rng2.end())
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]

    fn t1() {
        assert_eq_ansval!(2, day4_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day4_2("test"));
    }
}
