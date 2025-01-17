# Scientific Collaboration Network Analysis

This project explores a network graph representing scientific collaborations, using graph theory principles to analyze researcher connectivity, centrality, and collaboration patterns. The insights can help identify key researchers, understand community structures, and promote informed decision-making in scientific research.

## Project Objectives
- **Node Degree Analysis**: Calculate the connectivity degree of researchers to identify highly connected and less-connected individuals.
- **Statistical Summary**: Compute and analyze the mean and median degree of collaboration among researchers.

## Methodology
1. **Data Collection**:
   - Gathered data on co-authored scientific papers, detailing relationships between researchers.
2. **Graph Construction**:
   - Built a network graph with:
     - **Nodes**: Representing researchers.
     - **Edges**: Representing co-authorships.
3. **Degree Distribution Analysis**:
   - Calculated degree metrics to identify prominent researchers and collaboration patterns.

## Results
### Degree Distribution
- **Most Connected Researchers** (Top 1% by degree):
  ```
  [21012, 21281, 12365, 22691, 9785, 6610, ...]
  ```
- **Least Connected Researchers** (Top 1% by lowest degree):
  ```
  [22435, 7055, 23480, 15422, 18151, 21288, ...]
  ```

### Statistical Summary
- **Mean Degree**: 11.06
- **Median Degree**: 6

## Conclusion
This analysis demonstrates how network graph metrics can uncover collaboration dynamics and key individuals in scientific research. These insights can assist funding agencies, academic institutions, and policymakers in fostering innovation and scientific collaboration.

## Usage
1. **Run Analysis**:
   Use the provided code to analyze a dataset of scientific collaborations.
2. **Customize Metrics**:
   Modify analysis parameters to explore specific aspects of the network graph.
3. **Visualize Results**:
   Extend the project with visualization tools to display the network structure and prominent nodes.

