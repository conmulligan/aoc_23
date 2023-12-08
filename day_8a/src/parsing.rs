use std::collections::HashMap;

use crate::model::Node;

pub fn parse_nodes(lines: Vec<&str>) -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in lines {
        let node = parse_node(line);
        nodes.insert(node.id.clone(), node);
    }

    nodes
}

pub fn parse_node(line: &str) -> Node {
    let components: Vec<&str> = line.split('=').collect();
    let id = components.first().unwrap().trim().to_string();
    let ids: Vec<String> = components
        .last()
        .unwrap()
        .trim()
        .replace("(", "")
        .replace(")", "")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    Node {
        id,
        left_id: ids.first().unwrap().to_owned(),
        right_id: ids.last().unwrap().to_owned(),
    }
}
