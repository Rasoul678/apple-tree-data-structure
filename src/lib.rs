use std::{
    cell::{Ref, RefCell, RefMut},
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

    pub fn get_parent(&self) -> Option<Rc<Node<T>>> {
        self.parent.borrow().upgrade()
    }

    pub fn get_parent_mut(&self) -> Option<Rc<Node<T>>> {
        self.parent.borrow_mut().upgrade()
    }

    pub fn get_children(&self) -> Ref<Vec<Rc<Node<T>>>> {
        self.children.borrow()
    }

    pub fn get_children_mut(&self) -> RefMut<Vec<Rc<Node<T>>>> {
        self.children.borrow_mut()
    }
}

impl<T> Drop for Node<T>
where
    T: Display + Debug + PartialEq,
{
    fn drop(&mut self) {
        println!("Dropping Node with value: {:#?}", self.value.as_ref());
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
    root: Rc<Node<T>>,
}

impl<T> AppleTree<T>
where
    T: Display + Debug + PartialEq,
{
    pub fn root(&self) -> Rc<Node<T>> {
        Rc::clone(&self.root)
    }

    pub fn node(&self, val: T) -> Rc<Node<T>> {
        let node = Rc::new(Node::new(val));
        *node.parent.borrow_mut() = Rc::downgrade(&self.root);
        self.root.children.borrow_mut().push(Rc::clone(&node));
        node
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

        // Update node's parent
        *parent = Rc::downgrade(&new_parent);

        // Add node to new parent
        new_parent.children.borrow_mut().push(Rc::clone(&node));
    }
}
