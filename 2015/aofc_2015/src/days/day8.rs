use std::fs;

fn convert_escapes<S: AsRef<str>>(line: S) -> String {
    let chars = line.as_ref().chars().collect::<Vec<_>>();
    let mut working = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            if chars[i + 1] == 'x' {
                working.push('*');
                i += 3;
            } else {
                working.push(chars[i + 1]);
                i += 1;
            }
        } else {
            working.push(chars[i]);
        }
        i += 1;
    }

    working.iter().collect()
}

fn escape_line<S: AsRef<str>>(line: S) -> String {
    format!(
        "\"{}\"",
        line.as_ref()
            .chars()
            .map(|c| match c {
                '\\' => vec!['\\', '\\'],
                '"' => vec!['\\', '"'],
                c => vec![c],
            })
            .flatten()
            .collect::<String>()
    )
}

pub fn day8_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let og_char_count = data.lines().map(|l| l.len()).sum::<usize>();

    let after_char_count = data
        .clone()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            assert_eq!(&l[0..1], "\"");
            assert_eq!(&l[l.len() - 1..l.len()], "\"");
            &l[1..l.len() - 1]
        })
        .map(|l| convert_escapes(l))
        .map(|l| l.len())
        .sum::<usize>();
    // .for_each(|l| println!("`{}` {}", l, l.len()))

    og_char_count - after_char_count
}

pub fn day8_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let og_char_count = data.lines().map(|l| l.len()).sum::<usize>();
    let after_char_count = data
        .lines()
        .map(|l| escape_line(l))
        .map(|l| l.len())
        .sum::<usize>();

    after_char_count - og_char_count
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day8_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day8_2("test"));
    }
}
