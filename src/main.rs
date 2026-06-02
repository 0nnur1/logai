#[cfg(not(target_pointer_width = "64"))]
compile_error!(
    "This software requires a 64-bit target architecture and cannot be compiled on 32-bit systems."
);

mod data_register;
mod genome;

use crate::data_register::WorldState;

fn main() {
    print!("fortnite")
}
