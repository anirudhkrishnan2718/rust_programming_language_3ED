fn main() {
    let v1 = vec![1, 2, 3, 4, 100];
    let v2 = vec!['a', 'g', 'b', 'c', 'd'];

    println!("The largest element of v1 is {}", largest(&v1));
    println!("The largest element of v2 is {}", largest(&v2));

    // the struct's definition forces both x and y to be the same type T
    // this throws an error
    // let point = Point { x: 1, y: 4.0 };
    // This struct is defined using two generic types for x and y so it works
    let point = PointAny { x: 1, y: 4.0 };
}

// this function is generic over some type T
// complains that the type T has to have the PartialOrdered trait
// fn largest<T>(list: &[T]) -> &T {

// restrict T to only those types which have ordering to enable comparison using >
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// struct can now hold any number type as x and y coordinates
struct Point<T> {
    x: T,
    y: T,
}

// methods can have generic types in their definitions the same as functions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct PointAny<T, U> {
    x: T,
    y: U,
}

// method works only if the generic T is substituted by the concrete f32 type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
