mod graph;
mod degree_distribution;
mod one;

use std::path::Path;
use crate::graph::Graph;
use crate::graph::read_graph;
use crate::one::print_first_10_entries;
use crate::degree_distribution::{calculate_degree_distribution, find_top_percent};

fn main() -> std::io::Result<()> {
    let path = Path::new("/Users/dereklee/Desktop/DS 210/final_project/ca-GrQc.txt");

    let graph = read_graph(&path)?;
    print_first_10_entries(&path)?;

    let distribution = calculate_degree_distribution(&graph);
    if distribution.is_empty() {
        println!("Degree distribution is empty.");
        return Ok(());
    }

    let (most_connected, most_isolated) = find_top_percent(&distribution, 10);
    
    println!("Top 10% Most Connected Nodes:");
    for node in most_connected {
        println!("Node {}: Degree {}", node, distribution.get(&node).unwrap_or(&0));
    }

    println!("Top 10% Most Isolated Nodes:");
    for node in most_isolated {
        println!("Node {}: Degree {}", node, distribution.get(&node).unwrap_or(&0));
    }

    Ok(())
}



