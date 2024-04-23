use rayon::prelude::*;
use std::collections::HashMap;
use crate::Graph;
use dashmap::DashMap;
// Example using rayon to parallelize degree calculation
pub fn calculate_degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let distribution = DashMap::new();
    graph.edge_list.par_iter().for_each(|&(from_node, to_node)| {
        distribution.entry(from_node).and_modify(|e| *e += 1).or_insert(1);
        distribution.entry(to_node).and_modify(|e| *e += 1).or_insert(1);
    });

    // Convert DashMap to HashMap for return
    distribution.into_iter().collect()
}
/// Finds the top 10% most connected and isolated nodes.
pub fn find_top_percent(distribution: &HashMap<usize, usize>, percent: usize) -> (Vec<usize>, Vec<usize>) {
    let num_nodes = distribution.len();
    let threshold = std::cmp::max(1, num_nodes * percent / 100);  // Ensure at least 1

    println!("Threshold for top percent calculation: {}", threshold);

    let mut nodes_by_degree: Vec<_> = distribution.iter().collect();
    nodes_by_degree.sort_by(|a, b| b.1.cmp(a.1));

    // Print sorted nodes and their degrees to verify
    println!("Sorted Nodes by Degree:");
    for (node, degree) in nodes_by_degree.iter().take(10) {
        println!("Node {}, Degree {}", node, degree);
    }

    let most_connected = nodes_by_degree.iter().take(threshold).map(|(&node, &_)| node).collect::<Vec<usize>>();
    let least_connected = nodes_by_degree.iter().rev().take(threshold).map(|(&node, &_)| node).collect::<Vec<usize>>();

    (most_connected, least_connected)
}



