use std::collections::HashSet;

pub struct Node<T, U> {
  id: T,
  value: U,
}

impl<T, U> Node<T, U> {
  pub fn new(id: T, value: U) -> Node<T, U> {
    let n: Node<T, U> = Node { id, value, };
    n
  }
}

pub struct Edge<T, U> {
  source: Node<T, U>,
  sink: Node<T, U>,
}

pub struct Graph<T, U> {
  edges: HashSet<Edge<T, U>>,
} 

impl<T, U> Graph<T, U> {
  pub fn new() -> Graph<T, U> {
    let g: Graph<T, U> = Graph { edges: HashSet::new(), };
    g
  }
}
