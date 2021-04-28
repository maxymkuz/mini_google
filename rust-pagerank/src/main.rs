use std::fmt;

struct Node {
    index: u32,
    rank1: f64,
    rank2: f64,
    out_nodes_num: u16,
    links: Vec<u32> // adjacency matrix would be a suicide here
}


impl Node {
    fn rank2_mut(&mut self) -> &mut f64 {
        &mut self.rank1
    }
    fn set_rank2(&mut self, new_rank: f64) {
        self.rank2 = new_rank;
    }
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index: {}\nRank1: {}; Rank2: {};\n{} out links: {:?}", self.index, self.rank1, self.rank2, self.out_nodes_num, self.links)
    }
}

fn init_node(index: u32, links: Vec<u32>, init_rank: &f64) -> Node {
    // calculating the weight, each outgoing link should have:
    // let init_val: f64 = 1.0 / links.len() as f64;
    // eliminating self references:
    // println!("{}", links[0].what_is_this);
    // links = links.iter().filter(|&val| val != index).collect();

    Node{
        index, rank1: *init_rank, rank2: 0.0, out_nodes_num: links.len() as u16, links
    }
}


// fn pagerank_iteration(mut nodes: &Vec<Node>) {
//
// }


fn main() {
    // the total number of websites we will use in iteration
    let total_websites: u32 = 4;

    let init_rank: f64 = 1.0 / total_websites as f64;

    let mut first = init_node(0, vec![1, 2, 3], &init_rank);
    let mut second = init_node(1, vec![0], &init_rank);
    let mut third= init_node(2, vec![0], &init_rank);
    let mut fourth= init_node(3, vec![0], &init_rank);
    let mut nodes: Vec<Node> = vec![first, second, third, fourth];
    for i in 0..total_websites {
       println!("{}", nodes[i as usize]);
    }
    for iteration in 0..10 {
        for i in 0..nodes.len() {
            let mut sum: f64 = 0.0;
            for j in 0..nodes[i as usize].links.len() {
                let incoming_node_idx = nodes[i as usize].links[j as usize];
                sum += 0.5 * nodes[incoming_node_idx as usize].rank1 / nodes[incoming_node_idx as usize].out_nodes_num as f64;
            }
            nodes[i as usize].rank2 = sum + 0.5 * nodes[i as usize].rank1;
            // nodes.get_mut(i as usize).unwrap().set_rank2(sum);
        }
        for i in 0..nodes.len() {
            nodes[i as usize].rank1 = nodes[i as usize].rank2;
            // nodes.get_mut(i as usize).unwrap().set_rank2(sum);
        }
        println!("\n");
        for i in 0..total_websites {
            println!("{}", nodes[i as usize]);
        }
    }
    println!("\n");
    for i in 0..total_websites {
        println!("{}", nodes[i as usize]);
    }
}
