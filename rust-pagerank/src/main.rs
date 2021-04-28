fn pagerank_iteration<'a>(
    rank: &'a Vec<f64>,
    rank_new: &'a mut Vec<f64>,
    adjacency_matrix: &'a Vec<Vec<u32>>,
    out_nodes_num: &'a Vec<u32>,
    d: f64
) {

    for node_idx in 0..rank.len() {
        let mut sum: f64 = 0 as f64;
        for in_node_idx in &adjacency_matrix[node_idx] {
            // if node_idx == 3 {
            //     println!("__ {}", rank[*in_node_idx as usize] / out_nodes_num[*in_node_idx as usize] as f64);
            // }
            sum += rank[*in_node_idx as usize] / out_nodes_num[*in_node_idx as usize] as f64;
        }
        rank_new[node_idx as usize] = (1.0 - d) + d * sum;  // todo: add dampening factor, but later
    }
    // println!("INSIDE PAGERANK: {:?}", rank);
    // println!("INSIDE PAGERANK: {:?}", rank_new);
    // let rank_new = rank_new;
    // rank_new
}

fn main() {
    // the total number of websites we will use in iteration
    let total_websites: usize = 5;
    let dampening_factor:f64 = 0.85;

    // initialization value for all ranks
    let init_rank: f64 = 1.0 / total_websites as f64;

    let mut rank: Vec<f64> = vec![init_rank; total_websites];
    let mut rank_new: Vec<f64> = vec![0.0; total_websites];

    let mut adjacency_matrix: Vec<Vec<u32>> = vec![vec![]; total_websites];
    // artificially making up node connections until we have some real-life data


    adjacency_matrix[0] = vec![3];
    adjacency_matrix[1] = vec![0];
    adjacency_matrix[2] = vec![1];
    adjacency_matrix[3] = vec![0, 1, 4];
    adjacency_matrix[4] = vec![];

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

    let num_iterations:u32 = 10;

    println!("{:?}", rank);
    println!("{:?}", rank_new);
    for _iteration in 0..num_iterations {
        {
            pagerank_iteration(&rank, &mut rank_new, &adjacency_matrix, &out_nodes_num, dampening_factor);
        }
        // println!("{:?}", rank);
        // println!("{:?}", rank_new);
        // now we can just make rank to hold new rank without copying
        std::mem::swap(&mut rank, &mut rank_new);
        println!("After iteration {}", _iteration);
        println!("{:?}", rank);
        println!("{:?}", rank_new);
    }

    println!("\nFinal rankings: {:?}", rank);
}

// struct Node {
//     index: u32,
//     rank1: f64,
//     rank2: f64,
//     out_nodes_num: u16,
//     links: Vec<u32> // adjacency matrix would be a suicide here
// }
//
//
// impl Node {
//     fn rank2_mut(&mut self) -> &mut f64 {
//         &mut self.rank1
//     }
//     fn set_rank2(&mut self, new_rank: f64) {
//         self.rank2 = new_rank;
//     }
// }


// impl fmt::Display for Node {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Index: {}\nRank1: {}; Rank2: {};\n{} out links: {:?}", self.index, self.rank1, self.rank2, self.out_nodes_num, self.links)
//     }
// }
//
// fn init_node(index: u32, links: &Vec<u32>, init_rank: &f64) -> Node {
//     // calculating the weight, each outgoing link should have:
//     // let init_val: f64 = 1.0 / links.len() as f64;
//
//     // eliminating self references:
//     // links = links.iter().filter(|&val| val != index).collect();
//
//     Node{
//         index, rank1: *init_rank, rank2: 0.0, out_nodes_num: links.len() as u16, links: links.to_vec()
//     }
// }

