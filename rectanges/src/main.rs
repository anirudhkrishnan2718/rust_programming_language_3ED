// struct has to be defined oustide main to be visible to other functions
#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

// implement area as a method of the Rectange struct
impl Rectange {
    fn area(&self) -> u32 {
        // self refers to the instance of the struct
        self.width * self.height
    }
}

fn main() {
    // using a tuple, with no explicit labeling of height and width
    let rect1 = (30, 50);

    let rect2 = Rectange {
        width: 50,
        height: 100,
    };

    println!("The area of the rectange is {} square pixels", area(rect1));
    println!(
        "The area of the new rectange is {} square pixels",
        area_struct(&rect2)
    );
    println!(
        "The area of the new rectange using its method is {} square pixels",
        rect2.area()
    );

    // error because the struct does not implement debug or display yet
    println!("rect1 is {rect2:#?}");

    let scale = 2;
    let rect3 = Rectange {
        // dbg! does not take ownership, so the expression is still assigned to width
        width: dbg!(30 * scale),
        height: 50,
    };

    // debug macro prints location as well as resulting expression
    dbg!(&rect3);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectange) -> u32 {
    rectangle.width * rectangle.height
}
