pub struct Tree<T> {
  nodes: Vec<Option<T>>,
}

impl<T> Tree<T> {
  /// Create a new, empty Tree.
  pub fn new() -> Tree<T> {
    let mut t: Tree<T> = Tree { nodes: Vec::new(), };
    t.nodes.push(None);
    t
  }

  /// Check the top value of the Tree.
  pub fn peak(&self) -> Option<&T> {
    self.nodes[0].as_ref()
  }
}