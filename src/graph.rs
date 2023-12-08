// note that many functions used in this file came from the class notes corresponding to lecture 28
// note that part of the read_csv function has come from 'https://crates.io/crates/csv' which was shown in the lecture notes

use std::fs::File; // allows me to work with files
use csv::ReaderBuilder; // allows me to read csv files (the format euroroad is in)
use std::error::Error; // allows me to handle errors

pub type Vertex = usize; // usize represents vertices
pub type ListOfEdges = Vec<(Vertex,Vertex)>; // edges in the graph are represented by a vector of tuples
pub type AdjacencyLists = Vec<Vec<Vertex>>; // vector of a vector represents adjacency lists

// below I define a Graph struct with two fields: n is the number of vertices and outedges represents adjacency lists for each vertex (vertex with neighbors)
#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
}

// the function below allows me to reverse the nodes to create an undirected graph
pub fn edge_reversal(list:&ListOfEdges) -> ListOfEdges { // the inpute is a list of edges
    let mut new_list = vec![]; // creating a vector called new_list
    for (u,v) in list {
        new_list.push((*v,*u)); // reverses the vertices in "list" and adds them to "new_list"
    }
    new_list // returns the new list
}

// implementing the graph stuct and creating methods that will allows us to convert a csv file of highways to an undirected graph
impl Graph {
    
    // the method below creates a new instance of the Graph struct
    // note that this method is not used in main.rs because I use read_csv and "undirected" to create the graph, which is why it is commented out
    //pub fn new(n: usize, outedges: Vec<Vec<Vertex>>) -> Self {
        //Graph { n, outedges }
    //}

    
    // the function below iterates over directed edges and adds the neighbor (v) to the adjacency list of u (the vertex/current node)
    pub fn add_directed_edges(&mut self, edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v); // adding v to the adjacency list of u
        }
    }

    // the function below sorts the adjacency lists in ascending order
    pub fn sort(&mut self) {
        for neighbors in self.outedges.iter_mut() {
            neighbors.sort(); // sorting each list of neighbors in ascending numerical order
        }
    }

    // creates a directed graph (one way, meaning A -> B does not imply B -> A) ~ this graph will be used to create the undirected graph
    pub fn directed(n:usize,edges:&ListOfEdges) -> Graph {
        let mut graph = Graph{n,outedges:vec![vec![];n]}; // intializes a graph with n vertices and outedges that are currently empty
        graph.add_directed_edges(edges); // using a list of edges from the graph to add appropriate vertices to adjacency lists
        graph.sort(); // sorts the adjacency lists to that the values within them are in ascending order
        graph // returning the directed graph                                         
    }
    
    // creates an undirected graph using the directed graph created with the function above
    pub fn undirected(n:usize,edges:&ListOfEdges) -> Graph {
        let mut graph = Self::directed(n,edges); // initializing a directed graph using the create_directed function above
        graph.add_directed_edges(&edge_reversal(edges)); // using the reverse_edges function to add "directed edges" in reverse
        graph.sort(); // sorting the adjacency lists in ascending order
        graph // returning the undirected graph                                        
    }

    // the function below reads a csv file containing undirected edges and converts it into an undirected graph using the functions written above
    pub fn read_csv(file_path: &str, n: usize) -> Result<Self, Box<dyn Error>> {
        let mut edges = Vec::new(); // creating a new vector of edges
        let file = File::open(file_path)?; // opening the csv file
        let mut rdr = ReaderBuilder::new().from_reader(file); // initializing a CSV reader

        for result in rdr.records() { // iterating through each edge in the CSV file
            let record = result?; // record is the name for each edge
            let vertices: Vec<&str> = record.iter().map(|s| s).collect(); // creating a vector containing the two vertices in the line
            let node1: usize = vertices[0].parse()?; // letting node1 equal the first vertex
            let node2: usize = vertices[1].parse()?; // letting node2 equal the second vertex
            edges.push((node1 - 1, node2 - 1)); // adding a tuple of the two vertices to the edges vector (adjusting the node values to 0-1173 instead of 1-1174)
        }

        let graph = Graph::undirected(n, &edges); // creating an undirected graph using the edges
        Ok(graph) // returning the graph
    }
}