use super::{point::Point2D, x::X, y::Y};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "valuable", derive(valuable::Valuable))]
pub struct Vector {
    x: X,
    y: Y,
}

impl Vector {
    pub fn new(mag_x: X, mag_y: Y) -> Self {
        Self { x: mag_x, y: mag_y }
    }

    pub fn from_2_points(start: Point2D, end: Point2D) -> Self {
        Self {
            x: end.x - start.x,
            y: end.y - start.y,
        }
    }
}

impl From<Point2D> for Vector {
    /// Creates a vector from the origin [Point2D::origin()] to the point.
    fn from(value: Point2D) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<(Point2D, Point2D)> for Vector {
    /// Vector from point 0 to point 1 using [Self::from_2_points].
    ///
    /// ```
    /// Self::from_2_points(value.0, value.1)
    /// ```
    fn from(value: (Point2D, Point2D)) -> Self {
        Self::from_2_points(value.0, value.1)
    }
}
