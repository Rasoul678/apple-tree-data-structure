use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Node<T>
where
    T: Display + Debug,
{
    pub value: T,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Drop for Node<T>
where
    T: Display + Debug,
{
    fn drop(&mut self) {
        println!("Dropping Node with value {}", self.value);
    }
}
