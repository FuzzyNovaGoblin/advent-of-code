use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

fn distance_between_pair(a: usize, b: usize, distances: &Vec<Vec<usize>>) -> usize {
    let first = min(a, b);
    let last = max(a, b);
    distances[first][last - first - 1]
}

fn distance_of_path(path: [usize; 8], distances: &Vec<Vec<usize>>) -> usize {
    path.windows(2)
        .map(|w| distance_between_pair(w[0], w[1], distances))
        .sum()
}

fn find_shortest_path(distances: &Vec<Vec<usize>>) -> usize {
    let mut shortest = distance_of_path([0, 1, 2, 3, 4, 5, 6, 7], distances);
    for a in 0..8 {
        for b in 0..8 {
            if b == a {
                continue;
            }
            for c in 0..8 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..8 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 0..8 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        for f in 0..8 {
                            if f == a || f == b || f == c || f == d || f == e {
                                continue;
                            }
                            for g in 0..8 {
                                if g == a || g == b || g == c || g == d || g == e || g == f {
                                    continue;
                                }
                                for h in 0..8 {
                                    if h == a
                                        || h == b
                                        || h == c
                                        || h == d
                                        || h == e
                                        || h == f
                                        || h == g
                                    {
                                        continue;
                                    }
                                    let new_path =
                                        distance_of_path([a, b, c, d, e, f, g, h], distances);
                                    if new_path < shortest {
                                        shortest = new_path;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    shortest
}

fn find_longest_path(distances: &Vec<Vec<usize>>) -> usize {
    let mut longest = distance_of_path([0, 1, 2, 3, 4, 5, 6, 7], distances);
    for a in 0..8 {
        for b in 0..8 {
            if b == a {
                continue;
            }
            for c in 0..8 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..8 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 0..8 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        for f in 0..8 {
                            if f == a || f == b || f == c || f == d || f == e {
                                continue;
                            }
                            for g in 0..8 {
                                if g == a || g == b || g == c || g == d || g == e || g == f {
                                    continue;
                                }
                                for h in 0..8 {
                                    if h == a
                                        || h == b
                                        || h == c
                                        || h == d
                                        || h == e
                                        || h == f
                                        || h == g
                                    {
                                        continue;
                                    }
                                    let new_path =
                                        distance_of_path([a, b, c, d, e, f, g, h], distances);
                                    if new_path > longest {
                                        longest = new_path;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    longest
}

pub fn day9_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let mut place_names = HashSet::new();
    let mut places = Vec::new();
    let mut distances = Vec::new();

    let _data = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|l| {
            l.split(' ')
                .filter_map(|part| {
                    if part != "to" && part != "=" {
                        Some(part.to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .for_each(|line| {
            if place_names.insert(line[0].clone()) {
                places.push(line[0].clone());
                distances.push(vec![line[2].parse::<usize>().unwrap()]);
            } else {
                distances
                    .last_mut()
                    .unwrap()
                    .push(line[2].parse::<usize>().unwrap());
            }
        });

    find_shortest_path(&distances)
}

pub fn day9_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let mut place_names = HashSet::new();
    let mut places = Vec::new();
    let mut distances = Vec::new();

    let _data = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|l| {
            l.split(' ')
                .filter_map(|part| {
                    if part != "to" && part != "=" {
                        Some(part.to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .for_each(|line| {
            if place_names.insert(line[0].clone()) {
                places.push(line[0].clone());
                distances.push(vec![line[2].parse::<usize>().unwrap()]);
            } else {
                distances
                    .last_mut()
                    .unwrap()
                    .push(line[2].parse::<usize>().unwrap());
            }
        });

    find_longest_path(&distances)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day9_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day9_2("test"));
    }
}
