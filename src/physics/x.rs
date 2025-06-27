use std::ops::Rem;

use super::Number;

/// Newtype Struct around [Number] for only one axis.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "valuable", derive(valuable::Valuable))]
#[valuable(transparent)]
pub struct X(pub Number);

impl X {
    pub fn new(x: impl Into<Number>) -> Self {
        Self(x.into())
    }
}

impl AsRef<Number> for X {
    fn as_ref(&self) -> &Number {
        &self.0
    }
}
impl AsMut<Number> for X {
    fn as_mut(&mut self) -> &mut Number {
        &mut self.0
    }
}
impl ::std::ops::Deref for X {
    type Target = Number;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::ops::DerefMut for X {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ::std::fmt::Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X({})", self.0)
    }
}

impl ::std::ops::Add for X {
    type Output = X;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() + rhs.as_ref())
    }
}
impl ::std::ops::AddAssign for X {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() + rhs.as_ref();
    }
}

impl ::std::ops::Sub for X {
    type Output = X;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() - rhs.as_ref())
    }
}

impl ::std::ops::SubAssign for X {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() - rhs.as_ref();
    }
}

impl ::std::ops::Mul for X {
    type Output = X;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() * rhs.as_ref())
    }
}

impl ::std::ops::MulAssign for X {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() * rhs.as_ref();
    }
}

impl ::std::ops::Mul<Number> for X {
    type Output = X;

    fn mul(self, rhs: Number) -> Self::Output {
        Self(self.as_ref() * rhs)
    }
}

impl ::std::ops::MulAssign<Number> for X {
    fn mul_assign(&mut self, rhs: Number) {
        self.0 = self.as_ref() * rhs;
    }
}

impl ::std::ops::Div<Number> for X {
    type Output = X;

    fn div(self, rhs: Number) -> Self::Output {
        Self(self.as_ref() / rhs)
    }
}

impl ::std::ops::DivAssign for X {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() / rhs.as_ref();
    }
}

impl ::std::ops::Rem for X {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() % rhs.as_ref())
    }
}

impl ::std::ops::RemAssign for X {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}
