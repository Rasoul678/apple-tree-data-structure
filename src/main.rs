use std::rc::Rc;

use lib::AppleTree;

fn main() {
    let tree: AppleTree<String> = AppleTree::default();

    let leaf = tree.node("leaf".into());
    let leaf2 = tree.node("leaf2".into());

    tree.change_node_parent(Rc::clone(&leaf2), Rc::clone(&leaf));
    println!("tree {:?}\n\n", tree);
    println!("leaf {:?}\n\n", leaf);
    println!("leaf2 {:?}\n\n", leaf2);

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // println!("{} Scope-start", "=".repeat(20));

    // {
    //     let branch = Rc::new(Node::new("Branch".into()));

    //     branch.children.borrow_mut().push(Rc::clone(&leaf));

    //     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    //     println!(
    //         "branch strong = {}, weak = {}",
    //         Rc::strong_count(&branch),
    //         Rc::weak_count(&branch),
    //     );

    //     println!(
    //         "leaf strong = {}, weak = {}",
    //         Rc::strong_count(&leaf),
    //         Rc::weak_count(&leaf),
    //     );
    // }

    // println!("{} Scope-end", "=".repeat(20));
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // println!(
    //     "leaf strong = {}, weak = {}",
    //     Rc::strong_count(&leaf),
    //     Rc::weak_count(&leaf)
    // );
}
