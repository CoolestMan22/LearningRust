fn main() {
    // Number types
    let ti32: i32 = 214483647; // i32 (typed: 32-bit signed integer)
    let _ti64: i64 = 9223372036854775807; // i64 (typed: 64-bit signed integer)
    let _tf32: f32 = 214483647.0; // f32 (typed: 32-bit floating point)
    let _tf64: f64 = 9223372036854775807.0; // f64 (typed: 64-bit floating point)

    // Boolean type
    let _tbool: bool = true; // bool (typed: boolean)

    // Character type
    let _tchar: char = 'a'; // char (typed: character)

    // Compound types
    let tarray = [1, 2, 3]; // array (typed: fixed-size array)
    let _ttuple = (1, 2, 3); // tuple (typed: tuple)

    // Pointer types
    let _treference = &ti32; // reference (typed: reference)
    let _tslice = &tarray[..]; // slice (typed: slice)

    // Ownership types
    let _tstring: String = "Hello, world!"; // string (typed: string)
    let _tvector = vec![1, 2, 3]; // vector (typed: vector)

    // Function types
    let _tfn = |x: i32| -> i32 { x + 1 }; // fn (typed: function pointer)

    // Never type
    let _tnever = panic!("this is a never type!"); // never (typed: never)
}
