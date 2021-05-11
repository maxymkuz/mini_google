use::std::thread;
use std::rc::Rc;

// a function that performs a single page rank iteration
fn pagerank_iteration<'a>(
    rank: &'a Vec<f64>,
    rank_new: &'a mut Vec<f64>,
    adjacency_matrix: &'a Vec<Vec<u32>>,
    out_nodes_num: &'a Vec<u32>,
    d: &f64
) {
    for node_idx in 0..rank.len() {
        let mut sum: f64 = 0 as f64;
        // calculating the new rank of the node based on adjacency matrix
        for in_node_idx in &adjacency_matrix[node_idx] {
            sum += rank[*in_node_idx as usize] / out_nodes_num[*in_node_idx as usize] as f64;
        }
        rank_new[node_idx as usize] = (1.0 - d) + d * sum;
    }
    // println!("INSIDE PAGERANK: {:?}", rank);
    // println!("INSIDE PAGERANK: {:?}", rank_new);
}


fn main() {

    // the total number of websites we will use in iteration
    let total_websites: usize = 5;
    let dampening_factor:f64 = 0.8;

    let num_iterations:u32 = 10;

    // initialization value for all ranks
    let init_rank: f64 = 1.0 / total_websites as f64;

    // Initialising two vectors with default values(init_ran and 0.0)
    let mut rank: Vec<f64> = vec![init_rank; total_websites];
    let mut rank_new: Vec<f64> = vec![0.0; total_websites];

    let mut adjacency_matrix: Vec<Vec<u32>> = vec![vec![]; total_websites];
    // artificially making up node connections until we have some real-world data

    adjacency_matrix[0] = vec![3];
    adjacency_matrix[1] = vec![0];
    adjacency_matrix[2] = vec![1];
    adjacency_matrix[3] = vec![0, 1, 4];
    adjacency_matrix[4] = vec![];

    // initialising the number of out nodes based on adjacency matrix
    let mut out_nodes_num: Vec<u32> = vec![0; total_websites];
    for i in 0..total_websites {
        for website in 0..adjacency_matrix[i].len() {
            out_nodes_num[adjacency_matrix[i][website] as usize] += 1;
        }
    }
    // we dont need mutable thingy anymore
    let adjacency_matrix = adjacency_matrix;

    println!("Adj mrtx {:?}", adjacency_matrix);
    println!("Out nodes {:?}", out_nodes_num);

    println!("{:?}", rank);
    println!("{:?}", rank_new);
    for _iteration in 0..num_iterations {
        {
            pagerank_iteration(&rank, &mut rank_new, &adjacency_matrix, &out_nodes_num, &dampening_factor);
            // Multithreading is not implemented yet
            // let mut vec = Vec::new();
            // vec.push(thread::spawn( || {pagerank_iteration(&rank, &mut rank_new, &adjacency_matrix, &out_nodes_num, &dampening_factor)}));
            // for thread in vec {
            //     thread.join().unwrap();
            // }

        }
        // println!("{:?}", rank);
        // println!("{:?}", rank_new);
        // now we can just make rank to hold new rank without copying
        std::mem::swap(&mut rank, &mut rank_new);
        println!("After iteration {}:", _iteration);
        println!("{:?}", rank);
    }

    println!("\nFinal rankings: {:?}", rank);
}
