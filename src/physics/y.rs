use super::Number;

/// Newtype Struct around [Number] for only one axis.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Y(pub Number);

impl Y {
    pub fn new(y: impl Into<Number>) -> Self {
        Self(y.into())
    }
}

impl AsRef<Number> for Y {
    fn as_ref(&self) -> &Number {
        &self.0
    }
}
impl AsMut<Number> for Y {
    fn as_mut(&mut self) -> &mut Number {
        &mut self.0
    }
}
impl ::std::ops::Deref for Y {
    type Target = Number;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::ops::DerefMut for Y {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ::std::fmt::Display for Y {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Y({})", self.0)
    }
}
impl ::std::ops::Add for Y {
    type Output = Y;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() + rhs.as_ref())
    }
}
impl ::std::ops::AddAssign for Y {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() + rhs.as_ref();
    }
}
impl ::std::ops::Sub for Y {
    type Output = Y;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() - rhs.as_ref())
    }
}
impl ::std::ops::SubAssign for Y {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() - rhs.as_ref();
    }
}
impl ::std::ops::Mul for Y {
    type Output = Y;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() * rhs.as_ref())
    }
}

impl ::std::ops::MulAssign for Y {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() * rhs.as_ref();
    }
}

impl ::std::ops::Mul<Number> for Y {
    type Output = Y;

    fn mul(self, rhs: Number) -> Self::Output {
        Self(self.as_ref() * rhs)
    }
}

impl ::std::ops::MulAssign<Number> for Y {
    fn mul_assign(&mut self, rhs: Number) {
        self.0 = self.as_ref() * rhs;
    }
}
impl ::std::ops::Div<Number> for Y {
    type Output = Y;

    fn div(self, rhs: Number) -> Self::Output {
        Self(self.as_ref() / rhs)
    }
}

impl ::std::ops::DivAssign for Y {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() / rhs.as_ref();
    }
}
