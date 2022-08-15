mod graph;

#[cfg(test)]
mod graph_tests {
  use crate::graph::Graph;

  #[test]
  fn new_graph() {
    let result: Graph<i32, i32> = Graph::new();
    assert_ne!(result.is_empty(), true);
  }
}

#[cfg(test)]
mod node_tests {
  use crate::graph::Node;

  #[test]
  fn new_node() {
    let result: Node<i32, i32> = Node::new(0, 0);
    let actual: i32 = 0;
    assert_eq!(result.id, actual);
    assert_eq!(result.value, actual);
  }
}
