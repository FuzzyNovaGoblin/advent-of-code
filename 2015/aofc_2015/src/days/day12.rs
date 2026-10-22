use std::fs;

use serde_json::Value;

fn sum_all_digits(data: &Value) -> i64 {
    match data {
        Value::String(_) | Value::Bool(_) | Value::Null => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(values) => values.iter().map(|v| sum_all_digits(&v)).sum(),
        Value::Object(map) => map.values().map(|v| sum_all_digits(&v)).sum(),
    }
}

fn remove_red_objects(data: Value) -> Value {
    match data {
        Value::Object(map) => {
            for v in map.values() {
                if let Value::String(s) = v {
                    if s == "red" {
                        return Value::Null;
                    }
                }
            }
            map.iter()
                .map(|(k, v)| (k, remove_red_objects(v.clone())))
                .collect()
        }
        Value::Array(arr) => arr.iter().map(|v| remove_red_objects(v.clone())).collect(),
        o => o,
    }
}

pub fn day12_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );

    let data: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(input_file).unwrap()).unwrap();
    sum_all_digits(&data)
}

pub fn day12_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(input_file).unwrap()).unwrap();
    let data = remove_red_objects(data);

    sum_all_digits(&data)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day12_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day12_2("test"));
    }
}
