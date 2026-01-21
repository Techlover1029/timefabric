use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    TimecodeGenerator,
    TimecodeOutput,
    NetworkProtocol,
    Trigger,
}

#[derive(Debug, Clone)]
pub struct NodePort {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub node_type: NodeType,
    pub name: String,

    // Canvas position
    pub x: f32,
    pub y: f32,

    // Ports
    pub inputs: usize,
    pub outputs: usize,

    // Arbitrary per-node settings (future-proofing)
    pub properties: HashMap<String, String>,
}
