use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;

struct Node {
    v: i32,
    p: Option<Weak<RefCell<Node>>>,
}

fn main() {

    let mut arr: Vec<Rc<RefCell<Node>>> = Vec::new();
    arr.push(Rc::new(RefCell::new(Node {v: 0, p: None})));

    for i in (1..5) {
        arr.push(Rc::new(RefCell::new(Node {v: i, p: Some(Rc::downgrade(&arr[(i - 1) as usize]))})));
    }

    for i in arr.iter() {
        println!("addr:{:p}, strong_count:{}, weak_count:{}, v:{}", &(*i.borrow()), Rc::strong_count(&i), Rc::weak_count(&i), i.borrow().v);
    }

    loop_rc(arr.last().unwrap().clone());
    loop_raw(&arr.last().unwrap().borrow());
}

fn loop_rc(node: Rc<RefCell<Node>>) {
    println!("addr:{:p}, strong_count:{}, weak_count:{}, v:{}", &(*node.borrow()), Rc::strong_count(&node), Rc::weak_count(&node), node.borrow().v);

    if let Some(p) = &node.borrow().p {
        let rc = p.upgrade().unwrap();

        {
            let mut m = rc.borrow_mut();
            m.v *= 10;
        }

        loop_rc(rc.clone());
    }
}

fn loop_raw(node: &Node) {
    println!("addr:{:p}, v:{}", &(*node), node.v);
    if let Some(p) = &node.p {
        let rc = p.upgrade().unwrap();

        {
            let mut m = rc.borrow_mut();
            m.v *= 10;
        }

        loop_raw(&rc.borrow());
    }
}
