use std::iter::once;
use std::fs::read_to_string;

use itertools::Itertools;
use petgraph::prelude::NodeIndex;
use petgraph::graph::UnGraph;
use rustc_hash::FxHashSet;

use crate::etc::id_assigner::IDAssigner;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type IdMap<'a> = IDAssigner<&'a str, u32>;
type ConnGraph = UnGraph<u32, ()>;
type NodeSet = FxHashSet<NodeIndex>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day23.txt").unwrap();
    let (id_map, graph) = parse(&input);

    let sol1 = find_sets_p1(&id_map, &graph);
    let sol2 = find_largest_clique(&id_map, &graph);

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_sets_p1(id_map: &IdMap, graph: &ConnGraph) -> usize {
    // Find the nodes that start with 't'
    let ts = graph.node_indices().filter(
        |i| id_map.get_elem(i.index() as u32).unwrap().starts_with('t')
    );
    let mut found_sets = FxHashSet::default();

    // Find pairs of nodes connected to those ones so that the pair is connected too
    for ix in ts {
        let neighbors = graph.neighbors(ix);
        let pairs = neighbors.tuple_combinations();
        let connected_pairs = pairs.filter(|&(a, b)| graph.contains_edge(a, b));
        for (a, b) in connected_pairs {
            // Sort the 3 vertices to avoid inserting permutations of the same group
            let group = (ix.index(), a.index(), b.index());
            found_sets.insert(sort(group));
        }
    }

    found_sets.len()
}

fn find_largest_clique(id_map: &IdMap, graph: &ConnGraph) -> String {
    // Find the largest clique (complete subgraph) in the provided graph using the Bron–Kerbosch algorithm.
    // Returns the names of the nodes in the clique, sorted alphabetically, separated by commas.
    let (r, x) = (NodeSet::default(), NodeSet::default());
    let p = graph.node_indices().collect();
    let max_clique = bron_kerbosch((r, p, x), graph);

    max_clique.into_iter().map(|i| id_map.get_elem(i.index() as u32).unwrap())
        .sorted()
        .join(",")
}

fn bron_kerbosch((r, mut p, mut x): (NodeSet, NodeSet, NodeSet), g: &ConnGraph) -> NodeSet {
    if p.is_empty() && x.is_empty() {
        return r;
    }

    let mut best = NodeSet::default();

    let pivot = *(p.iter().chain(x.iter())).next().unwrap();
    let p_neighbors = g.neighbors(pivot).collect();
    let to_visit = p.difference(&p_neighbors).copied().collect_vec();

    for v in to_visit {
        let neighbors = g.neighbors(v).collect();

        let r2 = r.iter().chain(once(&v)).copied().collect();
        let p2 = p.intersection(&neighbors).copied().collect();
        let x2 = x.intersection(&neighbors).copied().collect();

        let clique = bron_kerbosch((r2, p2, x2), g);
        if clique.len() > best.len() {
            best = clique;
        }

        p.remove(&v);
        x.insert(v);
    }

    best
}

fn parse(input: &str) -> (IdMap, ConnGraph) {
    let mut id_map: IdMap = IDAssigner::new();
    let edges = input.lines()
        .map(|line| {
            let (l, r) = line.split_once('-').unwrap();
            (id_map.get_id(l), id_map.get_id(r))
        });
    let graph = UnGraph::from_edges(edges);
    (id_map, graph)
}

fn sort((a, b, c): (usize, usize, usize)) -> (usize, usize, usize) {
    let mut v = vec![a, b, c];
    v.sort();
    v.into_iter().collect_tuple().unwrap()
}
