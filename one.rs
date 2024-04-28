use crate::one::io::BufReader;
use std::fs::File;
use crate::Path;
use std::io;
use std::io::BufRead;
pub fn print_first_10_entries(file_path: &Path) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        if !line.starts_with("#") {  // Ignore comment lines
            println!("{}", line);
            count += 1;
            if count >= 10 {
                break;  // Stop after printing 10 entries
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test_print_first_edge() {
        let mut graph = Graph::new(2);
        assert_eq!(print_first_edge(&graph), None);
        graph.add_edge(0, 1);
        assert_eq!(print_first_edge(&graph), Some((0, 1)));
    }
}