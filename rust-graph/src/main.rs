mod parse;
mod buildgraph;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use petgraph::graph::DiGraph;
use petgraph::visit::{Dfs, Bfs};

// Function to compute the degree centrality of each node
fn compute_degree_centrality(graph: &DiGraph<u64, ()>) -> HashMap<u64, usize> {
    let mut centrality: HashMap<u64, usize> = HashMap::new();

    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count();
        centrality.insert(graph[node], degree);
    }

    centrality
}

//Function to find the top 10 patents with the most references to other patents.
fn top_10_highly_connected_nodes(graph: &DiGraph<u64, ()>) -> Vec<(u64, usize)> {
    let centrality = compute_degree_centrality(graph);
    let mut centrality_vec: Vec<(u64, usize)> = centrality.into_iter().collect();

    // Sort by centrality in descending order
    centrality_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Select top 10
    centrality_vec.into_iter().take(10).collect()
}

// Function to calculate the density of the graph
fn calculate_graph_density(graph: &DiGraph<u64, ()>) -> f64 {
    let node_count = graph.node_count() as f64;
    let edge_count = graph.edge_count() as f64;

    if node_count > 1.0 {
        edge_count / (node_count * (node_count - 1.0))
    } else {
        0.0
    }
}

// Function for Depth First Search traversal
fn dfs_traversal(graph: &DiGraph<u64, ()>, start_node: u64) -> HashSet<u64> {
    let start_index = graph.node_indices().find(|&n| graph[n] == start_node).unwrap();
    let mut dfs = Dfs::new(graph, start_index);
    let mut visited = HashSet::new();

    while let Some(nx) = dfs.next(graph) {
        visited.insert(graph[nx]);
    }

    visited
}

// Function for Breadth First Search traversal
fn bfs_traversal(graph: &DiGraph<u64, ()>, start_node: u64) -> HashSet<u64> {
    let start_index = graph.node_indices().find(|&n| graph[n] == start_node).unwrap();
    let mut bfs = Bfs::new(graph, start_index);
    let mut visited = HashSet::new();

    while let Some(nx) = bfs.next(graph) {
        visited.insert(graph[nx]);
    }

    visited
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("cit-Patents.txt");
    let edges = parse::parse_file(path)?;
    let graph = buildgraph::build_graph(&edges);

    println!("Graph has {} nodes and {} edges", graph.node_count(), graph.edge_count());
    
    // Top 10 highly connected nodes
    let top10nodes = top_10_highly_connected_nodes(&graph);
    println!("Top 10 highly connected patents:");
    for (node, degree) in top10nodes {
        println!("Patent {}: Centrality {}", node, degree);
    }

    // Graph Density
    let density = calculate_graph_density(&graph);
    println!("Graph density: {}", density);
    
    // Choose a starting patent number for traversal
    let start_patent: u64 = 3858241; // Replace with a patent number from the dataset

    // Perform DFS traversal
    let dfs_result = dfs_traversal(&graph, start_patent);
    println!("DFS Traversal from patent {}: {:?}", start_patent, dfs_result);

    // Perform BFS traversal
    let bfs_result = bfs_traversal(&graph, start_patent);
    println!("BFS Traversal from patent {}: {:?}", start_patent, bfs_result);


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    // Helper function to create a test graph
    fn create_test_graph() -> DiGraph<u64, ()> {
        let mut graph = DiGraph::new();
        let n1 = graph.add_node(1); // Node with index 0
        let n2 = graph.add_node(2); // Node with index 1
        let n3 = graph.add_node(3); // Node with index 2

        graph.add_edge(n1, n2, ()); // Edge 1->2
        graph.add_edge(n2, n3, ()); // Edge 2->3
        graph.add_edge(n3, n1, ()); // Edge 3->1

        graph
    }

    #[test]
    fn test_calculate_graph_density() {
        let graph = create_test_graph();
        let density = calculate_graph_density(&graph);
        let expected_density = 3.0 / (3.0 * (3.0 - 1.0)); // 3 edges, 3 nodes
        assert_eq!(density, expected_density);
    }

    #[test]
    fn test_top_10_highly_connected_nodes() {
        let graph = create_test_graph();
        let top_10 = top_10_highly_connected_nodes(&graph);
        assert_eq!(top_10.len(), 3); // The test graph has 3 nodes
        // Since all nodes have the same centrality, the order might not be guaranteed
    }

    #[test]
    fn test_dfs_traversal() {
        let graph = create_test_graph();
        let start_node = 1; // Node with value 1
        let dfs_result = dfs_traversal(&graph, start_node);
        assert_eq!(dfs_result, HashSet::from([1, 2, 3])); // Should visit all nodes
    }

    #[test]
    fn test_bfs_traversal() {
        let graph = create_test_graph();
        let start_node = 1; // Node with value 1
        let bfs_result = bfs_traversal(&graph, start_node);
        assert_eq!(bfs_result, HashSet::from([1, 2, 3])); // Should visit all nodes
    }
}
