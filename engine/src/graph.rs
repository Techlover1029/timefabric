use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::node::{Node, NodeId, NodeType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from_node: NodeId,
    pub from_output: usize,
    pub to_node: NodeId,
    pub to_input: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<u64, Node>,
    pub next_node_id: u64,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_node_id: 1,
        }
    }
}

impl Graph {
    pub fn create_node(
        &mut self,
        name: &str,
        node_type: NodeType,
        x: f32,
        y: f32,
    ) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;

        let node = Node {
            id,
            name: name.to_string(),
            node_type,
            x,
            y,
            inputs: 1,
            outputs: 1,
            properties: Default::default(),
        };

        self.nodes.insert(id.0, node);
        id
    }
}