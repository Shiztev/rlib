use std::collections::HashSet;

pub struct Node<T> {
  value: T,
}

pub struct Edge<T> {
  source: Node<T>,
  sink: Node<T>,
}

pub struct Graph<T> {
  edges: HashSet<Edge<T>>,
} 

impl<T> Graph<T> {
  pub fn new() -> Graph<T> {
      let g = Graph {
          edges: HashSet::new(),
      };
      g
  }
}
