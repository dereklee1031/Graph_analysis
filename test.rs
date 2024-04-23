#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Undirected;

    #[test]
    fn test_empty_graph() {
        let graph: Graph<(), (), Undirected> = Graph::new_undirected();
        let distribution = calculate_degree_distribution(&graph);
        assert!(distribution.is_empty(), "The distribution should be empty for an empty graph.");
    }

    #[test]
    fn test_single_node() {
        let mut graph = Graph::new_undirected();
        graph.add_node(()); // Add a single node
        let distribution = calculate_degree_distribution(&graph);
        assert_eq!(distribution[&0], 0, "A single node with no edges should have a degree of 0.");
    }

    #[test]
    fn test_simple_graph() {
        let mut graph = Graph::new_undirected();
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        graph.add_edge(n1, n2, ()); // Add a single edge between two nodes
        let distribution = calculate_degree_distribution(&graph);
        assert_eq!(distribution[&n1.index()], 1, "Node n1 should have a degree of 1.");
        assert_eq!(distribution[&n2.index()], 1, "Node n2 should have a degree of 1.");
    }
}