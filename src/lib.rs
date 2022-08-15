mod graph;

#[cfg(test)]
mod graph_tests {
  use crate::graph::Graph;

  #[test]
  fn new_graph() {
      let result: Graph<i32, i32> = Graph::new();
      assert_ne!(result, None);
  }
}
