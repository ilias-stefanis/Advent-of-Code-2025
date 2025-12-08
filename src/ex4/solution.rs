use rapidhash::{HashMapExt, RapidHashMap};

use crate::SolveSolution;
use std::cell::RefCell;
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
    #[hotpath::measure]
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let mut sum = 0;

        // let mut positions = deserialize_to_hashmap("./src/ex4/temp.txt")?;
        let positions = deserialize_to_hashmap("./src/ex4/dataset2.txt")?;

        let max_len = positions.len();

        assert!(
            (max_len as f32).sqrt().fract() == 0.0,
            "Data not square-like"
        );

        let mut neighboors_buffer = Vec::with_capacity(9);

        for (pos, _) in positions.iter().filter(|(_, v)| v.has_roll) {
            neighboors_buffer.clear();
            get_neighboors(pos, max_len as isize, &mut neighboors_buffer);

            let total_rolls: usize = neighboors_buffer
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

    #[hotpath::measure]
    fn solve_2() -> Result<String, Box<dyn Error>> {
        let mut sum = 0;

        let positions = deserialize_to_hashmap("./src/ex4/dataset2.txt")?;

        let max_len = positions.len();

        // assert!(
        //     (max_len as f32).sqrt().fract() == 0.0,
        //     "Data not square-like"
        // );

        let mut neighboors_buffer = Vec::with_capacity(9);

        loop {
            let mut turn_total = 0;

            for (pos, roll) in positions.iter().filter(|(_, v)| !*v.removed.borrow()) {
                neighboors_buffer.clear();

                get_neighboors(pos, max_len as isize, &mut neighboors_buffer);

                let total_rolls: usize = neighboors_buffer
                    .iter()
                    .map(|gp| {
                        if let Some(gp) = positions.get(gp)
                            // && gp.has_roll
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

#[hotpath::measure]
fn get_neighboors(pos: &GridPosition, max_pos: isize, buffer: &mut Vec<GridPosition>) {
    let GridPosition { x, y } = pos;
    let max_coord = max_pos - 1;

    let start_x = (*x - 1).max(0);
    let end_x = (*x + 1).min(max_coord);
    let start_y = (*y - 1).max(0);
    let end_y = (*y + 1).min(max_coord);

    for i in start_x..=end_x {
        for j in start_y..=end_y {
            if i == *x && j == *y {
                continue;
            }
            buffer.push(GridPosition { x: i, y: j });
        }
    }
}

fn fmt_positions(positions: &RapidHashMap<GridPosition, Roll>, max_len: usize) {
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

#[hotpath::measure]
fn deserialize_to_hashmap(
    file_name: &str,
) -> Result<RapidHashMap<GridPosition, Roll>, Box<dyn Error>> {
    let data: String = fs::read_to_string(file_name)?;

    let re = regex::Regex::new(r"(\.|@)")?;
    let mut instructions = RapidHashMap::with_capacity(250);

    for (y, line) in data.lines().enumerate() {
        for (x, symbol_match) in re.captures_iter(line).enumerate() {
            let symbol = symbol_match.get(0).unwrap().as_str();

            if symbol == "." {
                continue;
            }

            let grid_position = GridPosition {
                x: x as isize,
                y: y as isize,
            };
            let roll = Roll {
                pos: grid_position, // Clone the GridPosition for the Roll struct
                has_roll: match symbol {
                    // "." => false,
                    "@" => true,
                    _ => unreachable!("shouldnt happen"),
                },
                matches_ex_1: RefCell::new(false),
                removed: RefCell::new(false),
            };
            instructions.insert(grid_position, roll);
        }
    }

    Ok(instructions)
}
