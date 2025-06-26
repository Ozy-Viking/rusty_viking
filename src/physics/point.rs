use super::{x::X, y::Y};

use super::Number;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Point2D {
    pub x: X,
    pub y: Y,
}

impl ::std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Point{{ x: {}, y: {} }}",
            self.x.as_ref(),
            self.y.as_ref()
        )
    }
}

impl Point2D {
    pub const ORIGIN: Point2D = Point2D { x: X(0), y: Y(0) };

    pub fn new(x: X, y: Y) -> Self {
        Self { x, y }
    }
}

impl From<(X, Y)> for Point2D {
    fn from(value: (X, Y)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<(Y, X)> for Point2D {
    fn from(value: (Y, X)) -> Self {
        Self {
            x: value.1,
            y: value.0,
        }
    }
}

impl ::std::ops::Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ::std::ops::AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ::std::ops::Sub for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ::std::ops::SubAssign for Point2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ::std::ops::Mul<Number> for Point2D {
    type Output = Point2D;

    fn mul(self, rhs: Number) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn origin() {
        assert_eq!(Point2D { x: X(0), y: Y(0) }, Point2D::ORIGIN);
        assert_ne!(Point2D { x: X(1), y: Y(1) }, Point2D::ORIGIN);
    }
}
