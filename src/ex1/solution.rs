use crate::SolveSolution;
use std::error::Error;
use std::fs;
pub struct Ex1;

#[derive(Debug)]
struct Instruction {
    radius: isize,
    is_clockwise: bool,
}

impl SolveSolution for Ex1 {
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let instructions = deserialize_to_struct("./src/ex1/dataset2.txt")?;

        let mut current_dial = 50;
        let mut counter = 0;

        for ins in instructions
        // .chunks(100).skip(1).next().unwrap()
        {
            let Instruction {
                radius,
                is_clockwise,
            } = ins;

            let direction: isize = if is_clockwise { 1 } else { -1 };
            let temp_dial: isize = direction * radius + current_dial;
            current_dial = temp_dial;
            if current_dial % 100 == 0 {
                counter += 1;
            }
        }

        Ok(counter.to_string())
    }

    fn solve_2() -> Result<String, Box<dyn Error>> {
        let instructions = deserialize_to_struct("./src/ex1/dataset2.txt")?;

        let mut current_dial = 50;

        let mut zero_rotations = 0;

        for ins in instructions
        // .chunks(100).skip(1).next().unwrap()
        {
            let Instruction {
                radius,
                is_clockwise,
            } = ins;

            let direction: isize = if is_clockwise { 1 } else { -1 };

            for _i in 0..radius.abs() {
                current_dial = ((current_dial + direction) % 100 + 100) % 100;
                if current_dial % 100 == 0 {
                    zero_rotations += 1;
                }
            }
        }

        Ok(zero_rotations.to_string())
    }
}

fn deserialize_to_struct(file_name: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let data: String = fs::read_to_string(file_name)?;

    let re = regex::Regex::new(r"(L|R)([0-9]+)").unwrap();
    let mut instructions = Vec::new();

    for (_, [instr, radius]) in re.captures_iter(data.as_str()).map(|c| c.extract::<2>()) {
        let radius = radius.parse::<isize>()?;
        instructions.push(Instruction {
            radius,
            is_clockwise: match instr {
                "L" => false,
                "R" => true,
                _ => return Err(String::from("Wrong format").into()),
            },
        });
    }

    Ok(instructions)
}
