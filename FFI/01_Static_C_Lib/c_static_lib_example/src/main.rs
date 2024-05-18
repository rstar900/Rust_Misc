mod clib_rust_binding;
use clib_rust_binding::*;

fn main() {
    // All external code (in this case, the C function) needs to be run in unsafe block
    unsafe {
        print_coordinates(Coordinate {x: -67.898, y: 8.7611});
    }
}
