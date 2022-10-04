use std::fmt::{Display, Debug};

/// A Heap implementation with a vector.
pub struct Heap<T> {
  nodes: Vec<T>,
}

impl<T> Heap<T> where T: Eq + PartialEq+ PartialOrd + Debug + Display {

}