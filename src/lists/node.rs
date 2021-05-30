use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;
pub type NodeOption<T> = Option<NodeRef<T>>;

#[derive(Debug)]
pub struct Node<T> {
  pub data: T,
  pub next: NodeOption<T>,
}

impl<T> Node<T> {
  pub fn new(data: T) -> NodeRef<T> {
    return Rc::new(RefCell::new(Node { data, next: None }));
  }
}

pub struct Iter<T> {
  pub current: NodeOption<T>,
}

impl<T> Iterator for Iter<T> {
  type Item = NodeRef<T>;

  fn next(&mut self) -> NodeOption<T> {
    let mut result = None;
    self.current = match &self.current {
      Some(ref current) => {
        result = Some(Rc::clone(current));
        match &current.borrow().next {
          Some(next_node) => Some(Rc::clone(next_node)),
          _ => None,
        }
      }
      _ => None,
    };
    result
  }
}