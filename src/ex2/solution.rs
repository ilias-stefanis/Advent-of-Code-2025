use crate::SolveSolution;
use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;
pub struct Ex2;

#[derive(Debug)]
struct IdsRange((isize, isize));

impl SolveSolution for Ex2 {
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let instructions = deserialize_to_struct("./src/ex2/dataset2.txt")?;

        let mut counter = 0;

        for ins in instructions
        // .chunks(100).skip(1).next().unwrap()
        {
            let (start, end) = ins.0;
            counter += find_invalid_of_range(start..=end)
        }

        Ok(counter.to_string())
    }

    fn solve_2() -> Result<String, Box<dyn Error>> {
        let instructions = deserialize_to_struct("./src/ex2/dataset2.txt")?;

        let mut sum = 0;

        let mut string_buffer = String::with_capacity(20);

        for ins in instructions
        // .chunks(100).skip(1).next().unwrap()
        {
            let (start, end) = ins.0;

            for num in start..=end {
                string_buffer.clear();
                let num_as_string = num.to_string();
                string_buffer.push_str(&num_as_string);
                string_buffer.push_str(&num_as_string);

                if string_buffer[1..string_buffer.len() - 1].contains(&num_as_string) {
                    sum += num;
                }
            }
        }

        Ok(sum.to_string())
    }
}

fn find_invalid_of_range(range: RangeInclusive<isize>) -> isize {
    let mut sum = 0;
    for num in range {
        let length = num.checked_ilog10().unwrap_or(0).div_ceil(2);
        let left_side = num.strict_div(10_isize.pow(length));
        let right_side = num % (10_isize.pow(length));

        if left_side == right_side {
            sum += num;
        }
    }

    sum
}

fn deserialize_to_struct(file_name: &str) -> Result<Vec<IdsRange>, Box<dyn Error>> {
    let data: String = fs::read_to_string(file_name)?;

    let re = regex::Regex::new(r"(\d+)(?:-)(\d+)").unwrap();
    let mut instructions = Vec::new();

    for (_, [start, end]) in re.captures_iter(data.as_str()).map(|c| c.extract::<2>()) {
        let start = start.parse::<isize>()?;
        let end = end.parse::<isize>()?;

        instructions.push(IdsRange((start, end)));
    }

    Ok(instructions)
}
