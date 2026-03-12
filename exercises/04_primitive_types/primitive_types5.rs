// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// I AM DONE

struct Pet {
    name: &'static str, age: f32
}

fn main() {
    let cat = Pet{name: "Furry McFurson", age: 3.5};
    let Pet{name, age} = cat;

    println!("{} is {} years old.", name, age);
}
