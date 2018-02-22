
use std::collections::HashSet;

use utils;

enum NodeType {
    State,
    Transition
}

struct Node {
    node_type: NodeType,
    name: String,
    index: usize,
    source_pos: utils::SourcePos,
    label: String,
    pre: &mut Vec<usize>,
    post: &mut Vec<usize>,
    block_index: usize,
    delta_before: u32,
    delta_after: u32
}

impl Node {

    fn add_successor(&self, succ: Node) {
        self.post.push(succ.index)
    }

}

struct Block
{
    block_ref: usize,
    node_refs: &mut Vec<usize>,
    pre_refs: &mut HashSet<usize>
}

struct Graph {
    source: String,
    nodes: &mut Vec<Node>,
    blocks: &mut Vec<Block>
}
