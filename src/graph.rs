use std::{collections::HashMap, hash::Hash, fmt::{Display, Debug}};
use crate::node::Node;


/// A collection of nodes which can have directed connections.
#[derive(Debug)]
pub struct Graph<T, U> {
  nodes: HashMap<T, Node<T, U>>,
}

impl<T, U> Graph<T, U> where T: Eq + Hash + Debug + Display + Clone, U: Debug{
  pub fn new() -> Graph<T, U> {
    let g: Graph<T, U> = Graph { nodes: HashMap::new(), };
    g
  }

  pub fn is_empty(&self) -> bool {
    self.nodes.is_empty()
  }

  pub fn insert(&mut self, node: Node<T, U>) -> bool {
    let r: bool;
    let id: T = node.id.clone();

    if self.nodes.contains_key(&id) {
      println!("Graph already contains node {:?}", id);
      false

    } else {
      match self.nodes.insert(node.id.clone(), node) {
        Some(v) => {println!(
          "Got {:?} when inserting non-existing key {:?}", v, id);
          return false;
        },
        None => return true,
      } 
    }
  }

  pub fn connect(&mut self, source: &T, sink: &T) -> bool {
    if !self.nodes.contains_key(sink) {
      println!("Sink {} does not exist in this graph.", sink);
      false

    } else {
      if self.nodes.contains_key(source) {
        let s: &mut Node<T, U> = self.nodes.get_mut(source).expect("Node {source} existence confirmed, but does not exist in graph. Should not be possible.");
        s.connect(sink);
        true

      } else {
        println!("Source {} does not exist in this graph.", source);
        false
      }
    }
  }
}
