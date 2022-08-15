use std::collections::HashSet;

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


#[derive(Hash, Debug)]
pub struct Edge<'a, 'b, T, U> {
  pub source: &'a Node<T, U>,
  pub sink: &'b Node<T, U>,
}

impl<T, U> Edge<'_, '_, T, U> {
  pub fn new(source: &Node<T, U>, sink: &Node<T, U>) -> Node<T, U> {
    let e: Edge<T, U> = Edge { source, sink, };
    e
  }
}


#[derive(Debug)]
pub struct Graph<T, U> {
  edges: HashSet<Edge<T, U>>,
} 

impl<T, U> Graph<T, U> {
  pub fn new() -> Graph<T, U> {
    let g: Graph<T, U> = Graph { edges: HashSet::new(), };
    g
  }

  pub fn is_empty(&self) -> bool {
    self.edges.is_empty()
  }
}
