[#derive Hash]
pub struct Node<T> {
  value: T,
}

[#derive Hash]
pub struct Edge<T> {
  source: Node<T>,
  sink: Node<T>,
}

pub struct Graph<T> {
  edges: HashSet<Edge<T>>,
} 

impl Graph {
  pub new() -> self {
      let g = Graph {
          edges: HashSet::new();
      }
      g
  }
}
