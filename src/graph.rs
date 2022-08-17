use std::{collections::HashMap, hash::Hash, fmt::{Display, Debug}};

/// A Node which encapsulates a piece of data and can have 
/// directed connections to other nodes.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Node<T, U> where T: PartialEq + Display + Copy {
  pub id: T,
  pub value: U,
  edges: Vec<T>,
}

impl<T, U> Node<T, U> where T: PartialEq + Display + Copy + Clone {
  pub fn new(id: T, value: U) -> Node<T, U> {
    let n: Node<T, U> = Node { id, value, edges: Vec::new()};
    n
  }

  pub fn connect(&mut self, id: T) -> bool {
    if self.edges.contains(&id) {
      println!("Node {} already connected to node {}", self.id, id);
      false
    } else {
      self.edges.push(id);
      true
    }
  }
}


/// A collection of nodes which can have directed connections.
#[derive(Debug)]
pub struct Graph<T, U> where T: Eq + Hash + Debug + Display + Copy, U: Debug {
  nodes: HashMap<T, Node<T, U>>,
} 

impl<T, U> Graph<T, U> where T: Eq + Hash + Debug + Display + Copy, U: Debug{
  pub fn new() -> Graph<T, U> {
    let g: Graph<T, U> = Graph { nodes: HashMap::new(), };
    g
  }

  pub fn is_empty(&self) -> bool {
    self.nodes.is_empty()
  }

  pub fn insert(&mut self, node: Node<T, U>) -> bool {
    let r: bool;
    let id: T = node.id;

    if self.nodes.contains_key(&id) {
      println!("Graph already contains node {:?}", id);
      r = false
    } else {
      match self.nodes.insert(id, node) {
        Some(v) => panic!(
          "Got {:?} when inserting non-existing key {:?}", v, id),  // TODO: node needs string for error
        None => r = true,
      } 
    }
    r
  }

  pub fn connect(&mut self, source: T, sink: T) -> bool {
    if !self.nodes.contains_key(&sink) {
      println!("Sink {} does not exist in this graph.", sink);
      false
    } else {
      if self.nodes.contains_key(&source) {
        let s: &mut Node<T, U> = self.nodes.get_mut(&source).expect("Node {source} existence confirmed, but does not exist in graph. Should not be possible.");
        s.connect(sink);
        true
      } else {
        println!("Source {} does not exist in this graph.", source);
        false
      }
    }
  }
}
