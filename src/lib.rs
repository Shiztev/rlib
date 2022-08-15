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

#[cfg(test)]
mod edge_tests {
  use crate::graph::{Edge, Node};

  #[test]
  fn new_edge() {
    let n1: Node<i32, i32> = Node::new(0, 0);
    let n2: Node<i32, i32> = Node::new(0, 0);
    let result: Edge<i32, i32> = Edge::new(&n1, &n2);
    assert_eq!(result.source, n1);
    assert_eq!(result.sink, n2);
  }
}
