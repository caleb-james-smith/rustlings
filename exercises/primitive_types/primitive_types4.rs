// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

// https://doc.rust-lang.org/rust-by-example/primitives/array.html

// Slices can point to a section of an array.
// They are of the form [starting_index..ending_index].
// `starting_index` is the first position in the slice.
// `ending_index` is one more than the last position in the slice.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
