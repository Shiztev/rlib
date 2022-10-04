use std::fmt::{Display, Debug};

/// A Heap implementation with a vector.
pub struct Heap<T> {
  nodes: Vec<T>,
}

impl<T> Heap<T> where T: Eq + PartialEq+ PartialOrd + Debug + Display {
  /// Create a new empty heap.
  pub fn new() -> Heap<T> {
    let h: Heap<T> = Heap { nodes: Vec::new(), };
    h
  }

  /// Insert a unique element into the heap.
  pub fn insert(e: T) -> bool {
    false  // TODO: Implement
  }
}