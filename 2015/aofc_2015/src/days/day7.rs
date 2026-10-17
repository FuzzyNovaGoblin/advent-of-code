use Logic::*;
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Wire {
    input: Logic,
    output: Option<u16>,
}

impl Wire {
    fn new(input: Logic) -> Self {
        Self {
            output: match &input {
                Logic::Value(v) => Some(*v),
                _ => None,
            },
            input,
        }
    }

    fn from_line<S: AsRef<str>>(line: S) -> Wire {
        let parts = line.as_ref().split(' ').collect::<Vec<_>>();
        // And
        // Or
        // LShift
        // RShift

        if parts.len() == 1 {
            match parts[0].parse::<u16>() {
                Ok(v) => Wire::new(Value(v)),
                Err(_) => Wire::new(SingleWire(parts[0].to_owned())),
            }
        } else if parts[0] == "NOT" {
            Wire::new(Not(LogicUnit::from_line(parts[1].to_owned())))
        } else {
            match parts[1] {
                "AND" => Wire::new(And(
                    LogicUnit::from_line(parts[0]),
                    LogicUnit::from_line(parts[2]),
                )),
                "OR" => Wire::new(Or(
                    LogicUnit::from_line(parts[0]),
                    LogicUnit::from_line(parts[2]),
                )),
                "LSHIFT" => Wire::new(LShift(
                    LogicUnit::from_line(parts[0]),
                    parts[2].parse::<u8>().unwrap(),
                )),
                "RSHIFT" => Wire::new(RShift(
                    LogicUnit::from_line(parts[0]),
                    parts[2].parse::<u8>().unwrap(),
                )),
                v => {
                    eprintln!("at unreachable, value: {v}");
                    unreachable!()
                }
            }
        }
    }

    fn solve_wires(wires: &mut HashMap<String, Wire>) {
        let mut complete = false;
        let keys = wires.keys().cloned().collect::<Vec<_>>();
        while !complete {
            complete = true;
            for key in keys.iter() {
                if wires.get(key).unwrap().output.is_some() {
                    continue;
                }
                complete = false;

                wires.get_mut(key).unwrap().output = wires.get(key).unwrap().input.solve(&wires);
            }
        }
    }
}

#[derive(Debug)]
enum Logic {
    Value(u16),
    SingleWire(String),
    And(LogicUnit, LogicUnit),
    Or(LogicUnit, LogicUnit),
    Not(LogicUnit),
    LShift(LogicUnit, u8),
    RShift(LogicUnit, u8),
}

#[derive(Debug)]
enum LogicUnit {
    Value(u16),
    SingleWire(String),
}

impl LogicUnit {
    fn from_line<S: AsRef<str>>(line: S) -> LogicUnit {
        match line.as_ref().parse::<u16>() {
            Ok(v) => LogicUnit::Value(v),
            Err(_) => LogicUnit::SingleWire(line.as_ref().to_owned()),
        }
    }
}

macro_rules! unwrap_logic_unit {
    ($name:ident, $map:ident) => {
        match $name {
            LogicUnit::Value(v) => Some(*v),
            LogicUnit::SingleWire(name) => $map.get(name).unwrap().output,
        }
    };
}

impl Logic {
    fn solve(&self, wires: &HashMap<String, Wire>) -> Option<u16> {
        match self {
            Value(v) => Some(*v),
            SingleWire(s) => wires.get(s).unwrap().output,
            And(a, b) => match (unwrap_logic_unit!(a, wires), unwrap_logic_unit!(b, wires)) {
                (Some(a), Some(b)) => Some(a & b),
                _ => None,
            },
            // And(a, b) => match (wires.get(a).unwrap().output, wires.get(b).unwrap().output) {
            //     (Some(a), Some(b)) => Some(a & b),
            //     _ => None,
            // },
            Or(a, b) => match (unwrap_logic_unit!(a, wires), unwrap_logic_unit!(b, wires)) {
                (Some(a), Some(b)) => Some(a | b),
                _ => None,
            },
            Not(s) => match unwrap_logic_unit!(s, wires) {
                Some(v) => Some(!v),
                None => None,
            },
            LShift(s, n) => match unwrap_logic_unit!(s, wires) {
                Some(v) => Some(v << n),
                None => None,
            },
            RShift(s, n) => match unwrap_logic_unit!(s, wires) {
                Some(v) => Some(v >> n),
                None => None,
            },
        }
    }
}

pub fn day7_1(file_name: &str) -> u16 {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );

    let mut wires = HashMap::<String, Wire>::new();

    for (line, name) in fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .filter_map(|l| {
            let p = l.split(" -> ").collect::<Vec<_>>();
            if p.len() != 2 {
                None
            } else {
                Some((p[0], p[1]))
            }
        })
    {
        wires.insert(name.to_owned(), Wire::from_line(line));
    }

    /* manual test code */
    /*     wires.insert(String::from("x"), Wire::new(Value(123)));
       wires.insert(String::from("y"), Wire::new(Value(456)));
       wires.insert(
           String::from("d"),
           Wire::new(And(String::from("x"), String::from("y"))),
       );
       wires.insert(
           String::from("e"),
           Wire::new(Or(String::from("x"), String::from("y"))),
       );
       wires.insert(String::from("f"), Wire::new(LShift(String::from("x"), 2)));
       wires.insert(String::from("g"), Wire::new(RShift(String::from("y"), 2)));
       wires.insert(String::from("h"), Wire::new(Not(String::from("x"))));
       wires.insert(String::from("i"), Wire::new(Not(String::from("y"))));

       Wire::solve_wires(&mut wires);

       let mut keys = wires.keys().cloned().collect::<Vec<_>>();
       keys.sort();
       for k in keys {
           let w = wires.get(&k).unwrap();
           println!("{k}: {:?}", w.output)
       }
    */
    Wire::solve_wires(&mut wires);

    wires.get("a").unwrap().output.unwrap()
}

pub fn day7_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );

    let special_value = day7_1(file_name);

    let mut wires = HashMap::<String, Wire>::new();

    for (line, name) in fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .filter_map(|l| {
            let p = l.split(" -> ").collect::<Vec<_>>();
            if p.len() != 2 {
                None
            } else {
                Some((p[0], p[1]))
            }
        })
    {
        wires.insert(name.to_owned(), Wire::from_line(line));
    }

    wires.get_mut("b").unwrap().output = Some(special_value);

    Wire::solve_wires(&mut wires);

    wires.get("a").unwrap().output.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day7_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day7_2("test"));
    }
}
