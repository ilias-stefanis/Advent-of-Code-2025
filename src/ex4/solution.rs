use crate::SolveSolution;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub struct Ex4;

#[derive(Debug, Clone)]
struct Roll {
    pos: GridPosition,
    has_roll: bool,
    matches_ex_1: RefCell<bool>,
    removed: RefCell<bool>,
}

impl std::fmt::Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self.matches_ex_1.borrow() {
            write!(f, "X")
        } else if self.has_roll {
            write!(f, "@")
        } else {
            write!(f, ".")
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct GridPosition {
    x: isize,
    y: isize,
}

impl SolveSolution for Ex4 {
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let mut sum = 0;

        // let mut positions = deserialize_to_hashmap("./src/ex4/temp.txt")?;
        let mut positions = deserialize_to_hashmap("./src/ex4/dataset2.txt")?;

        let max_len = positions.len();

        assert!(
            (max_len as f32).sqrt().fract() == 0.0,
            "Data not square-like"
        );

        for (pos, _) in positions.iter().filter(|(_, v)| v.has_roll) {
            let neighboors = get_neighboors(pos, max_len as isize);

            let total_rolls: usize = neighboors
                .iter()
                .map(|gp| {
                    if let Some(gp) = positions.get(gp)
                        && gp.has_roll
                    {
                        1
                    } else {
                        0
                    }
                })
                .sum();

            if total_rolls < 4 {
                if let Some(val) = positions.get(pos) {
                    val.matches_ex_1.replace(true);
                }

                sum += 1;
            }
        }

        fmt_positions(&positions, max_len);

        Ok(sum.to_string())
    }
    fn solve_2() -> Result<String, Box<dyn Error>> {
        let mut sum = 0;

        let positions = deserialize_to_hashmap("./src/ex4/dataset2.txt")?;

        let max_len = positions.len();

        assert!(
            (max_len as f32).sqrt().fract() == 0.0,
            "Data not square-like"
        );

        loop {
            let mut turn_total = 0;

            for (pos, roll) in positions.iter().filter(|(_, v)| v.has_roll) {
                if *roll.removed.borrow() {
                    continue;
                }

                let neighboors = get_neighboors(pos, max_len as isize);

                let total_rolls: usize = neighboors
                    .iter()
                    .map(|gp| {
                        if let Some(gp) = positions.get(gp)
                            && gp.has_roll
                            && !*gp.removed.borrow()
                        {
                            1
                        } else {
                            0
                        }
                    })
                    .sum();

                if total_rolls < 4 {
                    if let Some(val) = positions.get(pos) {
                        val.matches_ex_1.replace(true);
                        val.removed.replace(true);
                    }

                    turn_total += 1;
                }
            }

            sum += turn_total;

            if turn_total == 0 {
                break;
            }
        }
        // fmt_positions(&positions, max_len);

        Ok(sum.to_string())
    }
}

fn get_neighboors(pos: &GridPosition, max_pos: isize) -> Vec<GridPosition> {
    let GridPosition { x, y } = pos;
    let max_pos = max_pos - 1;

    let mut vec = Vec::with_capacity(9);

    for i in x - 1..=x + 1 {
        for j in y - 1..=y + 1 {
            if i == *x && j == *y {
                continue;
            }

            if !(0..max_pos).contains(&i) {
                continue;
            }
            if !(0..max_pos).contains(&j) {
                continue;
            }

            vec.push(GridPosition { x: i, y: j });
        }
    }

    vec
}

fn fmt_positions(positions: &HashMap<GridPosition, Roll>, max_len: usize) {
    let mut sorted_positions: Vec<(&GridPosition, &Roll)> = positions.iter().collect();
    sorted_positions.sort_by(|(pos_a, _), (pos_b, _)| {
        pos_a.y.cmp(&pos_b.y).then_with(|| pos_a.x.cmp(&pos_b.x))
    });

    let side_length = (max_len as f32).sqrt() as usize;

    println!(
        "  {}",
        (0..side_length)
            .map(|x| format!("{x} "))
            .collect::<String>()
    );

    for (y, chunk) in sorted_positions.chunks(side_length).enumerate() {
        let line: String = chunk.iter().map(|(_, roll)| format!("{roll} ")).collect();
        println!("{} {} {}", y, line, y);
    }

    println!(
        "  {}",
        (0..side_length)
            .map(|x| format!("{x} "))
            .collect::<String>()
    );
}

fn deserialize_to_hashmap(
    file_name: &str,
) -> Result<std::collections::HashMap<GridPosition, Roll>, Box<dyn Error>> {
    let data: String = fs::read_to_string(file_name)?;

    let re = regex::Regex::new(r"(\.|@)")?;
    let mut instructions = std::collections::HashMap::new();

    for (y, line) in data.lines().enumerate() {
        for (x, symbol_match) in re.captures_iter(line).enumerate() {
            let symbol = symbol_match.get(0).unwrap().as_str();
            let grid_position = GridPosition {
                x: x as isize,
                y: y as isize,
            };
            let roll = Roll {
                pos: grid_position, // Clone the GridPosition for the Roll struct
                has_roll: match symbol {
                    "." => false,
                    "@" => true,
                    _ => panic!("Shouldn't happen"),
                },
                matches_ex_1: RefCell::new(false),
                removed: RefCell::new(false),
            };
            instructions.insert(grid_position, roll);
        }
    }

    Ok(instructions)
}
