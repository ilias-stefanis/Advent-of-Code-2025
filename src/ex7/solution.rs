use rapidhash::{HashMapExt, RapidHashMap};

use crate::SolveSolution;
use std::error::Error;
use std::{fs, vec};

pub struct Ex7;

#[derive(Clone, Debug, PartialEq, Eq)]
enum DiagramPointType {
    Start,
    Splitter,
    Line,
}

#[derive(Clone, Debug)]
struct DiagramPoint {
    x: usize,
    y: usize,
    ptype: DiagramPointType,
}

impl SolveSolution for Ex7 {
    #[hotpath::measure]
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let (mut point_grid, max_y) = deserialize("./src/ex7/dataset2.txt")?;

        let (_start_x, start_y) = *point_grid
            .iter()
            .find(|v| v.1.ptype == DiagramPointType::Start)
            .unwrap()
            .0;

        let mut current_y = start_y;

        while current_y < max_y {
            // fmt_grid(&point_grid, max_y);
            part1_process_next_step(&mut point_grid, current_y);
            // println!("------------------");
            current_y += 1;
        }
        // fmt_grid(&point_grid, max_y);

        let sum: usize = point_grid
            .iter()
            .map(|v| v.1)
            .filter(|el| matches!(el.ptype, DiagramPointType::Splitter))
            .filter(|el| {
                let up = point_grid
                    .get(&(el.x, el.y - 1))
                    .filter(|el| {
                        matches!(
                            el.ptype,
                            DiagramPointType::Splitter | DiagramPointType::Line
                        )
                    })
                    .is_some();

                up
            })
            .count();

        Ok(sum.to_string())
    }

    #[hotpath::measure]
    fn solve_2() -> Result<String, Box<dyn Error>> {
        let (point_grid, _max_y) = deserialize("./src/ex7/dataset2.txt")?;

        let x_count = point_grid.iter().max_by_key(|a| a.1.x).unwrap().1.x + 2;

        let mut points_buffer: Vec<_> = point_grid.iter().collect();
        points_buffer.sort_by(|a, b| a.1.y.cmp(&b.1.y).reverse().then(a.1.x.cmp(&b.1.x)));

        let mut timelines = vec![1_usize; x_count];

        for row in points_buffer.chunk_by(|a, b| a.1.y == b.1.y) {
            for (_, point) in row {
                if let DiagramPoint {
                    x,
                    ptype: DiagramPointType::Splitter,
                    ..
                } = point
                {
                    let x = *x;
                    timelines[x] = timelines[x - 1] + timelines[x + 1];
                }
            }
        }

        let start_x = points_buffer
            .iter()
            .rev()
            .find(|v| v.1.ptype == DiagramPointType::Start)
            .unwrap()
            .1
            .x;

        Ok(timelines[start_x].to_string())
    }
}

#[hotpath::measure]
fn deserialize(
    file_name: &str,
) -> Result<(RapidHashMap<(usize, usize), DiagramPoint>, usize), Box<dyn Error>> {
    let data = fs::read_to_string(file_name)?;

    let estimated_cap = data.len() / 10;
    let mut points = RapidHashMap::with_capacity(estimated_cap);

    let mut max_y = 0;

    for (y, line) in data.lines().enumerate() {
        max_y = y;

        for (x, char_code) in line.as_bytes().iter().enumerate() {
            let ptype = match char_code {
                b'S' => DiagramPointType::Start,
                b'^' => DiagramPointType::Splitter,
                b'.' => continue,
                c => return Err(format!("Unexpected character '{}' at ({}, {})", c, x, y).into()),
            };

            points.insert((x, y), DiagramPoint { x, y, ptype });
        }
    }

    let count = if data.is_empty() { 0 } else { max_y + 1 };

    Ok((points, count))
}

fn part1_process_next_step(
    grid: &mut RapidHashMap<(usize, usize), DiagramPoint>,
    current_y: usize,
) {
    let points_to_calc: Vec<_> = grid
        .iter()
        .map(|v| v.1)
        .filter(|el| {
            let is_correct_height = el.y == current_y;
            let matches = matches!(el.ptype, DiagramPointType::Start | DiagramPointType::Line);
            is_correct_height && matches
        })
        .cloned()
        .collect();

    for el in &points_to_calc {
        let new_coords = (el.x, el.y + 1);
        let new_manifold = grid.get(&new_coords);
        match new_manifold {
            Some(DiagramPoint {
                ptype: DiagramPointType::Splitter,
                ..
            }) => {
                let left = (new_coords.0 - 1, new_coords.1);
                let right = (new_coords.0 + 1, new_coords.1);

                grid.entry(left).or_insert(DiagramPoint {
                    x: left.0,
                    y: left.1,
                    ptype: DiagramPointType::Line,
                });
                grid.entry(right).or_insert(DiagramPoint {
                    x: right.0,
                    y: right.1,
                    ptype: DiagramPointType::Line,
                });
            }
            Some(_) => continue,
            None => {
                grid.insert(
                    new_coords,
                    DiagramPoint {
                        x: new_coords.0,
                        y: new_coords.1,
                        ptype: DiagramPointType::Line,
                    },
                );
            }
        };
    }
}

fn fmt_grid(grid: &RapidHashMap<(usize, usize), DiagramPoint>, max_y: usize) {
    let mut points_buffer: Vec<_> = grid.iter().collect();
    points_buffer.sort_by(|a, b| a.1.y.cmp(&b.1.y).then(a.1.x.cmp(&b.1.x)));

    let x_count = points_buffer.iter().max_by_key(|a| a.1.x).unwrap().1.x + 1;

    let mut formated_line = vec![vec!['.'; x_count]; max_y + 1];
    for point in points_buffer {
        let char_to_add = match point.1.ptype {
            DiagramPointType::Line => '|',
            DiagramPointType::Start => 'S',
            DiagramPointType::Splitter => '^',
        };
        formated_line[point.1.y][point.1.x] = char_to_add;
    }

    formated_line.iter().enumerate().for_each(|f| {
        let fmt: String = f.1.iter().cloned().collect();
        println!("{:3} - {}", f.0, fmt);
    });
}
