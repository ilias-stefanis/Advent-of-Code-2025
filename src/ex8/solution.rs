use rapidhash::{HashMapExt, HashSetExt, RapidHashMap, RapidHashSet};

use crate::SolveSolution;
use core::num;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::{fs, vec};

pub struct Ex8;

#[derive(Clone, Debug, PartialEq, Eq)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
    canonical_form: u128,
}

#[derive(Clone, Debug)]
struct Edge {
    a: usize,
    b: usize,
    distance: f64,
}

type EdgeKey = (usize, usize);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

struct Circuit {
    junction_boxes: Vec<usize>,
    edges: Vec<EdgeKey>,
}

impl SolveSolution for Ex8 {
    #[hotpath::measure]
    fn solve_1() -> Result<String, Box<dyn Error>> {
        let junction_boxes = deserialize("./src/ex8/dataset1.txt")?;
        dbg!(&junction_boxes);
        let sum = 0;

        const COUNT: usize = 10;

        let mut edges: RapidHashMap<EdgeKey, Edge> = RapidHashMap::with_capacity(COUNT);

        let mut connected_subgraphs = find_connected_components(
            &junction_boxes,
            &edges.values().cloned().collect::<Vec<_>>(),
        );
        let mut inserted_length = 0;
        for current_idx in 0..junction_boxes.len() {
            if inserted_length >= COUNT {
                break;
            }

            let jb = &junction_boxes[current_idx];

            let Some(closest_index) = search_closest_junction_box(&junction_boxes, current_idx)
            else {
                continue;
            };

            let closest_jb = &junction_boxes[closest_index];
            let distance = calc_distance_3d(jb, closest_jb);

            let key = if current_idx < closest_index {
                (current_idx, closest_index)
            } else {
                (closest_index, current_idx)
            };

            connected_subgraphs = find_connected_components(
                &junction_boxes,
                &edges.values().cloned().collect::<Vec<_>>(),
            );

            edges.entry(key).or_insert_with(|| {
                inserted_length += 1;
                Edge {
                    a: current_idx,
                    b: closest_index,
                    distance,
                }
            });
        }

        debug_fmt_merges(&junction_boxes, &edges);

        // let mut connected_junctions: RapidHashSet<usize> = RapidHashSet::with_capacity(COUNT);
        // let mut edge_count: Vec<u8> = vec![0; junction_boxes.len()];

        // edges.iter().map(|c| c.1).for_each(|edge| {
        //     connected_junctions.insert(edge.a);
        //     connected_junctions.insert(edge.b);
        //     edge_count[edge.a] += 1;
        //     edge_count[edge.b] += 1;
        // });

        // let connected_subgraphs = find_connected_components(&junction_boxes, &edges.values().cloned().collect::<Vec<_>>());

        dbg!(&connected_subgraphs);

        Ok(sum.to_string())
    }

    #[hotpath::measure]
    fn solve_2() -> Result<String, Box<dyn Error>> {
        let junction_boxes = deserialize("./src/ex8/dataset1.txt")?;

        let sum: u128 = 0;

        Ok(sum.to_string())
    }
}

#[hotpath::measure]
fn deserialize(file_name: &str) -> Result<Vec<JunctionBox>, Box<dyn Error>> {
    let data = fs::read_to_string(file_name)?;

    let mut points: Vec<JunctionBox> = vec![];

    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let values: Vec<usize> = trimmed
            .split(',')
            .map(|v| v.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut point = JunctionBox {
            x: values[0],
            y: values[1],
            z: values[2],
            canonical_form: 0,
        };

        point.canonical_form = calc_morton_code(&point);

        points.push(point);
    }

    // points.sort_unstable_by_key(|p| p.canonical_form);

    Ok(points)
}

struct Subgraph {
    nodes: Vec<JunctionBox>,
    edges: Vec<Edge>,
}

fn find_connected_components(nodes: &[JunctionBox], edges: &[Edge]) -> Vec<Subgraph> {
    let num_nodes = nodes.len();

    // 1. Build Adjacency List
    // We store (neighbor_index, edge_index_in_original_list)
    let mut adj: Vec<Vec<(usize, usize)>> = vec![vec![]; num_nodes];
    for (i, edge) in edges.iter().enumerate() {
        // Ensure indices are within bounds to prevent panics
        if edge.a < num_nodes && edge.b < num_nodes {
            adj[edge.a].push((edge.b, i));
            adj[edge.b].push((edge.a, i));
        }
    }

    let mut visited = vec![false; num_nodes];
    let mut subgraphs = Vec::new();

    // 2. Iterate through all nodes to find unvisited components
    for start_node_idx in 0..num_nodes {
        if visited[start_node_idx] {
            continue;
        }

        // --- Start of BFS for a new component ---
        let mut component_node_indices = Vec::new();
        let mut component_edge_indices = RapidHashSet::new(); // Use Set to avoid duplicate edges
        let mut queue = VecDeque::new();

        visited[start_node_idx] = true;
        queue.push_back(start_node_idx);

        while let Some(u) = queue.pop_front() {
            component_node_indices.push(u);

            for &(v, edge_idx) in &adj[u] {
                // Track the edge (undirected, so we might see it twice, Set handles this)
                component_edge_indices.insert(edge_idx);

                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }

        // 3. Construct the Subgraph with Remapped Indices
        // Map: Original Global Index -> New Local Subgraph Index
        let mut old_to_new_map = RapidHashMap::new();
        let mut new_nodes = Vec::with_capacity(component_node_indices.len());

        for (new_idx, &old_idx) in component_node_indices.iter().enumerate() {
            old_to_new_map.insert(old_idx, new_idx);
            new_nodes.push(nodes[old_idx].clone());
        }

        let mut new_edges = Vec::with_capacity(component_edge_indices.len());
        for &edge_idx in &component_edge_indices {
            let original_edge = &edges[edge_idx];
            let mut new_edge = original_edge.clone();

            // CRITICAL: Remap the indices to match the new `new_nodes` vector
            // We use unwrap because we know the nodes must exist in this component
            new_edge.a = *old_to_new_map.get(&original_edge.a).unwrap();
            new_edge.b = *old_to_new_map.get(&original_edge.b).unwrap();

            new_edges.push(new_edge);
        }

        subgraphs.push(Subgraph {
            nodes: new_nodes,
            edges: new_edges,
        });
    }

    subgraphs
}

fn calculate_canonical_form_simple(junction_box: &JunctionBox) -> u128 {
    let mut values = vec![junction_box.x, junction_box.y, junction_box.z];
    values.sort_unstable();

    (values[0] as u128) << 42 | (values[1] as u128) << 21 | (values[2] as u128)
}

fn calc_morton_code(junction_box: &JunctionBox) -> u128 {
    // We split the bits of each coordinate so they are spaced out by 2 zeros.
    // x: 000001 -> 001001
    fn split_by_3(a: usize) -> u64 {
        let mut x = a as u64;
        // Mask out the lower 21 bits (max value ~2 million)
        x &= 0x1fffff;

        // The "Magic Numbers" shift bits into positions 0, 3, 6, 9...
        x = (x | x << 32) & 0x1f00000000ffff;
        x = (x | x << 16) & 0x1f0000ff0000ff;
        x = (x | x << 8) & 0x100f00f00f00f00f;
        x = (x | x << 4) & 0x10c30c30c30c30c3;
        x = (x | x << 2) & 0x1249249249249249;
        x
    }

    let m_x = split_by_3(junction_box.x);
    let m_y = split_by_3(junction_box.y);
    let m_z = split_by_3(junction_box.z);

    // Combine them: Z shifted left 2, Y shifted left 1, X no shift
    let result = (m_z << 2) | (m_y << 1) | m_x;
    result as u128
}

fn calc_distance_3d(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let dx = (a.x as isize - b.x as isize) as f64;
    let dy = (a.y as isize - b.y as isize) as f64;
    let dz = (a.z as isize - b.z as isize) as f64;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn search_closest_junction_box<'a>(
    junction_boxes: &'a Vec<JunctionBox>,
    target_index: usize,
) -> Option<usize> {
    let target = &junction_boxes[target_index];
    let mut closest: Option<usize> = None;
    let mut closest_distance = f64::MAX;

    // Check neighbors within a certain range based on the sorted order
    let range = 1..=3; // Adjust this range as needed

    for offset in range {
        if target_index >= offset {
            let indice = target_index - offset;

            let jb = &junction_boxes[indice];
            let distance = calc_distance_3d(jb, target);
            if distance < closest_distance {
                closest_distance = distance;
                closest = Some(indice);
            }
        }

        if target_index + offset < junction_boxes.len() {
            let indice = target_index + offset;

            let jb = &junction_boxes[indice];
            let distance = calc_distance_3d(jb, target);
            if distance < closest_distance {
                closest_distance = distance;
                closest = Some(indice);
            }
        }
    }

    if closest.is_some() {
        return closest;
    };

    // Fallback: linear search (inefficient)
    for (i, jb) in junction_boxes.iter().enumerate() {
        if i == target_index {
            continue;
        }
        let distance = calc_distance_3d(jb, target);
        if distance < closest_distance {
            closest_distance = distance;
            closest = Some(i);
        }
    }

    closest
}

fn debug_fmt_merges(points: &[JunctionBox], edges: &RapidHashMap<EdgeKey, Edge>) {
    for ((a, b), edge) in edges.iter() {
        let a = &points[*a];
        let b = &points[*b];
        println!(
            "({}, {}, {}) - ({}, {}, {}) has distance: {}",
            a.x, a.y, a.z, b.x, b.y, b.z, edge.distance
        );
    }
}

impl Debug for Subgraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Subgraph with {} nodes and {} edges {{",
            self.nodes.len(),
            self.edges.len()
        )?;
        for (i, node) in self.nodes.iter().enumerate() {
            writeln!(f, "  {}: ({}, {}, {})", i, node.x, node.y, node.z)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

fn get_connected_edges(haystack_edge: &Edge, edges: &RapidHashMap<EdgeKey, Edge>) -> Vec<EdgeKey> {
    let mut connected_edges = vec![];

    for edge in edges.values() {
        if edge == haystack_edge {
            continue;
        }

        if edge.a == haystack_edge.a
            || edge.a == haystack_edge.b
            || edge.b == haystack_edge.a
            || edge.b == haystack_edge.b
        {
            connected_edges.push((edge.a, edge.b));
        }
    }

    connected_edges
}

