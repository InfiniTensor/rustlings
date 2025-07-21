fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let (name, age) = cat; // Destructuring the tuple into name and age
    println!("{} is {} years old",
             name,
             age);
}
