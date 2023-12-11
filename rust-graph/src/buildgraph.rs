use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

// directed graph
pub fn build_graph(edges: &[(u64, u64)]) -> DiGraph<u64, ()> {
    let mut graph = DiGraph::new();
    let mut indices: HashMap<u64, NodeIndex> = HashMap::new();

    for &(from, to) in edges {
        let from_index = *indices.entry(from).or_insert_with(|| graph.add_node(from));
        let to_index = *indices.entry(to).or_insert_with(|| graph.add_node(to));
        graph.add_edge(from_index, to_index, ());
    }

    graph
}
