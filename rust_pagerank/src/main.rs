use postgres::{Client, NoTls, Error};
// use::std::thread;
// use std::rc::Rc;


// a function that performs a single page rank iteration
fn pagerank_iteration<'a>(
    rank: &'a Vec<f64>,
    rank_new: &'a mut Vec<f64>,
    adjacency_matrix: &'a Vec<Vec<u32>>,
    out_nodes_num: &'a Vec<u32>,
    d: &f64,
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


fn main() -> Result<(), Error> {
    let dampening_factor: f64 = 0.8;
    let num_iterations: u32 = 40;

    // initializing connection to database
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/acs_db", NoTls)?;

    // counting the total number of websites indexed
    let mut x: i64 = client.query("SELECT count(*) FROM pagerank", &[])?[0].get(0);
    x += 1;

    let total_websites: usize = x as usize;

    println!("Total websites: {}", total_websites);

    // initializing the adjacency matrix
    let mut adjacency_matrix: Vec<Vec<u32>> = vec![vec![]; total_websites];

    // saving all edges in memory as iterator, and iterating over it
    for row in client.query("SELECT * FROM connections", &[])? {
        let out_website_id: i32 = row.get(0);
        let in_website_id: i32 = row.get(1);

        // Filtering out self links, and adding them to adjacency matrix
        if in_website_id != out_website_id {
            adjacency_matrix[in_website_id as usize].push(out_website_id as u32);
        }
        // else {
        // println!("Bad id: {}", in_website_id);
        // }
    }

    // we dont need mutable thingy anymore
    let adjacency_matrix = adjacency_matrix;


    // initialisation value for all ranks
    let init_rank: f64 = 1.0 / total_websites as f64;

    // Initialising two vectors with default values(init_ran and 0.0)
    let mut rank: Vec<f64> = vec![init_rank; total_websites];
    let mut rank_new: Vec<f64> = vec![0.0; total_websites];


    // initialising the number of out nodes based on adjacency matrix
    let mut out_nodes_num: Vec<u32> = vec![0; total_websites];
    for i in 0..total_websites {
        for website in 0..adjacency_matrix[i].len() {
            out_nodes_num[adjacency_matrix[i][website] as usize] += 1;
        }
    }

    println!("Adj mrtx {:?}", adjacency_matrix);
    println!("Out nodes {:?}", out_nodes_num);

    println!("{:?}", rank);
    println!("{:?}", rank_new);
    for _iteration in 0..num_iterations {
        {
            pagerank_iteration(&rank, &mut rank_new, &adjacency_matrix, &out_nodes_num, &dampening_factor);
        }
        // now we can just make rank to hold new rank without copying
        std::mem::swap(&mut rank, &mut rank_new);
        println!("After iteration {}:", _iteration);
        println!("{:?}", rank);
    }

    println!("\nFinal rankings: {:?}", rank);

    Ok(())
}
