// Exemplifies how modules work in Rust.

// Module structure:
//
// main
//  - mymodule
//      - nested
//  - factorial
//  - ...

pub const OTHER_TEXT: &str = "OTHER";

pub use nested::TEXT;
pub use nested::private;

mod nested {
    pub const TEXT: &str = "HELLO";

    pub fn private(){}
}
