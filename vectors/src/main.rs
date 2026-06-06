#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // new empty vector. All elements have to be of the same data type, which has
    // to be annotated if a new empty vector is being declared.
    // Can change size (is stored on heap)
    let mut v: Vec<i32> = Vec::new();
    println!("v = {v:?}");
    v.push(1);
    v.push(2);
    v.push(30);
    // refer to elements using square brackets
    println!("v = {v:?}, and its third element is {:?}", &v[2]);

    // using get notation returns None and continues the program
    println!("fourth element of v is {:?}", v.get(3));
    // referencing a nonexistent element crashes the program
    // println!("fourth element of v is {:?}", &v[3]);

    // cannot append to vector when a different element is borrowed
    let first = &v[0];
    // v.push(6);
    println!("The first element is {first}");

    // change each value of mutable vector in place using a for loop
    let mut u = vec![2, 3, 4, 5];
    for i in &mut u {
        // the leading asterisk is dereferecing (converting pointer to value)
        *i = *i * *i;
    }
    println!("The vector u is now {u:?}");

    // vector can hold heterogenous data types by encapsulating them inside an enum
    let row = vec![
        Cell::Int(3),
        Cell::Float(10.12),
        Cell::Text(String::from("Hello")),
    ];

    println!("row = {row:?}");
    // dropping a vector drops all its elements as well
}
