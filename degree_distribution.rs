use std::collections::HashMap;
use rayon::prelude::*;
use dashmap::DashMap;
use crate::graph::Graph;

pub fn calculate_degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let distribution = DashMap::new();
    graph.edge_list.par_iter().for_each(|&(from_node, to_node)| {
        distribution.entry(from_node).and_modify(|e| *e += 1).or_insert(1);
        distribution.entry(to_node).and_modify(|e| *e += 1).or_insert(1);
    });

    distribution.into_iter().collect()
}

pub fn find_top_percent(distribution: &HashMap<usize, usize>, percent: usize) -> (Vec<usize>, Vec<usize>) {
    let num_nodes = distribution.len();
    let threshold = std::cmp::max(1, num_nodes * percent / 100);

    println!("Threshold for top percent calculation: {}", threshold);

    if num_nodes == 0 {
        println!("No nodes to process.");
        return (Vec::new(), Vec::new()); // Ensures return of correct type if no nodes
    }

    let mut nodes_by_degree: Vec<_> = distribution.iter().collect();
    nodes_by_degree.sort_by(|a, b| b.1.cmp(a.1));

    // Print sorted nodes and their degrees to verify
    println!("Sorted Nodes by Degree:");
    for (node, degree) in nodes_by_degree.iter().take(10) {
        println!("Node {}, Degree {}", node, degree);
    }

    let most_connected = nodes_by_degree.iter()
        .take(threshold)
        .map(|(&node, &_degree)| node)
        .collect::<Vec<usize>>();
    let least_connected = nodes_by_degree.iter()
        .rev()
        .take(threshold)
        .map(|(&node, &_degree)| node)
        .collect::<Vec<usize>>();

    // Debug output before returning
    println!("Most connected (Top {}%): {:?}", percent, most_connected);
    println!("Least connected (Top {}%): {:?}", percent, least_connected);

    // Explicitly return the tuple
    return (most_connected, least_connected);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test_calculate_degree() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        let degrees = calculate_degree(&graph);
        assert_eq!(degrees, vec![1, 2, 1]);
    }
}

