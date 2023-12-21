use std::collections::VecDeque;
use std::fmt::Write;
use ndarray::prelude::*;
use itertools::Itertools;
use num::Integer;

advent_of_code::solution!(21);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Node {
    Empty,
    Blocked,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Node::Empty => '.',
            Node::Blocked => '#',
        })
    }
}

type Grid = Array2<Node>;
type Input = ([i32; 2], Grid);
fn parse(input: &str) -> Input {
    let mut width = 0;
    let mut nodes = Vec::new();
    let mut start = None;

    for (i, line) in input.lines().enumerate() {
        if width == 0 {
            width = line.len();
        }
        else if width != line.len() {
            panic!("unexpected line width");
        }

        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => nodes.push(Node::Empty),
                '#' => nodes.push(Node::Blocked),
                'S' => {
                    assert!(start.is_none());
                    nodes.push(Node::Empty);
                    start = Some([i as i32, j as i32]);
                }
                _ => panic!("unexpected char"),
            }
        }
    }

    (start.unwrap(), Grid::from_shape_vec((nodes.len() / width, width), nodes).unwrap() )

}

fn bfs_distance(start: &[i32; 2], grid: &Grid) -> Array2<u32> {
    let mut distance = ndarray::Array::from_elem(grid.raw_dim(), u32::MAX);

    let mut queue = VecDeque::new();
    queue.push_front((*start, 0));

    while let Some(([i, j], d)) = queue.pop_front() {
        let out_of_bounds = i < 0 || j < 0 || i as usize >= grid.nrows() || j as usize >= grid.ncols();
        let idx = [i as usize, j as usize];
        if out_of_bounds || distance[idx] <= d || grid[idx] == Node::Blocked {
            continue;
        }

        distance[idx] = d;
        queue.push_back(([i+1, j], d+1));
        queue.push_back(([i-1, j], d+1));
        queue.push_back(([i, j+1], d+1));
        queue.push_back(([i, j-1], d+1));
    }

    distance
}

fn count(start: &[i32; 2], grid: &Grid, steps: usize) -> u32 {
    let distance = bfs_distance(start, grid);
    let start = start.map(|v| v as usize);
    let parity = steps % 2;

    distance.indexed_iter().filter(|(p, d)| {
        **d <= steps as u32 && (p.0.abs_diff(start[0]) + p.1.abs_diff(start[1])) % 2 == parity
    }).count() as u32
}

fn part_one_impl(input: &str, steps: usize) -> Option<u32> {
    let (start, grid) = parse(input);
    Some(count(&start, &grid, steps))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

fn bfs_distance_modular(start: &[i32; 2], grid: &Grid, step: usize) -> Array2<u32> {
    let n = step * 2 + 1;
    let mut distance = ndarray::Array::from_elem((n, n), u32::MAX);
    
    // start is at the center of distance, distance grid has an offset wrt to grid (0,0)
    let offset = [start[0] - step as i32, start[1] - step as i32];

    let mut queue = VecDeque::new();
    queue.push_front(([step as i32, step as i32], 0));

    while let Some(([i, j], d)) = queue.pop_front() {
        let out_of_bounds = i < 0 || j < 0 || i as usize >= n || j as usize >= n;
        let idx = [i as usize, j as usize];
        let grid_idx = [(i + offset[0]).mod_floor(&(grid.nrows() as i32)) as usize, (j + offset[1]).mod_floor(&(grid.ncols() as i32)) as usize];
        if out_of_bounds || distance[idx] <= d || grid[grid_idx] == Node::Blocked {
            continue;
        }

        distance[idx] = d;
        queue.push_back(([i+1, j], d+1));
        queue.push_back(([i-1, j], d+1));
        queue.push_back(([i, j+1], d+1));
        queue.push_back(([i, j-1], d+1));
    }

    distance
}

fn count_modular(start: &[i32; 2], grid: &Grid, steps: usize) -> usize {
    let distance = bfs_distance_modular(start, grid, steps); // start is centered at [step, step]
    let parity = steps % 2;

    distance.indexed_iter().filter(|(p, d)| {
        **d <= steps as u32 && (p.0.abs_diff(steps) + p.1.abs_diff(steps)) % 2 == parity
    }).count()
}

#[allow(dead_code)]
fn extrapolate(start: &[i32; 2], grid: &Grid, steps: usize) -> usize {
    let w = grid.ncols();
    let h = grid.nrows();
    assert_eq!(w, h);

    let n = w;
    if steps > 3 * n {
        // extrapolate quadratic function
        let s = steps % n;
        let m = steps / n;

        let vs = [
            count_modular(start, grid, s),
            count_modular(start, grid, s + n),
            count_modular(start, grid, s + 2*n)
        ];

        // we do newtown polynomials, where the value we want to get is m
        let a = [
            vs[0] as isize,
            (vs[1] - vs[0]) as isize,
            (((vs[2] - vs[1]) - (vs[1] - vs[0])) / 2) as isize,
        ];

        let poly = |x| a[0] + a[1] * x + a[2] * x * (x - 1);

        assert_eq!(poly(0) as usize, vs[0]);
        assert_eq!(poly(1) as usize, vs[1]);
        assert_eq!(poly(2) as usize, vs[2]);
        
        poly(m as isize) as usize
    }
    else {
        count_modular(start, grid, steps)
    }
}

#[allow(dead_code)]
fn count_big_border(start: &[i32; 2], grid: &Grid, steps: usize) -> usize {   
    // we know that the input has some trenches of empty space
    // in the outer border, and so the minimum period between maps
    // is just the width/height of the initial map
    let w = grid.ncols();
    let h = grid.nrows();
    assert_eq!(w, h);
    let d = w;

    assert!(steps > 2*w); // sanity check

    let empty_cols = grid.columns().into_iter().enumerate().filter_map(|(j, m)| m.iter().all(|n| *n == Node::Empty).then_some(j)).collect_vec();
    let empty_rows = grid.rows().into_iter().enumerate().filter_map(|(i, m)| m.iter().all(|n| *n == Node::Empty).then_some(i)).collect_vec();
    assert_eq!(&empty_cols, &[0, d - 1]);
    assert_eq!(&empty_rows, &[0, d - 1]);

    // Given a cell (i,j), we reach their clones whenever we move from a map to another.
    // We index each map with (I,J).
    // Given that maps are odd, the parity of the cell changes accordingly.
    // When moving N maps at, say, direction (1,0),
    // then that cell is within
    //      d(S, c) = argmin (d(S, B0) + N*d(B0, B1) + d(B1, c))
    // where B0 correspond to a cell in the bottom trench and B1 a cell in the bottom trench of next bottom map

    // we also know that h = w, and so it is the same in terms of period to move one direction or the other

    // at a really big distance (N > w), the term d(B0, B1) dominates and so only those columns totally
    // free are to be considered, and by construction this map has at least one column/row that are empty (borders)

    // this makes the choice of B0 and B1 to be easy, as there are only two possibilities, the two bottom corners
    
    // a similar argument can be done for any direction, and when combining both vertical and horizontal movement
    // then the choice is the concrete corner

    let ul = bfs_distance(&[0, 0], grid);
    let bl = bfs_distance(&[(h - 1) as i32, 0], grid);
    let ur = bfs_distance(&[0, (w - 1) as i32], grid);
    let br = bfs_distance(&[(h - 1) as i32, (w - 1) as i32], grid);

    let start = (start[0] as usize, start[1] as usize);

    let parity = steps % 2;
    ul.indexed_iter().filter_map(|(p, n)| { (*n != u32::MAX).then_some(p) })
        .map(|p| {
            let mut result = 0;

            // map (0,0)
            let correct_parity = parity == (p.0.abs_diff(start.0) + p.1.abs_diff(start.1)) % 2;
            if correct_parity { result += 1; }

            // map moving to one of the main directions:
            for offset in [
                (bl[start] + ul[p]).min(br[start] + ur[p]) + 1, // moving down
                (ul[start] + bl[p]).min(ur[start] + br[p]) + 1, // moving up
                (ur[start] + ul[p]).min(br[start] + bl[p]) + 1, // moving right
                (ul[start] + ur[p]).min(bl[start] + br[p]) + 1, // moving left
            ]
            {
                if steps < offset as usize { continue; }
                let n = 1 + (steps - offset as usize) / d;
                result += n / 2; // only half of the maps will have correct parity
                if n % 2 == 1 && !correct_parity { result += 1; }    
            }

            // map moving to one of the main diagonals:
            for offset in [
                br[start] + ul[p] + 2,
                ur[start] + bl[p] + 2,
                bl[start] + ur[p] + 2,
                ul[start] + br[p] + 2,
            ]
            {
                if steps < offset as usize { continue; }
                let m = 1 + (steps - offset as usize) / d;
                if correct_parity {
                    // we sum up all odd numbers
                    let odd = (m+1) / 2;
                    result += odd * odd;
                }
                else {
                    // we sum up all even numbers
                    let even = m / 2;
                    result += even * (even + 1);

                }    
            }

            result
        }).sum()
}

#[allow(dead_code)]
fn count_big_cross(start: &[i32; 2], grid: &Grid, steps: usize) -> usize {   
    // we know that the input has some trenches of empty space
    // in the outer border and centered on start, and so the minimum period between maps
    // is just the width/height of the initial map

    let w = grid.ncols();
    let h = grid.nrows();
    assert_eq!(w, h);
    let d = w;

    // Given a cell (i,j), we reach their clones whenever we move from a map to another.
    // We index each map with (I,J).
    // Given that maps are odd, the parity of the cell changes accordingly.
    // When moving N maps at, say, direction (1,0),
    // then that cell is within
    //      d(S, c) = argmin (d(S, B0) + N*d(B0, B1) + d(B1, c))
    // where B0 correspond to a cell in the bottom trench and B1 a cell in the bottom trench of next bottom map

    // we also know that h = w, and so it is the same in terms of period to move one direction or the other

    // at a really big distance (N > w), the term d(B0, B1) dominates and so only those columns totally
    // free are to be considered, and by construction this map has the cross and the middle rows as empty

    assert!(steps > 2*w); // sanity check

    let empty_rows = grid.rows().into_iter().enumerate().filter_map(|(i, m)| m.iter().all(|n| *n == Node::Empty).then_some(i)).collect_vec();
    assert_eq!(&empty_rows, &[0, start[0] as usize, d - 1]);
    let empty_cols = grid.columns().into_iter().enumerate().filter_map(|(j, m)| m.iter().all(|n| *n == Node::Empty).then_some(j)).collect_vec();
    assert_eq!(&empty_cols, &[0, start[1] as usize, d - 1]);

    // this makes the choice of B0 and B1 to be easy, as there are only one possibility the row/column of start
    // a similar argument can be done when moving to a corner

    let last = (d-1) as i32;
    let ul = bfs_distance(&[0, 0], grid);
    let bl = bfs_distance(&[last, 0], grid);
    let ur = bfs_distance(&[0, last], grid);
    let br = bfs_distance(&[last, last], grid);
    let l = bfs_distance(&[start[0], 0], grid);
    let r = bfs_distance(&[start[0], last], grid);
    let u = bfs_distance(&[0, start[1]], grid);
    let b = bfs_distance(&[last, start[1]], grid);

    let start = (start[0] as usize, start[1] as usize);
    let parity = steps % 2;
    ul.indexed_iter().filter_map(|(p, d)| { (*d != u32::MAX).then_some(p) })
        .map(|p| {
            let mut result = 0;

            // map (0,0)
            let correct_parity = parity == (p.0.abs_diff(start.0) + p.1.abs_diff(start.1)) % 2;
            if correct_parity { result += 1; }

            // map moving to one of the main directions:
            for offset in [
                b[start] + u[p] + 1, // moving down
                u[start] + b[p] + 1, // moving up
                l[start] + r[p] + 1, // moving right
                r[start] + l[p] + 1, // moving left
            ]
            {
                if steps < offset as usize { continue; }
                let n = 1 + (steps - offset as usize) / d;
                result += n / 2; // only half of the maps will have correct parity
                if n % 2 == 1 && !correct_parity { result += 1; }    
            }

            // map moving to one of the main diagonals:
            for offset in [
                br[start] + ul[p] + 2,
                ur[start] + bl[p] + 2,
                bl[start] + ur[p] + 2,
                ul[start] + br[p] + 2,
            ]
            {
                if steps < offset as usize { continue; }
                let m = 1 + (steps - offset as usize) / d;
                if correct_parity {
                    // we sum up all odd numbers
                    let odd = (m+1) / 2;
                    result += odd * odd;
                }
                else {
                    // we sum up all even numbers
                    let even = m / 2;
                    result += even * (even + 1);

                }    
            }

            result
        }).sum()
}

fn part_two_impl(input: &str, steps: usize) -> Option<usize> {
    let (start, grid) = parse(input);
    let result = {
        if steps < 100 {
            count_modular(&start, &grid, steps)
        }
        else if grid.column(start[1] as usize).iter().all(|n| *n == Node::Empty) {
            //count_big_cross(&start, &grid, steps)
            extrapolate(&start, &grid, steps)
        }
        else {
            count_big_border(&start, &grid, steps)
        }
    };
    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_impl(input, 64)
}

pub fn part_two(input: &str) -> Option<usize> {
    part_two_impl(input, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_impl(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two_impl(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two_impl(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(50));
    }
    
    #[test]
    fn test_part_two_c() {
        let result = part_two_impl(&advent_of_code::template::read_file("examples", DAY), 50);
        assert_eq!(result, Some(1594));
    }

    #[test]
    fn test_part_two_d() {
        let result = part_two_impl(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(6536));
    }   

    #[test]
    fn test_part_two_e() {
        let result = part_two_impl(&advent_of_code::template::read_file("examples", DAY), 500);
        assert_eq!(result, Some(167004));
    }

    #[test]
    fn test_part_two_f() {
        let result = part_two_impl(&advent_of_code::template::read_file("examples", DAY), 1000);
        assert_eq!(result, Some(668697));
    }

    #[test]
    fn test_part_two_g() {
        let result = part_two_impl(&advent_of_code::template::read_file("examples", DAY), 5000);
        assert_eq!(result, Some(16733044));
    }

}
