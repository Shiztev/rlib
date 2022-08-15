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
pub struct Graph<T, U> {
  edges: HashMap<T, Node<T, U>>,
} 

impl<T, U> Graph<T, U> {
  pub fn new() -> Graph<T, U> {
    let g: Graph<T, U> = Graph { edges: HashMap::new(), };
    g
  }

  pub fn is_empty(&self) -> bool {
    self.edges.is_empty()
  }
}
