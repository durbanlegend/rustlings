// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// I AM DONE

fn main() {
    let a = ["Are we there yet?"; 100];

    let len = a.len();
    if len >= 100 {
        println!("Wow, {} - that's a big array!", len);
    } else {
        println!("Meh, {} - I eat arrays like that for breakfast.", len);
        panic!("Array not big enough, more elements needed")
    }
}
