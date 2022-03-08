use std::{
    collections::{HashSet, VecDeque},
    fs,
    io::stdin,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlipRange {
    start: i64,
    end: i64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    x: FlipRange,
    y: FlipRange,
    z: FlipRange,
}

impl Region {
    fn volume(&self) -> i64 {
        ((self.x.end - self.x.start).abs() + 1)
            * ((self.y.end - self.y.start).abs() + 1)
            * ((self.z.end - self.z.start).abs() + 1)
    }
    fn one_dim_overlap(a: &FlipRange, b: &FlipRange) -> FlipRange {
        let mut points = vec![a.end, a.start, b.end, b.start];
        points.sort();
        FlipRange {
            start: points[1],
            end: points[2],
        }
    }

    fn overlap(&self, other: &Region) -> Option<Region> {
        if other.x.end >= self.x.start
            && other.x.start <= self.x.end
            && other.y.end >= self.y.start
            && other.y.start <= self.y.end
            && other.z.end >= self.z.start
            && other.z.start <= self.z.end
        {
            Some(Region {
                x: Region::one_dim_overlap(&self.x, &other.x),
                y: Region::one_dim_overlap(&self.y, &other.y),
                z: Region::one_dim_overlap(&self.z, &other.z),
            })
        } else {
            None
        }
    }
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
    fn remove_chunk(&self, other: &Region) -> Vec<Region> {
        let mut ret_val = Vec::new();
        if other.z.end < self.z.end {
            ret_val.push(Region {
                x: self.x.clone(),
                y: self.y.clone(),
                z: FlipRange {
                    start: other.z.end + 1,
                    end: self.z.end,
                },
            })
        }
        if other.z.start > self.z.start {
            ret_val.push(Region {
                x: self.x.clone(),
                y: self.y.clone(),
                z: FlipRange {
                    start: self.z.start,
                    end: other.z.start - 1,
                },
            })
        }
        if other.x.end < self.x.end {
            ret_val.push(Region {
                x: FlipRange {
                    start: other.x.end + 1,
                    end: self.x.end,
                },
                y: self.y.clone(),
                z: FlipRange {
                    start: self.z.start.max(other.z.start),
                    end: self.z.end.min(other.z.end),
                },
            })
        }
        if other.x.start > self.x.start {
            ret_val.push(Region {
                x: FlipRange {
                    start: self.x.start,
                    end: other.x.start - 1,
                },
                y: self.y.clone(),
                z: FlipRange {
                    start: self.z.start.max(other.z.start),
                    end: self.z.end.min(other.z.end),
                },
            })
        }
        if other.y.end < self.y.end {
            ret_val.push(Region {
                x: FlipRange {
                    start: self.x.start.max(other.x.start),
                    end: self.x.end.min(other.x.end),
                },
                y: FlipRange {
                    start: other.y.end + 1,
                    end: self.y.end,
                },
                z: FlipRange {
                    start: self.z.start.max(other.z.start),
                    end: self.z.end.min(other.z.end),
                },
            })
        }
        if other.y.start > self.y.start {
            ret_val.push(Region {
                x: FlipRange {
                    start: self.x.start.max(other.x.start),
                    end: self.x.end.min(other.x.end),
                },
                y: FlipRange {
                    start: self.y.start,
                    end: other.y.start - 1,
                },
                z: FlipRange {
                    start: self.z.start.max(other.z.start),
                    end: self.z.end.min(other.z.end),
                },
            })
        }

        ret_val
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
    let mut map: HashSet<(i64, i64, i64)> = HashSet::new();
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

#[derive(Default, Debug)]
struct BlockyBlocus {
    blocks: Vec<Region>,
}

fn refine_regine_list_size() {}

pub fn day22_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let mut moves = data
        .split('\n')
        .map(FlipMove::from_str)
        .collect::<VecDeque<_>>();
    let mut blocky_thing_guy = BlockyBlocus::default();

    'mainloop: loop {
        let m = if let Some(v) = moves.pop_front() {
            v
        } else {
            break;
        };
        match m.dir {
            FlipDirection::Off => 'offer: loop {
                for i in 0..blocky_thing_guy.blocks.len() {
                    if let Some(reg) = blocky_thing_guy.blocks[i].overlap(&m.region) {
                        let b = blocky_thing_guy.blocks.remove(i);

                        let new_stuff = b.remove_chunk(&reg);
                        for r in new_stuff {
                            blocky_thing_guy.blocks.push(r);
                        }
                        continue 'offer;
                    }
                }
                break;
            },
            FlipDirection::On => {
                for block in blocky_thing_guy.blocks.iter() {
                    if let Some(reg) = block.overlap(&m.region) {
                        let new_stuff = m.region.remove_chunk(&reg);
                        for r in new_stuff {
                            moves.push_front(FlipMove {
                                dir: FlipDirection::On,
                                region: r,
                            })
                        }
                        continue 'mainloop;
                    }
                }
                blocky_thing_guy.blocks.push(m.region);
            }
        }
    }

    blocky_thing_guy
        .blocks
        .into_iter()
        .fold(0, |f_val, n_val| f_val + n_val.volume())
}
