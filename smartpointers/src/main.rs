use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    // cons list from Lisp
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let y1 = Box::new(x);
    let y2 = MyBox::new(x);

    assert_eq!(5, x);
    // have to use the dereference operator on y since its the value that is equal to 5
    // assert_eq!(5, y);
    assert_eq!(5, *y);
    assert_eq!(5, *y1);
    // will not work since compiler doesn't know how to dereference a MyBox
    assert_eq!(5, *y2);

    let m = MyBox::new(String::from("Rust"));
    // deref coercion converts the reference to a MyBox<String> into &String
    // Rust internally then derefs the &String into &str (string slice)
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    // calling the Drop trait's associated drop function won't work
    // c.drop();
    // calling the drop function works
    drop(c);
    println!("CustomSmartPointer force dropped");

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created")
    // at this point both c and d are dropped
}

// recursive data type, whose size is not fixed at compile time.
enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

// tuple struct
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // access the first value in the tuple struct of MyBox
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!")
}

struct CustomSmartPointer {
    data: String,
}

// custom action when variable is dropped
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
