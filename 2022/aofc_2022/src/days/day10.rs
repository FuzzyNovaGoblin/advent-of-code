use std::fs;

pub fn day10_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let mut x_reg = 1;
    let mut cycle = 0;

    let mut sum = 0;

    macro_rules! cycle_inc {
        () => {
            cycle += 1;
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => sum += cycle * x_reg,
                _ => (),
            }
        };
    }

    data.lines().for_each(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();

        cycle_inc!();
        if parts[0] == "addx" {
            cycle_inc!();
            x_reg += parts[1].parse::<i32>().unwrap();
        }
    });

    sum
}

pub fn day10_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let mut x_reg = 1;
    let mut cycle = 0;

    macro_rules! cycle_inc {
        () => {
            let crt_pixel = cycle % 40;

            if crt_pixel == x_reg - 1 || crt_pixel == x_reg || crt_pixel == x_reg + 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycle += 1;
            if crt_pixel == 39 {
                println!();
            }
        };
    }

    data.lines().for_each(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();

        cycle_inc!();
        if parts[0] == "addx" {
            cycle_inc!();
            x_reg += parts[1].parse::<i32>().unwrap();
        }
    });
}
