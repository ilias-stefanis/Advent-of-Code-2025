use crate::SolveSolution;
use std::error::Error;
use std::fs;
pub struct Ex10;

#[derive(Debug, Clone)]
struct Machine {
    final_state: Vec<bool>,
    length: usize,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl SolveSolution for Ex10 {
    #[hotpath::measure]
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let machines = deserialize("./src/ex10/dataset1.txt")?;
        dbg!("Machines: {:?}", &machines);
        let mut max_area = 0;

        Ok(max_area.to_string())
    }

    #[hotpath::measure]
    fn solve_2() -> Result<String, Box<dyn Error>> {
        let machines = deserialize("./src/ex10/dataset1.txt")?;

        let mut max_area = 0;

        Ok(max_area.to_string())
    }
}

fn deserialize(file_name: &str) -> Result<Vec<Machine>, Box<dyn Error>> {
    let data = fs::read_to_string(file_name)?;
    let mut machines: Vec<Machine> = vec![];

    let final_state_re = regex::Regex::new(r"(\.|#)+")?;
    let button_schematics_re = regex::Regex::new(r"(\((\d+(\,)?)+\))+")?;
    let joltage_schematics_re = regex::Regex::new(r"\{(\d+(\,)?)+\}")?;

    for line in data.lines() {
        let final_state: Vec<bool> = final_state_re
            .captures(line)
            .ok_or("Bad format")?
            .get(0)
            .ok_or("Bad format")?
            .as_str()
            .chars()
            .map(|c| c == '#')
            .collect();

        let button_schematics_caps = button_schematics_re
            .captures_iter(line)
            .map(|cap| {
                cap.get(0)
                    .ok_or("Bad format")?
                    .as_str()
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|num_str| num_str.parse::<usize>().map_err(|e| e.into()))
                    .collect::<Result<Vec<usize>, Box<dyn Error>>>()
            })
            .collect::<Result<Vec<Vec<usize>>, Box<dyn Error>>>()?;

        let joltage_caps = joltage_schematics_re
            .captures(line)
            .ok_or("Bad format")?
            .get(0)
            .ok_or("Bad format")?
            .as_str()
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|num_str| num_str.parse::<usize>().map_err(|e| e.into()))
            .collect::<Result<Vec<usize>, Box<dyn Error>>>()?;

        machines.push(Machine {
            length: final_state.len(),
            final_state: final_state,
            buttons: button_schematics_caps,
            joltage: joltage_caps,
        });
    }

    Ok(machines)
}
