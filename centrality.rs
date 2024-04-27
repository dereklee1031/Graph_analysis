use std::collections::{HashMap, HashSet, VecDeque};
use crate::graph::Graph;

pub fn calculate_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let mut centrality = HashMap::new();
    let nodes: HashSet<_> = graph.edge_list.iter().flat_map(|&(from, to)| vec![from, to]).collect();

    // Initialize centrality scores to zero for all nodes
    for &node in &nodes {
        centrality.insert(node, 0.0);
    }

    for s in &nodes {
        let mut stack = Vec::new();
        let mut paths = HashMap::new();
        let mut num_paths = HashMap::new();
        let mut distance = HashMap::new();
        let mut dependency = HashMap::new();

        // Initialization
        for &node in &nodes {
            paths.insert(node, Vec::new());
            num_paths.insert(node, 0.0);
            distance.insert(node, -1);
            dependency.insert(node, 0.0);
        }

        *num_paths.get_mut(s).unwrap() = 1.0;
        *distance.get_mut(s).unwrap() = 0;

        let mut queue = VecDeque::new();
        queue.push_back(*s);

        // Step 1: Single-source shortest paths problem
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            let v_distance = *distance.get(&v).unwrap();
            for &(from, to) in &graph.edge_list {
                if from == v {
                    let to_distance = distance.get(&to).unwrap();
                    if *to_distance == -1 {
                        distance.insert(to, v_distance + 1);
                        queue.push_back(to);
                    }
                    if *distance.get(&to).unwrap() == v_distance + 1 {
                        let num_paths_v = *num_paths.get(&v).unwrap();  // Get the number of paths to 'v' first
                        *num_paths.get_mut(&to).unwrap() += num_paths_v;  // Then update the number of paths to 'to'
                        paths.get_mut(&to).unwrap().push(v);
                    }
                }
            }
        }

        // Step 2: Accumulation
        while let Some(w) = stack.pop() {
            for v in paths.get(&w).unwrap() {
                let delta_v = dependency.get(v).unwrap();
                let delta_w = dependency.get(&w).unwrap();
                let num_paths_v = num_paths.get(v).unwrap();
                let num_paths_w = num_paths.get(&w).unwrap();

                let contribution = (num_paths_v / num_paths_w) * (1.0 + delta_w);
                *dependency.get_mut(v).unwrap() += contribution;
            }
            if w != *s {
                *centrality.get_mut(&w).unwrap() += dependency.get(&w).unwrap();
            }
        }
    }

    centrality
}

pub fn node_with_highest_centrality(centrality: &HashMap<usize, f64>) -> Option<usize> {
    centrality.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)).map(|(&node, _)| node)
}

