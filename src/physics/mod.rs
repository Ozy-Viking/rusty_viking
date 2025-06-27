pub(crate) mod point;
pub(crate) mod vector;
pub(crate) mod x;
pub(crate) mod y;
pub(crate) mod z;

/// Alias for New type structs [i128].
pub type Number = i128;

pub use point::Point2D;
pub use vector::Vector;
pub use x::X;
pub use y::Y;
pub use z::Z;
