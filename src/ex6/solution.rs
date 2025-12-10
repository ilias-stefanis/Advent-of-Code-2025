use crate::SolveSolution;
use core::num;
use std::error::Error;
use std::ops::RangeInclusive;
use std::{fs, vec};

pub struct Ex6;

#[derive(Clone, Debug)]
struct NumberList {
    numbers: Vec<usize>,
    operation: Operation,
    length: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Operation {
    Add,
    Mult,
    Uninit,
}

impl Default for NumberList {
    fn default() -> Self {
        NumberList {
            numbers: Vec::with_capacity(100),
            operation: Operation::Uninit,
            length: 0,
        }
    }
}

impl SolveSolution for Ex6 {
    #[hotpath::measure]
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let number_lists = deserialize("./src/ex6/dataset2.txt")?;

        let sum: usize = number_lists
            .iter()
            .map(|el| {
                if el.operation == Operation::Add {
                    el.numbers.iter().sum::<usize>()
                } else {
                    el.numbers.iter().product()
                }
            })
            .sum();

        Ok(sum.to_string())
    }

    #[hotpath::measure]
    fn solve_2() -> Result<String, Box<dyn Error>> {
        let number_lists = deserialize("./src/ex6/dataset2.txt")?;

        let sum: u128 = number_lists
            .iter()
            .map(|el| {
                if el.operation == Operation::Add {
                    let mut sum = 0;
                    for i in 0..el.length {
                        let mut digit = el
                            .numbers
                            .iter()
                            .enumerate()
                            .map(|(idx, digit)| {
                                let picked_index = el.length - idx + 1;

                                let digit = pick_digit(*digit, i).unwrap_or(0);
                                (digit * 10_usize.pow((picked_index) as u32)) as u128
                            })
                            .sum::<u128>();

                        while digit % 10 == 0 {
                            digit /= 10;
                        }
                        sum += digit;
                    }
                    sum
                } else {
                    let mut prod = 1;
                    for i in 0..el.length {
                        let mut digit = el
                            .numbers
                            .iter()
                            .enumerate()
                            .map(|(idx, digit)| {
                                let picked_index = el.length - idx + 1;

                                let digit = pick_digit(*digit, i).unwrap_or(0);
                                (digit * 10_usize.pow((picked_index) as u32)) as u128
                            })
                            // .inspect(|temp_prod| {
                            //     dbg!(temp_prod);
                            // })
                            .sum::<u128>();

                        while digit % 10 == 0 && digit != 0 {
                            digit /= 10;
                        }

                        prod *= digit;
                    }
                    prod
                }
            })
            .sum::<u128>();

        Ok(sum.to_string())
    }
}

#[hotpath::measure]
fn deserialize(file_name: &str) -> Result<Vec<NumberList>, Box<dyn Error>> {
    let data: String = fs::read_to_string(file_name)?;

    let re = regex::Regex::new(r"\d+\s?")?;
    let mut number_lists: Vec<NumberList> = Vec::new();

    let last_line = data.lines().rev().next().unwrap().split(" ");

    // let mut curr_list = NumberList::default();
    let mut is_reading = false;
    let mut curr_op = Operation::Mult;
    let mut curr_count = 0;
    for char in last_line {
        match char {
            "*" => {
                if !is_reading {
                    is_reading = true;
                    curr_op = Operation::Mult;
                    curr_count = 1;
                    continue;
                }

                // curr_count += 1;
                number_lists.push(NumberList {
                    numbers: vec![],
                    operation: curr_op,
                    length: curr_count,
                });

                // is_reading = false;
                curr_op = Operation::Mult;
                curr_count = 1;
            }
            "+" => {
                if !is_reading {
                    is_reading = true;
                    curr_op = Operation::Add;
                    curr_count = 1;
                    continue;
                }
                // curr_count += 1;
                number_lists.push(NumberList {
                    numbers: vec![],
                    operation: curr_op,
                    length: curr_count,
                });

                // is_reading = false;
                curr_op = Operation::Add;
                curr_count = 1;
            }
            ch if ch.is_empty() => {
                curr_count += 1;
            }
            _ => unreachable!("arent supposed to get a number here"),
        }
    }
    number_lists.push(NumberList {
        numbers: vec![],
        operation: curr_op,
        length: curr_count,
    });

    for line in data.lines() {
        if line.starts_with('*') || line.starts_with('+') {
            continue;
        }

        let mut slots: Vec<String> = Vec::new();
        for token in line.split(' ') {
            if token.is_empty() {
                slots.push(String::new());
            } else if token.chars().count() == 1 {
                slots.push(token.to_string());
            } else {
                for ch in token.chars() {
                    slots.push(ch.to_string());
                }
            }
        }

        let mut slot_idx = 0_usize;

        for arr in number_lists.iter_mut() {
            let length_needed = arr.length;
            let mut value: usize = 0;

            for pos_in_group in 0..length_needed {
                let s = if slot_idx < slots.len() {
                    &slots[slot_idx]
                } else {
                    ""
                };

                if !s.is_empty() {
                    if let Some(d) = s.chars().next().and_then(|c| c.to_digit(10)) {
                        let exp = (length_needed - pos_in_group - 1) as u32;
                        value += (d as usize) * 10_usize.pow(exp);
                    } else {
                        return Err(format!("Non-digit character in data slot: {}", s).into());
                    }
                }

                slot_idx += 1;
            }

            arr.numbers.push(value);
        }
    }

    let number_lists = number_lists
        .into_iter()
        .filter(|n| !matches!(n.operation, Operation::Uninit))
        .collect();

    Ok(number_lists)
}

fn pick_digit(num: usize, digit: usize) -> Option<usize> {
    let power_of_10: usize = 10usize.checked_pow(digit as u32)?;

    if num < power_of_10 {
        return None;
    }
    let picked_digit = num / power_of_10 % 10;
    Some(picked_digit)
}
