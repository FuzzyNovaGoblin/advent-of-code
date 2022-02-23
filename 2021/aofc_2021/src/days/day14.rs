use std::{
    cell::RefCell,
    collections::HashMap,
    default::default,
    fs,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

pub fn day14_1(file_name: &str) -> impl crate::AnsType  {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    let (mut data_code, pair_conversion) = {
        let both = _data
            .unwrap()
            .split("\n\n")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        (
            both[0].chars().collect::<Vec<char>>(),
            both[1]
                .split('\n')
                .map(|line| {
                    let parts = line.split(" -> ").collect::<Vec<_>>();
                    let f_part = parts[0].chars().collect::<Vec<_>>();
                    ((f_part[0], f_part[1]), parts[1].chars().next().unwrap())
                })
                .collect::<HashMap<(char, char), char>>(),
        )
    };

    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for c in data_code.iter() {
        let e = char_counts.entry(*c).or_default();
        *e += 1;
    }

    for _ in 0..10 {
        let mut next_code = vec![data_code[0]];
        for c in data_code
            .iter().copied()
            .zip(data_code.iter().skip(1).copied())
        {
            next_code.push(pair_conversion[&c]);
            let e = char_counts.entry(pair_conversion[&c]).or_default();
            *e += 1;
            next_code.push(c.1);
        }
        data_code = next_code;
    }


    char_counts
        .iter()
        .reduce(|largest, current| {
            if current.1 > largest.1 {
                current
            } else {
                largest
            }
        })
        .unwrap()
        .1
        - char_counts
            .iter()
            .reduce(|smallest, current| {
                if current.1 < smallest.1 {
                    current
                } else {
                    smallest
                }
            })
            .unwrap()
            .1
}

type AtomicFinalConversion = Arc<Mutex<HashMap<(u8, (char, char)), HashMap<char, usize>>>>;

fn rec_get_conversion(
    pair: (char, char),
    depth: u8,
    max_depth: u8,
    pair_conversion: Arc<HashMap<(char, char), char>>,
    final_conversions: AtomicFinalConversion,
) -> HashMap<char, usize> {
    if depth >= max_depth {
        return default();
    }
    {

            if let Ok(mg_fc) = final_conversions.lock() {
                if mg_fc.contains_key(&(depth, pair)) {
                    return mg_fc[&(depth, pair)].clone();
                }
            }
    }

    let mut char_count = HashMap::new();
    let new_char = pair_conversion[&pair];
    {
        let ent = char_count.entry(new_char).or_insert(0);
        *ent += 1;
    }
    let mut threads = Vec::new();
    {
        let pair_conversion = pair_conversion.clone();
        let final_conversions = final_conversions.clone();
        threads.push(thread::spawn(move || {
            rec_get_conversion(
                (pair.0, new_char),
                depth + 1,
                max_depth,
                pair_conversion,
                final_conversions,
            )
        }));
    }
    if depth >= 3 {
        while let Some(t) = threads.pop() {
            match t.join() {
                Ok(m) => for (c, count) in m {
                let ent = char_count.entry(c).or_insert(0);
                *ent += count;
            },
                Err(e) => {eprintln!("error: {:?}", e); panic!("error")},
            }

        }
    }
    {
        let final_conversions = final_conversions.clone();
        threads.push(thread::spawn(move || {
            rec_get_conversion(
                (new_char, pair.1),
                depth + 1,
                max_depth,
                pair_conversion,
                final_conversions,
            )
        }));
    }
    while let Some(t) = threads.pop() {
        for (c, count) in t.join().expect("thread failed") {
            let ent = char_count.entry(c).or_insert(0);
            *ent += count;
        }
    }

    {
        if let Ok(mut mg_fc) = final_conversions.lock() {
            mg_fc.insert((depth, pair), char_count.clone());
        }
    }

    char_count
}

pub fn day14_2(file_name: &str) -> impl crate::AnsType  {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    let (data_code, pair_conversion) = {
        let both = _data
            .unwrap()
            .split("\n\n")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        (
            both[0].chars().collect::<Vec<char>>(),
            both[1]
                .split('\n')
                .map(|line| {
                    let parts = line.split(" -> ").collect::<Vec<_>>();
                    let f_part = parts[0].chars().collect::<Vec<_>>();
                    ((f_part[0], f_part[1]), parts[1].chars().next().unwrap())
                })
                .collect::<HashMap<(char, char), char>>(),
        )
    };

    let pair_conversion = Arc::new(pair_conversion);
    let final_conversions = Arc::new(Mutex::new(HashMap::new()));
    let char_counts: Rc<RefCell<HashMap<char, usize>>> = Rc::new(RefCell::new(HashMap::new()));


    {
        let mut char_counts = char_counts.borrow_mut();
        let  ent = char_counts.entry(data_code[0]).or_default();
        *ent += 1;
    }

    let mut threads = Vec::new();
    for c in data_code
        .iter()
        .copied()
        .zip(data_code.clone().iter().skip(1).copied())
    {
        let pair_conversion = pair_conversion.clone();
        let final_conversions = final_conversions.clone();
        threads.push(thread::spawn(move || {
            rec_get_conversion(c, 0, 40, pair_conversion, final_conversions)
        }));
        let mut char_counts = char_counts.borrow_mut();
        let e = char_counts.entry(c.1).or_insert(0);
        *e += 1;
    }

    let mut char_counts = char_counts.borrow_mut();
    for t in threads {
        for (c, count) in t.join().expect("thread failed") {
            let ent = char_counts.entry(c).or_insert(0);
            *ent += count;
        }
    }


    {
        char_counts
            .iter()
            .reduce(|largest, current| {
                if current.1 > largest.1 {
                    current
                } else {
                    largest
                }
            })
            .unwrap()
            .1
            - char_counts
                .iter()
                .reduce(|smallest, current| {
                    if current.1 < smallest.1 {
                        current
                    } else {
                        smallest
                    }
                })
                .unwrap()
                .1
    }
}
