use std::rc::Rc;

use lib::AppleTree;

fn main() {
    let tree: AppleTree<String> = AppleTree::default();

    let root = tree.root();
    println!("root parent = {:?}", root.get_parent());
    println!("root children(before) = {:?}", root.get_children());
    println!(
        "root strong = {}, weak = {}",
        Rc::strong_count(&root),
        Rc::weak_count(&root),
    );

    let branch = tree.node("branch".into());
    let leaf = tree.node("leaf".into());
    let fruit = tree.node("fruit".into());

    println!(
        "root strong = {}, weak = {}",
        Rc::strong_count(&root),
        Rc::weak_count(&root),
    );
    println!("root children = {:?}", root.get_children());

    tree.change_node_parent(Rc::clone(&leaf), Rc::clone(&branch));
    tree.change_node_parent(Rc::clone(&fruit), Rc::clone(&branch));

    println!(
        "root strong = {}, weak = {}",
        Rc::strong_count(&root),
        Rc::weak_count(&root),
    );
    println!("root children = {:?}", root.get_children());
}
