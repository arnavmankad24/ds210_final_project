mod graph;
mod bfs;

use graph::Graph;
use bfs::{average_distance, max_distance, mode_distance, distribution_percentage};
use crate::graph::ListOfEdges;

fn main() {
    
    let file_path = "euroroad.csv"; // euroroad.csv is the file contains the nodes and edges I am using in the project
    let n = 1174; // There are 1174 vertices according to 'http://konect.cc/networks/subelj_euroroad/' ranging from 0-1173 (shifted from 1-1174)

    // creating a graph using the read_csv functionc
    let graph = match Graph::read_csv(file_path, n) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error creating graph: {}", e);
            return; // or handle the error further
        }
    };
        
    for i in 1..=25 {
        let avg_distance = average_distance(&graph, i); // calculating the average distance for each degree of separation
        let max_distance = max_distance(&graph, i); // calculating the maximum distance for each degree of separation
        let mode_distance = mode_distance(&graph, i); // calculating the mode distance for each degree of separation
        let distribution_percentage = distribution_percentage(&graph, i); // calculating the percentage of valid pairs for each degree of separation
        println!("{} degrees of separation:", i); // displaying the number of degrees of separation
        println!("Average distance: {}", avg_distance); // displaying the average distance
        println!("Maximum distance: {}", max_distance); // displaying the maximum distance
        println!("Most commonly occurring distance: {}", mode_distance); // displaying the mode distance
        println!("Percentage of vertex pairs with valid distances: {} %", distribution_percentage); // displaying the maximum distance
        println!(); // adding a gap between data points for each degree of separation
    }
}

// the function below creates a sample graph which I will use for my tests
fn sample_graph() -> Graph {
    
    let n: usize = 5;
    let mut edges: ListOfEdges = vec![(0,1),(1,2),(2,3),(3,4)];
    edges.sort();
    let graph2 = Graph::undirected(n,&edges);

    graph2 // returning the graph
}

// the code below tests average_distance
#[test]
fn test_average_distance() {
    let graph2 = sample_graph();
    assert_eq!(average_distance(&graph2, 1), 1.0); // for degree of separation 1 the average is supposed to be 1
}

// the code below tests max_distance
#[test]
fn test_max_distance() {
    let graph2 = sample_graph();
    assert_eq!(max_distance(&graph2, 2), 2); // for degree of separation 2 the max is 2
}

// the code below tests mode_distance
#[test]
fn test_mode_distance() {
    let graph2 = sample_graph();
    assert_eq!(mode_distance(&graph2, 2), 1); // we expect the mode distance to be 1
}

// the code below tests distribution_percent
#[test]
fn test_distribution_percentage() {
    let graph2 = sample_graph();
    let expected = 80.0; // when the degree is 4, we expect all of the paths to work except (0<->0), (1<->1), (2<->2), (3<->3), (4<->4)
    assert_eq!(distribution_percentage(&graph2, 4), expected); // comparing the function value with the expected
}
