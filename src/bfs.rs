use crate::graph::Graph;
use std::collections::VecDeque;
use std::collections::HashMap;
use crate::graph::Vertex;

pub fn bfs_distance(start: Vertex, graph: &Graph) -> Vec<Option<u32>> {
    let mut distance: Vec<Option<u32>> = vec![None; graph.n]; // intializing a vector distance with a length equaling the number of vertices
    distance[start] = Some(0); // Initialize the distance for the starting vertex 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start); // Add the starting vertex to the queue

    while let Some(v) = queue.pop_front() { // the loop continues as long as there are neighbors in the queue
        for u in graph.outedges[v].iter() { // iterating over neighbors of the current vertex
            if let None = distance[*u] { // chdcks if the neighbor has not been visited yet
                distance[*u] = Some(distance[v].unwrap() + 1); // incrementing the distance by 1
                queue.push_back(*u); // adding the neighbor to the queue so its neighbors can be explored
            }
        }
    }

    distance // Return the vector of distances
}

// the function below considers degree of separation ~ distances that are larger than the degree of separation get replaced with "None"
// note that distances that are equal to 0 also get set to "None" (i.e. going from Node A to Node A)
pub fn separation_degree(distance: Vec<Vec<Option<u32>>>, degree: u32) -> Vec<Vec<Option<u32>>> {
    let filtered_distances: Vec<Vec<Option<u32>>> = distance
        .iter() // iterating through each row of distances
        .map(|distances| {
            distances
                .iter() // iterating through each distance
                .map(|&d| if let Some(val) = d { if val <= degree && val > 0 { Some(val) } else { None } } else { None }) // setting invalid distances to "None"
                .collect()
        })
        .collect(); // collecting all the modified rows into a new array called "filtered_distances"

    filtered_distances // returning the new vector of vectors
}

// using the functions above to get the distances for all the nodes, while filtering out distances of 0 and those larger than the input degree
pub fn filtered_distances(graph: &Graph, degree: u32) -> Vec<Vec<Option<u32>>> {
    let mut distances = Vec::new(); // creating a vector which will contain distances from each node to all other nodes

    for i in 0..graph.n { // iterating through each  node
        let distances_from_i = bfs_distance(i, graph); // calculating the bfs distance for each vertex
        distances.push(distances_from_i); // adding the vector of distances to the "distances" vector
    }

    let filtered_distances = separation_degree(distances, degree); // filtering out distances larger than degree of separation
    filtered_distances // Return the vector containing distances from each node to all other nodes
}

// The function below calculates the average of valid distances for a given degree (ignores distances of 0 and None)
pub fn average_distance(graph: &Graph, degree: u32) -> f64 {
    let filtered_distances = filtered_distances(graph, degree); // using bfs and the filtering function to create a list of distances
    let mut total = 0; // represents the total distance
    let mut count = 0; // represents the number of valid vertex pairs

    for distances in &filtered_distances { // iterating through rows of distances
        for &d in distances { // for each distance in each row
            if let Some(val) = d { // checking if the distance value exists for the given degree
                total += val; // increments the total_distance by val
                count += 1; // incrementing the counter that represents the total number of pairs
            }
        }
    }

    if count > 0 {
        total as f64 / count as f64 // returns the average distance
    } 
    else {
        0.0 // Return 0 if no valid distances were found to avoid division by 0 (unlikely scenario)
    }
}

// the function below calculates the maximum distance for a given degree (ignoring distances of 0 and None)
pub fn max_distance(graph: &Graph, degree: u32) -> u32 {
    let filtered_distances = filtered_distances(graph, degree); // using bfs and the filtering function to create a list of distances
    let mut max = 0; // represents the maximum distance

    for distances in &filtered_distances { // iterating through the rows of distances
        for &d in distances { // for each distance in each row
            if let Some(val) = d { // checking if the distance value exists
                if val > max { // checks if that distance is larger than the maximum
                    max = val; // re-assigns the maximum
                }
            }
        }
    }

    max // returns the maximum distance
}

// the function below calculates the mode (most common) distance for a given degree (ignoring distances of 0 and None)
pub fn mode_distance(graph: &Graph, degree: u32) -> u32 {
    let filtered_distances = filtered_distances(graph, degree); // using bfs and the filtering function to create a list of distances
    let mut dist_frequency: HashMap<u32, u32> = HashMap::new(); // creating a HashMap that that keeps track of the count of each distance

    for distances in &filtered_distances { // iterating through the rows of distances
        for &d in distances { // for each distance in each row
            if let Some(val) = d { // checking if the distance value exists
                *dist_frequency.entry(val).or_insert(0) += 1; // adding val (new distance) as a key and incrementing the count of the particular distance by 1
            }
        }
    }

    let mode = dist_frequency
        .iter() // iterating through the HashMap
        .max_by_key(|&(_, count)| count) // getting the key that has the maximum value
        .unwrap() // accessing the tuple containing the mode
        .0; // extracting the value of the count and setting it to mode

    *mode // returning the mode
}

// the function below calculates the distribution percentage for a particular degree of separation (percentage of vertex pairs with valid distances between them)
pub fn distribution_percentage(graph: &Graph, degree: u32) -> f64 {
    let filtered_distances = filtered_distances(graph, degree); // using bfs and the filtering function to create a list of distances
    let mut total = 0; // represents total number of pairs
    let mut valid = 0; // represents the vertex pairs that do not have a "None" distance between them

    for distances in &filtered_distances { // iterating through the rows of distances
        for &d in distances { // for each distance in each row
            total += 1; // incrementing the number of total pairs by 1
            if let Some(_val) = d { // checking if the distance value exists
                valid += 1; // incrementing the number of valid pairs
            }
        }
    }

    if total > 0 {
        (valid as f64 / total as f64) * 100.0 // returns the percentage of valid edges
    } 
    else {
        0.0 // Return 0 if there are no pairs to avoid division by 0 (unlikely event)
    }
}