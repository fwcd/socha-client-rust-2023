use std::{fmt, ops::{Add, Sub, Mul, Div, DivAssign, MulAssign}};

use crate::util::{Element, SCError, SCResult};

/// The four vectors in cardinal direction.
pub const CARDINALS: [Vec2; 4] = [
    Vec2 { x: -1, y:  0 },
    Vec2 { x:  1, y:  0 },
    Vec2 { x:  0, y: -1 },
    Vec2 { x:  0, y:  1 },
];

/// The four vectors in diagonal direction.
pub const DIAGONALS: [Vec2; 4] = [
    Vec2 { x: -1, y: -1 },
    Vec2 { x: -1, y:  1 },
    Vec2 { x:  1, y: -1 },
    Vec2 { x:  1, y:  1 },
];

/// A position on the board or 2D integer vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::zero()
    }
}

impl Vec2 {
    /// The coordinate origin, i.e. (0, 0).
    #[inline]
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    /// Creates a new vector from the given x- and y-components.
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// The area of the rectangle spanned by this vector.
    #[inline]
    pub fn area(self) -> i32 { (self.x * self.y).abs() }

    /// The squared length of this vector.
    #[inline]
    pub fn squared_length(self) -> i32 { self.x * self.x + self.y * self.y }

    /// The length of this vector.
    #[inline]
    pub fn length(self) -> f32 { (self.squared_length() as f32).sqrt() }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

impl MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<i32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<i32> for Vec2 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl TryFrom<&Element> for Vec2 {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Vec2::new(elem.attribute("x")?.parse()?, elem.attribute("y")?.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, game::Vec2};

    #[test]
    fn test_parsing() {
        assert_eq!(Vec2::try_from(&Element::from_str(r#"
            <coords x="23" y="0" />
        "#).unwrap()).unwrap(), Vec2::new(23, 0));
    }
}
