use std::fs;

#[derive(Debug)]
struct Bank {
    digits: Vec<u64>,
}

//0123456789
//4082506572

// biggest is at 2
// 10 -2 = 8
// 8 - 4 - 1 = 3
// end is 3

impl Bank {
    fn new<S: AsRef<str>>(digits: S) -> Self {
        Self {
            digits: digits
                .as_ref()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect(),
        }
    }

    fn get_largest_poses(&self) -> (usize, usize, Option<usize>) {
        let mut largest = 0;
        let mut second = 1;
        let mut after_largest = Some(1);
        if self.digits[1] > self.digits[0] {
            largest = 1;
            second = 0;
            after_largest = None;
        }

        for (i, &d) in self.digits.iter().enumerate() {
            if i <= 1 {
                continue;
            }
            if d > self.digits[largest] {
                second = largest;
                largest = i;
                after_largest = None;
            } else {
                if d > self.digits[second] {
                    second = i;
                    after_largest = Some(i);
                } else if let Some(val) = after_largest {
                    if self.digits[val] < d {
                        after_largest = Some(i)
                    }
                } else {
                    after_largest = Some(i);
                }
            }
        }

        (largest, second, after_largest)
    }

    /// [`start`] is the index of first valid element
    /// [`end`] is not inclusive, the index of last valid element plus 1
    fn get_largest_in_range(&self, start: usize, end: usize) -> (usize, u64) {
        println!("range: {:?}", &self.digits[start..end]);
        // let largest_pos = start;
        let range_len = end - start;
        self.digits
            .iter()
            .enumerate()
            .skip(start)
            .take(range_len)
            .fold((0, 0), |acc, val| {
                // dbg!(acc, val);
                if *val.1 > acc.1 { (val.0, *val.1) } else { acc }
            })
        // for &num in self.digits[start+1..end].iter().{

        // }
        // largest_pos
    }

    fn get_nwide_joltage(&self, width: u64) -> u64 {
        let mut collected = Vec::new();
        let mut last_collected_pos: i32 = -1;

        let needed = width as usize - collected.len();

        // let tmp = self.get_largest_in_range(0, self.digits.len() - needed);
        // collected.push(tmp.1);
        // last_collected_pos = tmp.0 as i32;
        while collected.len() < width as usize {
            // dbg!(&collected);

            let (new_pos, new_val) = self.get_largest_in_range(
                (last_collected_pos + 1) as usize,
                self.digits.len() - (width as usize - collected.len()-1),
            );
            collected.push(new_val);
            last_collected_pos = new_pos as i32;
        }
        // start, end
        // end = available - needed_nums - 1
        // available = len - last_collected_pos

        dbg!(
            collected
                .iter()
                .fold((0, 0), |acc, &val| {
                    // dbg!(val);
                    // dbg!(width - acc.0-1);
                    // dbg!(10_u64.pow((width - acc.0-1) as u32));
                    (acc.0+1, val * (10_u64.pow((width - acc.0-1) as u32)) + acc.1)
                })
                .1
        )
    }
}

pub fn day3_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    // let data = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";

    // let banks = data.lines().map(Bank::new).collect::<Vec<_>>();
    data.lines()
        .map(Bank::new)
        .map(|b| {
            let pos_list = b.get_largest_poses();
            // dbg!(&pos_list);
            // println!("{:?}", b.digits);
            match pos_list.2 {
                Some(v) => b.digits[pos_list.0] * 10 + b.digits[v],
                None => b.digits[pos_list.1] * 10 + b.digits[pos_list.0],
            }
        })
        .sum::<u64>()
}

pub fn day3_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    // let data = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";
    // let data = "4212222344622233366212256216226628462122221625532232326322455222225242622256243112325252252221222242";

    data.lines()
        .map(Bank::new)
        .map(|b| b.get_nwide_joltage(12))
        .sum::<u64>()

    // let banks = data.lines().map(Bank::new).collect::<Vec<_>>();
    // let test = banks[0].get_largest_in_range(0, banks[0].digits.len() - 12 - 1);
    // test
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day3_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day3_2("test"));
    }
}
