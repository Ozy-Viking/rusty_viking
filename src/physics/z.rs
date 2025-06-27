use super::Number;

/// Newtype Struct around [Number] for only one axis.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "valuable", derive(valuable::Valuable))]
pub struct Z(pub Number);

impl Z {
    pub fn new(y: impl Into<Number>) -> Self {
        Self(y.into())
    }
}

impl AsRef<Number> for Z {
    fn as_ref(&self) -> &Number {
        &self.0
    }
}
impl AsMut<Number> for Z {
    fn as_mut(&mut self) -> &mut Number {
        &mut self.0
    }
}
impl ::std::ops::Deref for Z {
    type Target = Number;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::ops::DerefMut for Z {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ::std::fmt::Display for Z {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Y({})", self.0)
    }
}
impl ::std::ops::Add for Z {
    type Output = Z;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() + rhs.as_ref())
    }
}
impl ::std::ops::AddAssign for Z {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() + rhs.as_ref();
    }
}
impl ::std::ops::Sub for Z {
    type Output = Z;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() - rhs.as_ref())
    }
}
impl ::std::ops::SubAssign for Z {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() - rhs.as_ref();
    }
}
impl ::std::ops::Mul for Z {
    type Output = Z;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.as_ref() * rhs.as_ref())
    }
}

impl ::std::ops::MulAssign for Z {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() * rhs.as_ref();
    }
}

impl ::std::ops::Mul<Number> for Z {
    type Output = Z;

    fn mul(self, rhs: Number) -> Self::Output {
        Self(self.as_ref() * rhs)
    }
}

impl ::std::ops::MulAssign<Number> for Z {
    fn mul_assign(&mut self, rhs: Number) {
        self.0 = self.as_ref() * rhs;
    }
}
impl ::std::ops::Div<Number> for Z {
    type Output = Z;

    fn div(self, rhs: Number) -> Self::Output {
        Self(self.as_ref() / rhs)
    }
}

impl ::std::ops::DivAssign for Z {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = self.as_ref() / rhs.as_ref();
    }
}
