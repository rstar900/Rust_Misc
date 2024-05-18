// This file defines the interface or basically the declarations as described in a C header file but in it's rust version
// use repr attribute to tell that we want the struct to be C compatible, hence no Rust optimizations
#[repr(C)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

// The extern "C" tells that the block consists of declarations of code which will come from C in this case
extern "C" {
    pub fn print_coordinates(coord: Coordinate);
}