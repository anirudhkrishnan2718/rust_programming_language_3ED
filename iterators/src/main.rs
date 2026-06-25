fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // iterators only require next() to be implemented. This returns an Option
    // with the next value, and None upon reaching the end
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());

    // The sum of v1_iter is zero because there are no terms to consume
    // need a fresh iterator for consumers like sum()
    let total: i32 = v1_iter.sum();
    let total2: i32 = v1.iter().sum();
    println!("The sum using v1_iter is: {total}");
    println!("but the sum using a fresh iterator is: {total2}");

    // iterator adapters take an iterator and closure as inputs to produce another
    let v2 = vec![1, 2, 3, 4, 5];
    // warns about iterators being lazy
    let v2_iter = v2.iter().map(|x| x * x);
    println!(
        "The new iterator after being consumed by a vector: {:?}",
        Vec::from_iter(v2_iter)
    );
    // need to specify type annotation to use collect()
    let v3: Vec<_> = v2.iter().map(|x| x * x * x).collect();
    println!("Using collect gives: {:?}", v3);
}
