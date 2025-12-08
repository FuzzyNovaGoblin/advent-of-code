use std::fs;

#[derive(Debug, Clone, Copy)]
enum MathOp {
    Addition,
    Multiplication,
}

impl<S: AsRef<str>> From<S> for MathOp {
    fn from(value: S) -> Self {
        match value.as_ref() {
            "+" => MathOp::Addition,
            "*" => MathOp::Multiplication,
            v => unreachable!("found:{}", v),
        }
    }
}

#[derive(Debug)]
struct MathProblem {
    numbers: Vec<u64>,
    operation: MathOp,
}

// #[derive(Debug)]
// struct NewMathProblem {
//     numbers: Vec<u64>,
//     operation: MathOp,
// }

// impl NewMathProblem {
//     fn solve(&self) -> u64 {
//         match self.operation {
//             MathOp::Addition => self.numbers.iter().sum(),
//             MathOp::Multiplication => self.numbers.iter().product(),
//         }
//     }
// }

// impl From<&MathProblem> for NewMathProblem {
//     fn from(value: &MathProblem) -> Self {
//         let mut new_numbers = Vec::new();

//         dbg!(&value.numbers);

//         let num_strs = value
//             .numbers
//             .iter()
//             .map(u64::to_string)
//             .map(|s| s.chars().collect::<Vec<_>>())
//             .collect::<Vec<_>>();
//         let max_len = num_strs.iter().map(Vec::len).max().unwrap();

//         for i in 0..max_len {
//             new_numbers.push(
//                 num_strs
//                     .iter()
//                     .filter_map(|ns| ns.get(i))
//                     .fold(0, |acc, c| acc * 10 + c.to_string().parse::<u64>().unwrap()),
//             );
//         }
//         NewMathProblem {
//             numbers: dbg!(new_numbers),
//             operation: value.operation,
//         }
//     }
// }

impl MathProblem {
    fn new(numbers: Vec<u64>, operation: MathOp) -> Self {
        Self { numbers, operation }
    }
    fn solve(&self) -> u64 {
        match self.operation {
            MathOp::Addition => self.numbers.iter().sum(),
            MathOp::Multiplication => self.numbers.iter().product(),
        }
    }
}

pub fn day6_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let lines = data
        .lines()
        .map(|l| l.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let operands = lines.len() - 1;

    let mut problems = Vec::new();
    for i in 0..lines[0].len() {
        problems.push(MathProblem::new(
            (0..operands)
                .into_iter()
                .map(|j| (lines[j][i]).parse::<u64>().unwrap())
                .collect(),
            MathOp::from(lines[operands][i]),
        ));
    }

    problems.iter().map(|p| p.solve()).sum::<u64>()
}

pub fn day6_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let data = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut operators = data.last().unwrap().iter().filter_map(|c| match c {
        '*' => Some(MathOp::Multiplication),
        '+' => Some(MathOp::Addition),
        _ => None,
    });

    let mut problems = Vec::new();
    let mut nums = Vec::new();
    for i in 0..(data.iter().map(Vec::len).max().unwrap()) {
        let mut s = String::new();
        for j in 0..data.len() - 1 {
            // print!("{}", data[j].get(i).unwrap_or(&' '));
            if let Some(c) = data[j].get(i) {
                s.push(*c);
            }
        }
        if s.trim().len() == 0 {
            problems.push(MathProblem::new(nums.clone(), operators.next().unwrap()));
            nums = Vec::new();
        } else {
            nums.push(dbg!(s).trim().parse::<u64>().unwrap());
        }
        // println!()
    }
    problems.push(MathProblem::new(nums.clone(), operators.next().unwrap()));
    dbg!(&problems);
    problems.iter().map(MathProblem::solve).sum::<u64>()

    // let operands = lines.len() - 1;

    // let mut problems = Vec::new();
    // for i in 0..lines[0].len() {
    //     problems.push(MathProblem::new(
    //         (0..operands)
    //             .into_iter()
    //             .map(|j| (lines[j][i]).parse::<u64>().unwrap())
    //             .collect(),
    //         MathOp::from(lines[operands][i]),
    //     ));
    // }

    // let t = problems.iter().collect::<Vec<_>>();
    // // let t = (problems.iter().map(|p| NewMathProblem::from(p))).collect::<Vec<_>>();
    // dbg!(t);
    // problems
    //     .iter()
    //     .map(|p| NewMathProblem::from(p))
    //     .map(|p| p.solve())
    //     .sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!("6295830249262", day6_1("day6"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!(9194682052782_u64, day6_2("day6"));
    }
}
