// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and botton right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is and expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Calculate the area of a rectangle
    let rect_area = rect_area(&_rectangle);
    println!("The area of the rectangle is {}", rect_area);

    // Create a square based on a point and a size
    let my_square = square(point, 5.0);
    println!(
        "Square top left: ({}, {}), bottom right: ({}, {})",
        my_square.top_left.x,
        my_square.top_left.y,
        my_square.bottom_right.x,
        my_square.bottom_right.y
    );
}

// Function to calculate the area of a rectangle
fn rect_area(rectangle: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rectangle;

    let width = (x2 - x1).abs();
    let height = (y2 - y1).abs();
    width * height
}

// Function to create a square Rectangle given a starting point and size
fn square(top_left: Point, size: f32) -> Rectangle {
    let x = top_left.x;
    let y = top_left.y;
    Rectangle {
        top_left,
        bottom_right: Point {
            x: x + size,
            y: y + size,
        },
    }
}
