mod parse;
mod buildgraph;

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("cit-Patents.txt");
    let edges = parse::parse_file(path)?;
    let graph = buildgraph::build_graph(&edges);

    println!("Graph has {} nodes and {} edges", graph.node_count(), graph.edge_count());

    Ok(())
}
