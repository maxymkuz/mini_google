use std::fmt;

struct Node {
    index: u32,
    rank1: f64,
    rank2: f64,
    out_nodes: u16,
    links: Vec<u32> // adjacency matrix would be a suicide here
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index: {}\nRank1: {}; Rank2: {};\n{} out links: {:?}", self.index, self.rank1, self.rank2, self.out_nodes, self.links)
    }
}

fn init_node(index: u32, mut links: Vec<u32>) -> Node {
    // calculating the weight, each outgoing link should have:
    let init_val: f64 = 1.0 / links.len() as f64;
    // eliminating self references:
    println!("{}", links[0].what_is_this);
    // links = links.iter().filter(|&val| val != index).collect();

    Node{
        index, rank1: init_val, rank2: init_val, out_nodes: links.len() as u16, links
    }
}

fn main() {
    let mut first = init_node(0, vec![1, 2, 3]);
    first.rank1 = 1.0;
    println!("{}", first);
}
