use std::fs;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const NAUGHTY_PAIRS: [[char; 2]; 4] = [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];

pub fn day5_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    data.lines()
        .map(|l| if is_nice_string_part1(l) { 1 } else { 0 })
        .sum::<u32>()
}

fn is_nice_string_part1<S: AsRef<str>>(line: S) -> bool {
    if line.as_ref().is_empty() {
        return false;
    }

    let line = line.as_ref().chars().collect::<Vec<_>>();

    let mut vowel_count = if VOWELS.contains(&line[0]) { 1 } else { 0 };
    let mut double_letter = false;

    for win in line.windows(2) {
        for p in NAUGHTY_PAIRS {
            /* early return for naughty pair */
            if p == win {
                return false;
            }
        }
        if VOWELS.contains(&win[1]) {
            vowel_count += 1
        }
        if win[0] == win[1] {
            double_letter = true;
        }
    }

    vowel_count >= 3 && double_letter
}

pub fn day5_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    data.lines()
        .map(|l| if is_nice_string_part2(l) { 1 } else { 0 })
        .sum::<u32>()
}

fn is_nice_string_part2<S: AsRef<str>>(line: S) -> bool {
    if line.as_ref().len() < 4 {
        return false;
    }

    let line = line.as_ref().chars().collect::<Vec<_>>();

    // let mut vowel_count = if VOWELS.contains(&line[0]) { 1 } else { 0 };
    // let mut double_letter = false;

    repeated_pair(&line) && contains_double_with_space(&line)
}

fn repeated_pair(line: &Vec<char>) -> bool {
    for i in 0..(line.len() - 2) {
        for j in (i + 2)..line.len()-1 {
            if line[i] == line[j] && line[i + 1] == line[j + 1] {
                return true;
            }
        }
    }
    false
}

fn contains_double_with_space(line: &Vec<char>) -> bool {
    for win in line.windows(3) {
        if win[0] == win[2] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day5_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day5_2("test"));
    }
}
