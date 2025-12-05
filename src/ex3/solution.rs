use crate::SolveSolution;
use std::error::Error;
use std::fs;
pub struct Ex3;

#[derive(Debug)]
struct Battery(Vec<u32>);

impl SolveSolution for Ex3 {
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let batteries = deserialize_to_struct("./src/ex3/dataset2.txt")?;
        // let batteries = deserialize_to_struct("./src/ex3/temp.txt")?;

        let mut sum = 0;

        for battery in batteries {
            let battery = battery.0;

            // dbg!(&battery);

            #[derive(Debug)]
            struct BatteryDigit {
                digit: u32,
                index: usize,
            };

            let mut sorted_array: Vec<_> = battery
                .into_iter()
                .enumerate()
                .map(|(index, digit)| BatteryDigit { index, digit })
                .collect();

            sorted_array.sort_by(|a, b| {
                if a.digit == b.digit {
                    return a.index.cmp(&b.index).reverse();
                }

                a.digit.cmp(&b.digit)
            });

            let max_digit = sorted_array.last().unwrap();

            let fitting_2nd_max = sorted_array
                .iter()
                .rev()
                .find(|d| d.index > max_digit.index);

            dbg!(&sorted_array);
            // dbg!(&max_digit);
            // dbg!(&fitting_2nd_max);

            if let Some(_2nd_max) = fitting_2nd_max {
                let digit = max_digit.digit * 10 + _2nd_max.digit;
                sum += digit;
                println!("---");
                continue;
            }

            let max_digit = &sorted_array[sorted_array.len() - 2];
            let fitting_2nd_max = sorted_array
                .iter()
                .rev()
                .find(|d| d.index > max_digit.index);

            dbg!(&max_digit);
            dbg!(&fitting_2nd_max);
            println!("---");

            if let Some(_2nd_max) = fitting_2nd_max {
                let digit = max_digit.digit * 10 + _2nd_max.digit;
                sum += digit;
                continue;
            }
        }

        Ok(sum.to_string())
    }

    fn solve_2() -> Result<String, Box<dyn Error>> {
        let mut instructions = deserialize_to_struct("./src/ex3/dataset2.txt")?;

        let mut sum = 0;

        for battery in &mut instructions {
            let mut product = 0;
            for len in (1usize..=12).rev() {
                let (start, n) = get_best_greedy(battery.0.as_slice(), len).unwrap();
                product = product * 10 + u64::from(n);
                battery.0 = battery.0[start + 1..].to_vec();
            }
            sum += product;
        }

        Ok(sum.to_string())
    }
}

fn get_best_greedy(batteries: &[u32], len: usize) -> Option<(usize, u32)> {
    batteries[..=(batteries.len() - len)]
        .iter()
        .copied()
        .enumerate()
        .rev()
        .max_by_key(|(_, digit)| *digit)
}

fn deserialize_to_struct(file_name: &str) -> Result<Vec<Battery>, Box<dyn Error>> {
    let data: String = fs::read_to_string(file_name)?;

    let re = regex::Regex::new(r"(\d+)").unwrap();
    let mut instructions = Vec::new();

    for (_, [digit]) in re.captures_iter(data.as_str()).map(|c| c.extract::<1>()) {
        let battery: Vec<u32> = digit.chars().map(|c| c.to_digit(10).unwrap()).collect();

        instructions.push(Battery(battery));
    }

    Ok(instructions)
}
