use std::{hash::Hash, fmt::{Display, Debug}};

/// A Node which encapsulates a piece of data and can have 
/// directed connections to other nodes.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Node<T, U> {
  pub id: T,
  pub value: U,
  edges: Vec<T>,
}

impl<T, U> Node<T, U> where T: PartialEq + Display + Clone {
  pub fn new(id: T, value: U) -> Node<T, U> {
    let n: Node<T, U> = Node { id, value, edges: Vec::new()};
    n
  }

  pub fn connect(&mut self, id: &T) -> bool {
    if self.edges.contains(id) {
      println!("Node {} already connected to node {}", self.id, id);
      false
    } else {
      self.edges.push(id.clone());
      true
    }
  }
}

