// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    //let a = ["Are we there yet?"; 100];
    //let a = [0; 100];
    //let a: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a: [i32; 100] = [0; 100];
    
    println!("length of a: {0}", a.len());

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
