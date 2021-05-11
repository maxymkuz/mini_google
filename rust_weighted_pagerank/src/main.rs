use postgres::{Client, NoTls, Error};

// a function that performs a single page rank iteration
fn pagerank_iteration<'a>(
    rank: &'a Vec<f64>,
    rank_new: &'a mut Vec<f64>,
    adjacency_matrix: &'a Vec<Vec<u32>>,
    weight_matrix: &'a Vec<Vec<f64>>,
    d: &f64,
) {
    for node_idx in 0..rank.len() {
        let mut sum: f64 = 0 as f64;

        // calculating the new rank of the node based on adjacency matrix and weights
        for idx in 0..adjacency_matrix[node_idx].len() {
            sum += rank[adjacency_matrix[node_idx][idx] as usize] * weight_matrix[node_idx][idx];
        }
        rank_new[node_idx as usize] = (1.0 - d) + d * sum;
    }

}


fn get_manhattan_distance(rank: &Vec<f64>, rank_new: &Vec<f64>) -> f64{
    let mut manhattan_distance: f64 = 0.0;
    for index in 0..rank.len() as usize {
        manhattan_distance += (rank[index] - rank_new[index]).abs();
    }
    manhattan_distance
}


fn main() -> Result<(), Error> {
    let dampening_factor: f64 = 0.8;
    let num_iterations: u32 = 40;

    // initializing connection to database
    let mut client = Client::connect(
        "postgresql://postgres:postgres@localhost:5433/acs_db",
        NoTls
    )?;

    // counting the total number of websites indexed
    let mut count: i64 = client.query("SELECT count(*) FROM pagerank", &[])?[0].get(0);
    count += 1;

    let total_websites: usize = count as usize;

    println!("Total websites: {}", total_websites);

    // initializing the adjacency matrix and weight matrix
    let mut adjacency_matrix: Vec<Vec<u32>> = vec![vec![]; total_websites];
    let mut weight_matrix: Vec<Vec<f64>> = vec![vec![]; total_websites];

    client.query("REFRESH MATERIALIZED VIEW counts;", &[])?;
    client.query("REFRESH MATERIALIZED VIEW weight;", &[])?;

    // saving all edges in memory as iterator, and iterating over it
    for row in client.query("SELECT * FROM weight", &[])? {
        let out_website_id: i32 = row.get(0);
        let in_website_id: i32 = row.get(1);
        let weight: f64 = row.get(2);

        // Filtering out self links, and adding them to adjacency matrix
        // and weight matrix
        if in_website_id != out_website_id {
            adjacency_matrix[in_website_id as usize].push(out_website_id as u32);
            weight_matrix[in_website_id as usize].push(weight);
        }

    }

    // we dont need mutable thingy anymore
    let adjacency_matrix = adjacency_matrix;
    let weight_matrix = weight_matrix;


    // initialisation value for all ranks
    let init_rank: f64 = 1.0 / total_websites as f64;

    // Initialising two vectors with default values(init_ran and 0.0)
    let mut rank: Vec<f64> = vec![init_rank; total_websites];
    let mut rank_new: Vec<f64> = vec![0.0; total_websites];
    let mut manhattan_distances: Vec<f64> = vec![0.0; num_iterations as usize];

    for iteration in 0..num_iterations as usize {
        {
            pagerank_iteration(&rank, &mut rank_new, &adjacency_matrix, &weight_matrix, &dampening_factor);
        }

        manhattan_distances[iteration] = get_manhattan_distance(&rank, &rank_new);

        // now we can just make rank to hold new rank without copying
        std::mem::swap(&mut rank, &mut rank_new);
        println!("After iteration {}:", iteration);
        println!("{:?}", rank);

    }

    println!("\nFinal rankings: {:?}", rank);
    println!("\nManhattan distances: {:?}", manhattan_distances);

    Ok(())
}
