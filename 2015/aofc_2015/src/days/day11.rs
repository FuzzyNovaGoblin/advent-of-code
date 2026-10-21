use std::fs;

fn inc_password(pass: &mut Vec<char>, index: usize) {
    if pass[index] == 'z' {
        pass[index] = 'a';
        inc_password(pass, index - 1);
    } else {
        pass[index] = (pass[index] as u8 + 1) as char;
    }
    if matches!(pass[index], 'i' | 'o' | 'l') {
        pass[index] = match pass[index] {
            'i' => 'j',
            'o' => 'p',
            'l' => 'm',
            _ => unreachable!(),
        };
        for i in index + 1..pass.len() {
            pass[i] = 'a';
        }
    }
}

fn check_valid_password(pass: &[char]) -> bool {
    let mut pairs_seen = 0;
    let mut straight_seen = false;

    for w in pass.windows(3) {
        if matches!(w[0], 'i' | 'o' | 'l')
            || matches!(w[1], 'i' | 'o' | 'l')
            || matches!(w[2], 'i' | 'o' | 'l')
        {
            return false;
        }
        if w[0] == w[1] && w[0] != w[2] {
            pairs_seen += 1;
        }
        if w[0] as u8 == w[1] as u8 - 1 && w[1] as u8 == w[2] as u8 - 1 {
            straight_seen = true;
        }
    }
    if pairs_seen < 2 {
        let last_pair = pass.windows(2).last().unwrap();
        if last_pair[0] == last_pair[1] {
            pairs_seen += 1;
        }
    }

    pairs_seen >= 2 && straight_seen
}

pub fn day11_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let mut data = fs::read_to_string(input_file)
        .unwrap()
        .trim()
        .chars()
        .collect::<Vec<_>>();
    let data_len = data.len();
    loop {
        if check_valid_password(&data) {
            break;
        }
        inc_password(&mut data, data_len - 1);
    }
    data.iter().collect::<String>()
}

pub fn day11_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let mut data = fs::read_to_string(input_file)
        .unwrap()
        .trim()
        .chars()
        .collect::<Vec<_>>();
    let data_len = data.len();
    loop {
        if check_valid_password(&data) {
            break;
        }
        inc_password(&mut data, data_len - 1);
    }
    inc_password(&mut data, data_len - 1);
    loop {
        if check_valid_password(&data) {
            break;
        }
        inc_password(&mut data, data_len - 1);
    }

    data.iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day11_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day11_2("test"));
    }
}
