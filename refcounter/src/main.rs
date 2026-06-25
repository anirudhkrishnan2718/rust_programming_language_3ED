use crate::List::{Cons, Nil};
use crate::NewList::{Cons2, Nil2};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // clone the reference to a and pass it to the ref counting pointer
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b: {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c: {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope: {}", Rc::strong_count(&a));

    // use Rc::clone to give multiple ownership of value to a1, b1 and c1
    let value = Rc::new(RefCell::new(5));
    let a1 = Rc::new(Cons2(Rc::clone(&value), Rc::new(Nil2)));
    let b1 = Cons2(Rc::new(RefCell::new(3)), Rc::clone(&a1));
    let c1 = Cons2(Rc::new(RefCell::new(4)), Rc::clone(&a1));

    // a1, b1, c1 remain immutable but value is mutated using refcell
    *value.borrow_mut() += 10;

    println!("a1 after = {a1:?}");
    println!("b1 after = {b1:?}");
    println!("c1 after = {c1:?}");
}

// a reference counting pointer enables shared ownership of a between b and c
// it waits for the number of references to become zero before dropping the variable
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// multiple references through Rc and interior mutability through RefCell
#[derive(Debug)]
enum NewList {
    Cons2(Rc<RefCell<i32>>, Rc<NewList>),
    Nil2,
}
