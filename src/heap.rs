use std::{fmt::{Display, Debug}, ptr::null};

/// A Heap implementation with a vector.
pub struct Heap<T> {
  nodes: Vec<Option<T>>,
}

impl<T> Heap<T> where T: Eq + PartialEq+ PartialOrd + Debug + Display {
  /// Create a new empty heap.
  pub fn new() -> Heap<T> {
    let h: Heap<T> = Heap { nodes: Vec::new(), };
    h
  }

  /// Insert a unique element into the heap.
  pub fn insert(&mut self, e: T) -> bool {
    let t: Option<T> = Option::Some(e);
    self.nodes.push(t);
    false  // TODO: Implement
  }

  /// Check the top value of the heap.
  pub fn peak(&self) -> Option<&T> {
    self.nodes[0].as_ref()
  }

  // Remove the top most element from the heap.
  pub fn pop(&mut self) -> Option<T> {
    match self.nodes.pop() {
      Some(e) => return e,
      None => None,
    }
  }
}