use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
struct FlipRange {
    start: i32,
    end: i32,
}
#[derive(Debug, Clone)]
struct Region {
    x: FlipRange,
    y: FlipRange,
    z: FlipRange,
}

#[derive(Debug, Clone)]
enum FlipDirection {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct FlipMove {
    dir: FlipDirection,
    region: Region,
}

impl FlipMove {
    fn from_str(string: &str) -> FlipMove {
        let mut parts = string.split(' ');
        FlipMove {
            dir: if let Some("on") = parts.next() {
                FlipDirection::On
            } else {
                FlipDirection::Off
            },
            region: Region::from_str(parts.next().unwrap()),
            // x: FlipMove::from_str(parts.next().unwrap()),
            // y: FlipMove::from_str(parts.next().unwrap()),
            // z: FlipMove::from_str(parts.next().unwrap()),
        }
    }
}

impl Region {
    fn from_str(string: &str) -> Region {
        let mut parts = string.split(',');
        Region {
            x: FlipRange::from_str(parts.next().unwrap()),
            y: FlipRange::from_str(parts.next().unwrap()),
            z: FlipRange::from_str(parts.next().unwrap()),
        }
    }
}

impl FlipRange {
    fn from_str(string: &str) -> FlipRange {
        let mut parts = string[2..].split("..");
        FlipRange {
            start: parts.next().unwrap().parse().unwrap(),
            end: parts.next().unwrap().parse().unwrap(),
        }
    }
}

pub fn day22_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let data = data.split('\n').map(FlipMove::from_str);
    let mut map: HashSet<(i32, i32, i32)> = HashSet::new();
    for (ei, m) in data.enumerate() {
        for x in (-50).max(m.region.x.start)..=50.min(m.region.x.end) {
            for y in (-50).max(m.region.y.start)..=50.min(m.region.y.end) {
                for z in (-50).max(m.region.z.start)..=50.min(m.region.z.end) {
                    match m.dir {
                        FlipDirection::On => {
                            map.insert((x, y, z));
                        }
                        FlipDirection::Off => {
                            map.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }

    map.len()
}

pub fn day22_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    fn t1() {
        assert_eq_ansval!(590784, day22_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day22_2("test"));
    }
}
