#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    match &optional_point {
        Some(optional_point) => println!("Coordinates are {},{}", optional_point.x, optional_point.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
