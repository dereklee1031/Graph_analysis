mod graph;
mod degree_distribution;
mod one;
use crate::graph::read_graph;
use crate::degree_distribution::find_top_percent;
use crate::degree_distribution::calculate_degree_distribution;
use std::io::BufReader;
use crate::graph::Graph;
use std::path::Path;
use crate::one::print_first_10_entries;


fn main() -> std::io::Result<()> {
    let path = Path::new("/Users/dereklee/Desktop/DS 210/final_project/ca-GrQc.txt");
    let graph = read_graph(&path)?;
    // Make sure data is being read
    print_first_10_entries(&path)?;

    // Calculate the degree distribution
    let distribution = calculate_degree_distribution(&graph);
    // Determine the top 10% most connected and most isolated nodes
    let (most_connected, most_isolated) = find_top_percent(&distribution, 10);
    
    // Print the degree of these nodes
    println!("Top 10% Most Connected Nodes:");
    for node in &most_connected {
        println!("Node {}: Degree {}", node, distribution.get(&node).unwrap_or(&0));
    }

    println!("Top 10% Most Isolated Nodes:");
    for node in &most_isolated {
        println!("Node {}: Degree {}", node, distribution.get(&node).unwrap_or(&0));
    }

    Ok(())
}



