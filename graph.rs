use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::num::ParseIntError;

pub struct Graph {
    pub nodes: usize,
    pub edges: usize,
    pub edge_list: Vec<(usize, usize)>,
}
impl Graph {
    pub fn new(nodes: usize, edges: usize, edge_list: Vec<(usize, usize)>) -> Self {
        Graph { nodes, edges, edge_list }
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
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new(5);
        graph.add_edge(1, 2);
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0], (1, 2));
    }
}