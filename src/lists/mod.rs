use std::rc::Rc;

mod node;

use node::{Node, NodeOption, Iter};

#[derive(Debug)]
pub struct List<T> {
  pub head: NodeOption<T>,
}

impl<T> List<T> {
  pub fn new_empty() -> Self {
    List { head: None }
  }

  pub fn append_start(&mut self, data: T) {
    let new_head = Node::new(data);
    match self.head.take() {
      Some(old_head) => {
        new_head.borrow_mut().next = Some(Rc::clone(&old_head));
      }
      _ => (),
    }
    self.head = Some(new_head);
  }

  pub fn iter(&self) -> Iter<T> {
    let current = match self.head {
      Some(ref head) => Some(Rc::clone(head)),
      None => None,
    };
    Iter { current }
  }
}
