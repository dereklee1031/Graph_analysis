use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::num::ParseIntError;
use std::collections::VecDeque;

pub struct Graph {
    pub nodes: usize,
    pub edges: usize,
    pub edge_list: Vec<(usize, usize)>,
    pub adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(nodes: usize, edges: usize, edge_list: Vec<(usize, usize)>) -> Self {
        Graph { nodes, edges, edge_list, adjacency_list: vec![Vec::new(); nodes] }
    }
    pub fn add_edge(&mut self, from_node: usize, to_node: usize) {
        self.edge_list.push((from_node, to_node));
        self.adjacency_list[from_node].push(to_node);
        self.adjacency_list[to_node].push(from_node); // Assuming the graph is undirected
        self.edges += 1;
    }
/// Finds and returns all connected components as a vector of vectors.
    pub fn find_connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.nodes];
        let mut components = Vec::new();

        for start in 0..self.nodes {
            if !visited[start] {
                let mut component = Vec::new();
                let mut stack = VecDeque::new();
                stack.push_back(start);

                while let Some(node) = stack.pop_back() {
                    if !visited[node] {
                        visited[node] = true;
                        component.push(node);
                        for &neighbor in &self.adjacency_list[node] {
                            if !visited[neighbor] {
                                stack.push_back(neighbor);
                            }
                        }
                    }
                }

                components.push(component);
            }
        }

        components
    }
}

pub fn read_graph(file_path: &Path) -> io::Result<Graph> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut nodes = 0;
    let mut edges = 0;
    let mut edge_list: Vec<(usize, usize)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("#") {
            if line.contains("Nodes:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                nodes = parts[2].parse().map_err(parse_error_to_io_error)?;
                edges = parts[4].parse().map_err(parse_error_to_io_error)?;
            }
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let from_node = parts[0].parse().map_err(parse_error_to_io_error)?;
            let to_node = parts[1].parse().map_err(parse_error_to_io_error)?;
            edge_list.push((from_node, to_node));
        }
    }

    Ok(Graph::new(nodes, edges, edge_list))
}

fn parse_error_to_io_error(e: ParseIntError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_initialization() {
        let graph = Graph::new(5);
        assert_eq!(graph.nodes, 5);
        assert_eq!(graph.edges, 0);
        assert!(graph.adjacency_list.iter().all(|neighbors| neighbors.is_empty()));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new(5);
        graph.add_edge(1, 2);
        assert_eq!(graph.edges, 1);
        assert_eq!(graph.edge_list, vec![(1, 2)]);
        assert_eq!(graph.adjacency_list[1], vec![2]);
        assert_eq!(graph.adjacency_list[2], vec![1]);
    }
}

















