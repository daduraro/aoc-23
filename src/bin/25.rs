use std::{collections::HashMap, f32::consts::SQRT_2};
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use disjoint_sets::UnionFind;

advent_of_code::solution!(25);

type Graph = Vec<Vec<usize>>;
type Edges = Vec<(usize, usize)>;

fn parse(input: &str) -> (HashMap<String, usize>, Graph) {
    let mut graph = Vec::new();

    let mut name_map = input.lines().enumerate().map(|(idx, line)| {
        (line.split_once(':').unwrap().0.to_owned(), idx)
    }).collect::<HashMap<String, usize>>();

    for line in input.lines() {
        let adj = line.split_once(':').unwrap().1;
        graph.push(adj.trim().split_ascii_whitespace().map(|k| {
            if let Some(idx) = name_map.get(k) {
                *idx
            }
            else {
                let idx = name_map.len();
                name_map.insert(k.to_owned(), idx);
                idx
            }
        }).collect());
    }
    graph.resize(name_map.len(), vec![]);

    (name_map, graph)
}

fn vertex_size(partition: &UnionFind) -> usize {
    let mut repr = partition.to_vec();
    repr.sort();
    repr.dedup();
    repr.len()
}

fn filter_edges(edges: &mut Edges, partition: &UnionFind) {
    edges.retain(|(a,b)| !partition.equiv(*a, *b));
}

fn collapse(mut edges: Edges, mut partition: UnionFind, t: usize) -> (Edges, UnionFind) {
    let mut v = vertex_size(&partition);

    while v > t {
        let (a, b) = edges.pop().unwrap();
        if partition.union(a, b) {
            v -= 1;
        }
    }
    filter_edges(&mut edges, &partition);
    (edges, partition)
}

fn rand_collapse(mut edges: Edges, partition: UnionFind) -> (Edges, UnionFind) {
    partition.force();
    let v = vertex_size(&partition);
    if v <= 6 {
        collapse(edges, partition, 2)
    }
    else {
        let t = 1 + (((v as f32) / SQRT_2).ceil() as usize);
        let (e1, p1) = collapse(edges.clone(), partition.clone(), t);
        let (e1, p1) = rand_collapse( e1, p1);

        edges.shuffle(&mut thread_rng());
        let (e2, p2) = collapse(edges, partition, t);
        let (e2, p2) = rand_collapse( e2, p2);

        if e1.len() <= e2.len() { (e1, p1) } else { (e2, p2) }
    }
}

// Karger–Stein algorithm for finding min-cut 
// https://en.wikipedia.org/wiki/Karger%27s_algorithm
// using union-find data structure
#[allow(dead_code)]
fn min_cut(graph: &Graph) -> u32 {
    let mut edges: Vec<(usize, usize)> = graph.iter().enumerate().flat_map(|(a, bs)| bs.iter().map(|b| (a, *b)).collect_vec() ).collect();
    edges.shuffle(&mut thread_rng());

    let (e, p) = rand_collapse(edges, UnionFind::new(graph.len()));
    
    assert_eq!(e.len(), 3);

    let mut vs = p.to_vec();
    vs.sort();
    let a = vs.partition_point(|x| *x == vs[0]);
    (a * (vs.len() - a)) as u32
}

#[allow(dead_code)]
fn iterativa_mincut(graph: &Graph) -> u32 {
    let mut edges: Vec<(usize, usize)> = graph.iter().enumerate().flat_map(|(a, bs)| bs.iter().map(|b| (a, *b)).collect_vec() ).collect();
    loop {
        edges.shuffle(&mut thread_rng());
        let (e, p) = collapse(edges.clone(), UnionFind::new(graph.len()), 2);
        if e.len() > 3 { continue; }
        
        let mut vs = p.to_vec();
        vs.sort();
        let a = vs.partition_point(|x| *x == vs[0]);
        return (a * (vs.len() - a)) as u32;   
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, g) = parse(input);
    Some(iterativa_mincut(&g))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
