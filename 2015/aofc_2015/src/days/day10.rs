use std::fs;

fn look_and_say(num: &Vec<u8>) -> Vec<u8> {
    let mut ret_parts = Vec::new();

    let mut splits = Vec::new();
    for (i, w) in num.windows(2).enumerate() {
        if w[0] != w[1] {
            splits.push(i);
        }
    }

    if splits.is_empty() {
        ret_parts.push(num.len() as u8);
        ret_parts.push(num[0]);
        return ret_parts;
    }

    // dbg!(&splits);

    let mut prev_s = 0;
    for (i, s) in splits.iter().enumerate() {
        if *s == 0 {
            ret_parts.push(1);
            ret_parts.push(num[*s]);
        } else {
            if i == 0 {
                ret_parts.push(((s + 1) - prev_s) as u8);
            } else {
                ret_parts.push((s - prev_s) as u8);
            }
            ret_parts.push(num[*s]);
        }
        // if s == num.len() - 2 {
        //     ret_parts.push(1);
        //     ret_parts.push(num[s + 1]);
        // }

        prev_s = *s;
    }
    ret_parts.push((num.len() - (prev_s + 1)) as u8);
    ret_parts.push(num[prev_s + 1]);
    // dbg!(&ret_parts);

    ret_parts
}

pub fn day10_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file)
        .unwrap()
        .trim()
        .chars()
        .filter_map(|d| match d.to_string().parse::<u8>() {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect::<Vec<_>>();
    let mut val = data;
    for _ in 0..40 {
        val = look_and_say(&val);
    }

    val.len()
}

pub fn day10_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file)
        .unwrap()
        .trim()
        .chars()
        .filter_map(|d| match d.to_string().parse::<u8>() {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect::<Vec<_>>();
    let mut val = data;
    for _ in 0..50 {
        val = look_and_say(&val);
    }

    val.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day10_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day10_2("test"));
    }
}
