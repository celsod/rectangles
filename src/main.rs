fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // let rect1 = (30, 50); //tuple

    // println!("The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // let scale = 2;
    
    let rect1 = Rectangle{
        // width: dbg!(30 * scale), //dbg! is the debug macro output
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    println!(
        "The values of rect1 are {:?}", rect1 //:? is output format debug
    ); //prints out struct in a way that is useful for devs to see the values while debugging code

    println!(
        "The values of rect1 are {:#?}", rect1 //:#? is output format debug but with style!
    ); //prints out struct in a way that is useful for devs to see the values while debugging code

    println!("The area of the rectangle is {} square pixels.",
        rect1.area() //called area method on rectangle instance
    );
    
    if rect1.width() { //rect1.width() is the method within the implementation block
        println!("The rectangle has a nonzero width; it is {}", rect1.width) //rect1.width is the value within the structure
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); 
    //rect1 is the self rectangle and rect2 is the other in the method below
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    //same except rect3 is the other in the method below

    let sq = Rectangle::square(3); //calling the associated function that is built into the Rectangle implementation
    println!("The square of the rectangle is: {}", sq);

    // dbg!(&rect1); //prints out the structure in the format of the structure
}

//fn area(width: u32, height: u32) -> u32 { //old way of doing the function
//    width*height
//}

// fn area (dimensions: (u32, u32)) -> u32 { //slightly better due to structure but not clear due to lack of names
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)] //must explicitly opt in to make debug functionality available
struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(rectangle: &Rectangle) -> u32 { //immutable borrow of structure instance and field values
//     rectangle.width * rectangle.height //clarity is achieved with this
// }

impl Rectangle { //implementation block for Rectangle
    fn area(&self) -> u32 { //self is only argument for area, no input from other sources
        //&self is short for self: &Self; and it is only a reference, not ownership of self
        //only want to read the data in the structure, not write to the structure
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        //other is an immutable reference to a Rectangle structure
    }

    fn square(size: u32) -> Self { //associated function but not a method
        //does not use an instance of the type of the implementation block
        Self{
            width: size,
            height: size,
        }
    }

}