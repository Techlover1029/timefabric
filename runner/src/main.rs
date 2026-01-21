use engine::graph::Graph;
use engine::node::{Node, NodeType};
use std::collections::HashMap;

fn main() {
    let mut graph = Graph::new();

    let node = Node {
        id: engine::node::NodeId(0),
        name: "Timecode Generator".to_string(),
        node_type: NodeType::TimecodeGenerator,

        // These are COUNTS, not ports
        inputs: 0,
        outputs: 1,

        // Position is split into x / y
        x: 100.0,
        y: 100.0,

        // Empty property bag
        properties: HashMap::new(),
    };

    graph.add_node(node);

    graph.save_to_file("test_project.tf.json").unwrap();

    let loaded = Graph::load_from_file("test_project.tf.json").unwrap();
    println!("Loaded {} nodes", loaded.nodes.len());
}
