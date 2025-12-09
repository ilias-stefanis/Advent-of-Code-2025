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
        let number_lists = deserialize("./src/ex6/dataset1.txt")?;
        // dbg!(&number_lists);
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
        let mut sum = 0;

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
        dbg!(&char);
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

    println!("{:?}", &number_lists);

    for line in data.lines() {
        let mut numbers_iter = number_lists.iter_mut();

        if line.starts_with('*') || line.starts_with('+') {
            continue;
        }

        let symbols = line.split(" ");

        let mut arr = numbers_iter.next().unwrap();
        let mut length_needed = arr.length as u32;
        let mut curr_length = 0;
        let mut curr_number = 0;

        for symbol in symbols {
            dbg!(&symbol);
            match symbol.parse::<usize>() {
                Ok(digit) => {
                    if count_digits(digit) == length_needed - curr_length {
                        let digit = digit * 10_usize.pow(length_needed);
                        arr.numbers.push(digit);
                    } else {
                        curr_number = digit;
                    }
                }
                Err(_) if symbol.is_empty() => {
                    if length_needed == 0 {
                    } else {
                        curr_length += 1;
                    }
                }
                Err(e) => return Err(format!("Bad parsing: {e}").into()),
            };
        }
    }
    // dbg!(&symbol);

    let number_lists = number_lists
        .into_iter()
        .filter(|n| !matches!(n.operation, Operation::Uninit))
        .collect();

    Ok(number_lists)
}

#[hotpath::measure]
fn deserialize_part2(file_name: &str) -> Result<Vec<NumberList>, Box<dyn Error>> {
    let data: String = fs::read_to_string(file_name)?;

    let re = regex::Regex::new(r"\d+\s?")?;
    let mut number_lists: Vec<NumberList> = Vec::new();

    let last_line = data.lines().rev().next().unwrap().split(" ");

    // let mut curr_list = NumberList::default();
    let mut is_reading = false;
    let mut curr_op = Operation::Mult;
    let mut curr_count = 0;
    for char in last_line {
        dbg!(&char);
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

    println!("{:?}", &number_lists);

    let mut columns: Vec<Vec<Option<u32>>> = vec![];

    for (y, line) in data.lines().enumerate() {
        if line.starts_with('*') || line.starts_with('+') {
            continue;
        }

        let mut arr: Vec<Option<u32>> = Vec::with_capacity(100);

        for (x, char) in line.chars().enumerate() {
            arr.push(match char {
                ' ' => None,
                s => s.to_digit(10),
            });
        }

        columns.push(arr);
    }
    // dbg!(&symbol);

    let number_lists = number_lists
        .into_iter()
        .filter(|n| !matches!(n.operation, Operation::Uninit))
        .collect();

    Ok(number_lists)
}

fn count_digits(n: usize) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}
