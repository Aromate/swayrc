use std::io;

use crate::ipc::creat_node;

use super::ipc;
use super::struch::Node;

fn find_by_name(node: &Node, name: &str) -> bool {
    if let Some(n) = node.name.clone() {
        n == name
    } else {
        false
    }
}

pub fn special_exec(node_name: &str, args: &[&str]) -> io::Result<()> {
    creat_node(node_name)?;
    let node = Node::new()?;
    let node = node
        .find_node(|node| find_by_name(node, node_name))
        .unwrap();
    if node.nodes.is_empty() {
        ipc::exec_binary(args)?;
    }
    Ok(())
}
