mod graph;
mod degree_distribution;
mod one;
use std::path::Path;
use crate::graph::{Graph, read_graph};
use crate::one::print_first_10_entries;
use crate::degree_distribution::{calculate_degree_distribution, find_top_percent, mean_degree,median_degree};

fn main() -> std::io::Result<()> {
    let path = Path::new("/Users/dereklee/Desktop/DS 210/final_project/ca-GrQc.txt");
    let graph = read_graph(&path)?;
    // Ensure the graph is read correctly
    print_first_10_entries(&path)?;
    let distribution = calculate_degree_distribution(&graph);
    if distribution.is_empty() {
        println!("Degree distribution is empty.");
        return Ok(());
    }
    // Calculate and print the mean degree
    let mean = mean_degree(&distribution);
    println!("Mean degree of the graph is {:.2}", mean);

    // Calculate and print the median degree
    let median = median_degree(&distribution);
    println!("Median degree of the graph is {}", median);
    //Find most connected and most isolated
    let (most_connected, most_isolated) = find_top_percent(&distribution, 1);
    println!("Top 1% Most Connected Nodes:");
    for node in most_connected {
        println!("Node {}: Degree {}", node, distribution.get(&node).unwrap_or(&0));
    }
    println!("Top 1% Most Isolated Nodes:");
    for node in most_isolated {
        println!("Node {}: Degree {}", node, distribution.get(&node).unwrap_or(&0));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(), "Application is running");
    }
}
