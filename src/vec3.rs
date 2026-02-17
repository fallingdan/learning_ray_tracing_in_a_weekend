use std::ops::{Add, Div, Mul, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn dotproduct(v: &Vec3, u: &Vec3) -> f64 {
        (v.e[0] * u.e[0]) + (v.e[1] * u.e[1]) + (v.e[2] * u.e[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let actual: Vec3 = Vec3::new(1.0, 2.0, 3.0);

        let expected: [f64; 3] = [1.0, 2.0, 3.0];

        assert!(actual.e == expected)
    }

    #[test]
    fn test_x() {
        let subject: Vec3 = Vec3::new(1.0, 2.0, 3.0);

        assert!(subject.x() == 1.0);
    }

    #[test]
    fn test_y() {
        let subject: Vec3 = Vec3::new(1.0, 2.0, 3.0);

        assert!(subject.y() == 2.0);
    }

    #[test]
    fn test_z() {
        let subject: Vec3 = Vec3::new(1.0, 2.0, 3.0);

        assert!(subject.z() == 3.0);
    }

    #[test]
    fn test_add() {
        let lhs: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let rhs: Vec3 = Vec3::new(1.0, 2.0, 3.0);

        let expected: [f64; 3] = [2.0, 4.0, 6.0];
        let actual: Vec3 = lhs + rhs;

        assert!(actual.e == expected)
    }

    #[test]
    fn test_neg() {
        let subject = Vec3::new(1.0, 1.0, 1.0);
        let actual = -subject;

        assert!([-1.0, -1.0, -1.0] == actual.e);
    }

    #[test]
    fn test_dotproduct() {
        let left = Vec3::new(2.0, 3.0, 4.0);
        let right = Vec3::new(3.0, 4.0, 5.0);

        let expected: f64 = 38.0;

        let actual = Vec3::dotproduct(&left, &right);

        assert!(actual == expected);
    }

    #[test]
    fn test_mul() {
        let subject = Vec3::new(2.0, 3.0, 4.0);
        let right = Vec3::new(3.0, 4.0, 5.0);

        let expected: [f64; 3] = [6.0, 12.0, 20.0];

        let actual = subject * right;

        assert!(actual.e == expected);
    }
}
