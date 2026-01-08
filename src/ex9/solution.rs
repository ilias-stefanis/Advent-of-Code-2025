use crate::SolveSolution;
use std::error::Error;
use std::fs;

pub struct Ex9;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Rectangle {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Rectangle {
    fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle {
            x_min: p1.x.min(p2.x),
            x_max: p1.x.max(p2.x),
            y_min: p1.y.min(p2.y),
            y_max: p1.y.max(p2.y),
        }
    }

    fn area(&self) -> i64 {
        let width = (self.x_max - self.x_min).abs() as i64 + 1;
        let height = (self.y_max - self.y_min).abs() as i64 + 1;
        width * height
    }
}

impl SolveSolution for Ex9 {
    #[hotpath::measure]
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let points = deserialize("./src/ex9/dataset2.txt")?;
        let mut max_area = 0;

        // yes it's O(n^2), let it bruteforce in peace
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let rect = Rectangle::new(&points[i], &points[j]);
                let area = rect.area();
                if area > max_area {
                    max_area = area;
                }
            }
        }

        Ok(max_area.to_string())
    }

    #[hotpath::measure]
    fn solve_2() -> Result<String, Box<dyn Error>> {
        let points = deserialize("./src/ex9/dataset2.txt")?;

        let mut max_area = 0;

        let mut edges = Vec::with_capacity(points.len());
        for i in 0..points.len() {
            let p1 = &points[i];
            let p2 = &points[(i + 1) % points.len()];
            edges.push((p1, p2));
        }

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let rect = Rectangle::new(&points[i], &points[j]);
                let area = rect.area();

                if area <= max_area {
                    continue;
                }

                if is_valid_rectangle(&rect, &points, &edges) {
                    max_area = area;
                }
            }
        }

        Ok(max_area.to_string())
    }
}

fn is_valid_rectangle(rect: &Rectangle, poly_verts: &[Point], edges: &[(&Point, &Point)]) -> bool {
    let center_2x = rect.x_min + rect.x_max;
    let center_2y = rect.y_min + rect.y_max;

    if !is_point_in_polygon_doubled(center_2x, center_2y, poly_verts) {
        return false;
    }

    for (p1, p2) in edges {
        if p1.x == p2.x {
            let edge_x = p1.x;
            let y_start = p1.y.min(p2.y);
            let y_end = p1.y.max(p2.y);

            if edge_x > rect.x_min && edge_x < rect.x_max {
                let overlap_start = y_start.max(rect.y_min);
                let overlap_end = y_end.min(rect.y_max);

                if overlap_start < overlap_end {
                    return false;
                }
            }
        } else {
            let edge_y = p1.y;
            let x_start = p1.x.min(p2.x);
            let x_end = p1.x.max(p2.x);

            if edge_y > rect.y_min && edge_y < rect.y_max {
                let overlap_start = x_start.max(rect.x_min);
                let overlap_end = x_end.min(rect.x_max);

                if overlap_start < overlap_end {
                    return false;
                }
            }
        }
    }

    true
}

fn is_point_in_polygon_doubled(test_2x: isize, test_2y: isize, verts: &[Point]) -> bool {
    let mut inside = false;
    let mut j = verts.len() - 1;
    for i in 0..verts.len() {
        let pi = &verts[i];
        let pj = &verts[j];

        let pix = pi.x * 2;
        let piy = pi.y * 2;
        let pjx = pj.x * 2;
        let pjy = pj.y * 2;

        let in_y_range = (piy > test_2y) != (pjy > test_2y);

        if in_y_range {
            let intersect_x = pix as f64
                + (test_2y as f64 - piy as f64) / (pjy as f64 - piy as f64)
                    * (pjx as f64 - pix as f64);
            if (test_2x as f64) < intersect_x {
                inside = !inside;
            }
        }

        j = i;
    }

    inside
}

fn deserialize(file_name: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let data = fs::read_to_string(file_name)?;
    let mut points: Vec<Point> = vec![];

    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let values: Vec<isize> = trimmed
            .split(',')
            .map(|v| v.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;

        points.push(Point {
            x: values[0],
            y: values[1],
        });
    }

    Ok(points)
}
