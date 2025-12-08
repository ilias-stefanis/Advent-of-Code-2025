use crate::SolveSolution;
use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;

pub struct Ex5;

impl SolveSolution for Ex5 {
    #[hotpath::measure]
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let (ranges, ingredients) = deserialize("./src/ex5/dataset2.txt")?;
        let sum: usize = ingredients
            .iter()
            .map(|i| {
                if ranges.iter().any(|r| r.contains(i)) {
                    1
                } else {
                    0
                }
            })
            .sum();

        Ok(sum.to_string())
    }

    #[hotpath::measure]
    fn solve_2() -> Result<String, Box<dyn Error>> {
        let (ranges, ingredients) = deserialize("./src/ex5/dataset2.txt")?;

        let count: usize = ranges.iter().map(|r| r.size_hint().1.unwrap()).sum();

        Ok(count.to_string())
    }
}

#[hotpath::measure]
fn deserialize(
    file_name: &str,
) -> Result<(Vec<RangeInclusive<usize>>, Vec<usize>), Box<dyn Error>> {
    let data: String = fs::read_to_string(file_name)?;

    // let re = regex::Regex::new(r"\d+")?;
    let mut ranges: Vec<RangeInclusive<usize>> = Vec::with_capacity(250);

    let mut ingredients = Vec::with_capacity(250);

    enum ParseState {
        Ranges,
        Ingredients,
    }
    let mut state = ParseState::Ranges;

    for line in data.lines() {
        if line.is_empty() {
            state = ParseState::Ingredients;
            continue;
        }

        if matches!(state, ParseState::Ranges) {
            let mut range = line.split("-");
            let start: usize = range.next().unwrap().parse().unwrap();
            let end: usize = range.next().unwrap().parse().unwrap();

            let mut add_range = true;
            let range_to_add = start..=end;

            for range in &mut ranges {
                if range_to_add.contains(range.start()) || range_to_add.contains(range.end()) {
                    *range = RangeInclusive::new(start.min(*range.start()), end.max(*range.end()));
                    add_range = false;
                }
            }

            if add_range {
                ranges.push(range_to_add);
            }
        } else {
            let number: usize = line.parse().unwrap();
            ingredients.push(number);
        }

        flattern_ranges(&mut ranges);
    }

    Ok((ranges, ingredients))
}

fn flattern_ranges(ranges: &mut Vec<RangeInclusive<usize>>) {
    let mut buffer: Vec<_> = Vec::with_capacity(ranges.len());

    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    buffer.push(ranges[0].clone());

    for current_range in &ranges[1..] {
        let last_merged_range = buffer.last_mut().unwrap();

        if *current_range.start() <= *last_merged_range.end() + 1 {
            *last_merged_range = RangeInclusive::new(
                *last_merged_range.start(),
                *current_range.end().max(last_merged_range.end()),
            );
        } else {
            buffer.push(current_range.clone());
        }
    }

    *ranges = buffer;
}
