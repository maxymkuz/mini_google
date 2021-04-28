use std::fmt;

// fn pagerank_iteration(mut nodes: &Vec<Node>) {
//
// }

fn main() {
    // the total number of websites we will use in iteration
    let total_websites: usize = 4;

    // initialization value for all ranks
    let init_rank: f64 = 1.0 / total_websites as f64;

    let mut rank:Vec<f64> = vec![init_rank; total_websites];
    let mut rank_new:Vec<f64> = vec![0.0; total_websites];

    let mut adjacency_matrix:Vec<Vec<u32>> = vec![vec![]; total_websites];
    // artificially making up node connections until we have some real-life data


    adjacency_matrix[0] = vec![1, 2, 3];
    adjacency_matrix[1] = vec![0];
    adjacency_matrix[2] = vec![0];
    adjacency_matrix[3] = vec![0];

    // we dont need mutable thingy anymore
    let adjacency_matrix = adjacency_matrix;

    println!("{:?}", adjacency_matrix);

    let num_iterations:u32 = 1;
    for iteration in 0..num_iterations {
        for node_idx in 0..total_websites {
            let mut sum:f64 = 0.0;
            for in_node_idx in &adjacency_matrix[node_idx] {
                sum += rank[*in_node_idx as usize] / adjacency_matrix[*in_node_idx as usize].len() as f64;
            }
            rank_new[node_idx as usize] = sum;  // todo: add dampening factor, but later
        }
        // now we can just make rank to hold new rank without copying
        std::mem::swap(&mut rank, &mut rank_new);
    }
    println!("{:?}", rank);
    println!("{:?}", rank_new);
    // let first = init_node(0, &vec![1, 2, 3], &init_rank);
    // let second = init_node(1, &vec![0], &init_rank);
    // let third= init_node(2, &vec![0], &init_rank);
    // let fourth= init_node(3, &vec![0], &init_rank);
    // let mut nodes: Vec<Node> = vec![first, second, third, fourth];
    // for i in 0..total_websites {
    //    println!("{}", nodes[i as usize]);
    // }
    // for iteration in 0..1 {
    //     for i in 0..nodes.len() {
    //         let mut sum: f64 = 0.0;
    //         {
    //             let links = &nodes[i as usize].links;
    //             let in_link: usize = 0;
    //             for in_link in links {
    //                 let incoming_node: &Node = &nodes[*in_link as usize];
    //                 println!("{}", incoming_node.rank1);
    //                 sum += 0.5 * incoming_node.rank1 / incoming_node.out_nodes_num as f64;
    //                 // sum += 0.5 * nodes[incoming_node_idx as usize].rank1 / nodes[incoming_node_idx as usize].out_nodes_num as f64;
    //             }
    //         }
    //         let mut node = &mut nodes[i as usize];
    //         node.rank2 = sum + 0.5 * node.rank1;
        //     // nodes.get_mut(i as usize).unwrap().set_rank2(sum);
        // }
        // for i in 0..nodes.len() {
        //     nodes[i as usize].rank1 = nodes[i as usize].rank2; // бидлокод, потім заберу
        //     // nodes.get_mut(i as usize).unwrap().set_rank2(sum);
        // }


        // println!("\n");
        // for i in 0..total_websites {
        //     println!("{}", nodes[i as usize]);
        // }
    // // }
    // // println!("\n");
    // // for i in 0..total_websites {
    // //     println!("{}", nodes[i as usize]);
    // }
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

