use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::{Rc, Weak},
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Node<T>
where
    T: Display + Debug + PartialEq,
{
    uuid: String,
    value: Option<T>,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Display + Debug + PartialEq,
{
    fn new(value: T) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            value: Some(value),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }
}

impl<T> Default for Node<T>
where
    T: Display + Debug + PartialEq,
{
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            value: None,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }
}

#[derive(Debug, Default)]
pub struct AppleTree<T>
where
    T: Display + Debug + PartialEq,
{
    pub root: Rc<Node<T>>,
}

impl<T> AppleTree<T>
where
    T: Display + Debug + PartialEq,
{
    pub fn node(&self, val: T) -> Rc<Node<T>> {
        let leaf = Rc::new(Node::new(val));
        *leaf.parent.borrow_mut() = Rc::downgrade(&self.root);
        self.root.children.borrow_mut().push(Rc::clone(&leaf));
        leaf
    }

    pub fn change_node_parent(&self, node: Rc<Node<T>>, new_parent: Rc<Node<T>>) {
        let mut parent = node.parent.borrow_mut();
        // Remove node from old parent
        parent
            .upgrade()
            .unwrap()
            .children
            .borrow_mut()
            .retain(|x| x.uuid != node.uuid);

        // Update nodes parent
        *parent = Rc::downgrade(&new_parent);

        // Add node to new parent
        new_parent.children.borrow_mut().push(Rc::clone(&node));
    }
}
