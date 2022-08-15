use std::collections::{HashMap};

#[derive(Hash, Debug)]
pub struct Node<T, U> {
  pub id: T,
  pub value: U,
}

impl<T, U> Node<T, U> {
  pub fn new(id: T, value: U) -> Node<T, U> {
    let n: Node<T, U> = Node { id, value, };
    n
  }
}


#[derive(Debug)]
pub struct Graph<'a, T, U> {
  edges: HashMap<T, Vec<&'a Node<T, U>>>,
} 

impl<T, U> Graph<'_, T, U> {
  pub fn new() -> Graph<T, U> {
    let g: Graph<T, U> = Graph { edges: HashMap::new(), };
    g
  }

  pub fn is_empty(&self) -> bool {
    self.edges.is_empty()
  }

  pub fn insert(&self, node: &Node<T, U>) {
    if self.edges.contains_key(&node.id) {
      println!("Graph already contains node {}", node.id);
    } else {
      self.edges.insert(node.id, vec![node]);
    }
  }
}
