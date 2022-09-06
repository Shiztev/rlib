mod graph;
mod node;

#[cfg(test)]
mod graph_tests {
  use crate::graph::Graph;
  use crate::node::Node;

  #[test]
  fn new_graph() {
    let result: Graph<i32, i32> = Graph::new();
    assert_eq!(result.is_empty(), true);
  }

  #[test]
  fn insert_node() {
    let mut g: Graph<i32, i32> = Graph::new();
    let n1: Node<i32, i32> = Node::new(0, 0);
    let n2: Node<i32, i32> = Node::new(0, 0);
    assert!(g.insert(n1));
    assert!(!g.insert(n2));
  }

  #[test]
  fn connect_nodes() {
    let mut g: Graph<i32, i32> = Graph::new();
    let n1: Node<i32, i32> = Node::new(0, 0);
    let n2: Node<i32, i32> = Node::new(1, 0);
    assert!(g.insert(n1));
    assert!(g.insert(n2));
    assert!(g.connect(&0, &1));
    assert!(!g.connect(&2, &0));
    assert!(!g.connect(&0, &2))
  }
}

#[cfg(test)]
mod node_tests {
  use crate::node::Node;

  #[test]
  fn new_node() {
    let result: Node<i32, i32> = Node::new(0, 0);
    let actual: i32 = 0;
    assert_eq!(result.id, actual);
    assert_eq!(result.value, actual);
  }

  #[test]
  fn connect_nodes() {
    let mut n1: Node<i32, i32> = Node::new(0, 0);
    let n2: Node<i32, i32> = Node::new(1, 0);
    assert!(n1.connect(&n2.id));
    assert!(!n1.connect(&n2.id));
  }
}
