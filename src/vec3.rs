use std::{
    ops::{Add, Div, Mul, Neg},
    process::Output,
};

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x() * self, rhs.y() * self, rhs.z() * self)
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_new() {
        let actual: Vec3 = Vec3::new(1.0, 2.0, 3.0);

        let expected = Vec3 { e: [1.0, 2.0, 3.0] };

        assert!(actual == expected);
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

        let expected = Vec3 { e: [2.0, 4.0, 6.0] };

        let actual: Vec3 = lhs + rhs;

        assert!(actual == expected);
    }

    #[test]
    fn test_neg() {
        let subject = Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3 {
            e: [-1.0, -1.0, -1.0],
        };

        let actual = -subject;

        assert!(actual == expected);
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
    fn test_mul_two_vec3() {
        let subject = Vec3::new(2.0, 3.0, 4.0);
        let right = Vec3::new(3.0, 4.0, 5.0);

        let expected = Vec3 {
            e: [6.0, 12.0, 20.0],
        };

        let actual = subject * right;

        assert!(actual == expected);
    }

    #[test]
    fn test_mul_vec3_f64() {
        let vector = Vec3::new(1.0, 2.0, 3.0);
        let float: f64 = 2.0;

        let expected = Vec3 { e: [2.0, 4.0, 6.0] };

        let actual = vector * float;

        assert!(actual == expected);
    }

    #[test]
    fn test_mul_f64_vec3() {
        let vector = Vec3::new(1.0, 2.0, 3.0);
        let float: f64 = 2.0;

        let expected = Vec3 { e: [2.0, 4.0, 6.0] };

        let actual = float * vector;

        assert!(actual == expected);
    }
}
