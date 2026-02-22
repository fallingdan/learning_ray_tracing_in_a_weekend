use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

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

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        (self.e[0] * self.e[0])
            + (self.e[1] * self.e[1])
            + (self.e[2] * self.e[2])
    }

    pub fn dotproduct(u: &Vec3, v: &Vec3) -> f64 {
        (v.e[0] * u.e[0]) + (v.e[1] * u.e[1]) + (v.e[2] * u.e[2])
    }

    pub fn crossproduct(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            (u.e[1] * v.e[2]) - (u.e[2] * v.e[1]),
            (u.e[2] * v.e[0]) - (u.e[0] * v.e[2]),
            (u.e[0] * v.e[1]) - (u.e[1] * v.e[0]),
        )
    }

    pub fn unitvector(self) -> Vec3 {
        self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
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

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] *= 1.0 / rhs;
        self.e[1] *= 1.0 / rhs;
        self.e[2] *= 1.0 / rhs;
    }
}

#[cfg(test)]
mod test {
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
    fn test_length() {
        let subject = Vec3::new(1.0, 2.0, 3.0);

        let expected = 14.0_f64.sqrt();

        let actual = subject.length();

        assert!(actual == expected);
    }

    #[test]
    fn test_length_squared() {
        let subject = Vec3::new(1.0, 2.0, 3.0);

        let expected = 14.0;

        let actual = subject.length_squared();

        assert!(actual == expected);
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
    fn test_add_assign() {
        let rhs = Vec3::new(2.0, 2.0, 2.0);
        let mut actual = Vec3::new(1.0, 1.0, 1.0);

        actual += rhs;

        let expected_vector = [3.0, 3.0, 3.0];

        assert!(actual.e == expected_vector);
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
    fn test_crossproduct() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let rhs = Vec3::new(3.0, 2.0, 1.0);

        let expected = Vec3::new(-4.0, 8.0, -4.0);

        let actual = Vec3::crossproduct(&lhs, &rhs);

        assert!(actual == expected);
    }

    #[test]
    fn test_sub() {
        let lhs = Vec3::new(5.0, 4.0, 3.0);
        let rhs = Vec3::new(4.0, 3.0, 6.0);

        let expected = Vec3::new(1.0, 1.0, -3.0);

        let actual = lhs - rhs;

        assert!(actual == expected);
    }

    #[test]
    fn test_unitvector() {
        let subject = Vec3::new(1.0, 2.0, 3.0);

        let length = 14.0_f64.sqrt();
        let expected_e0 = 1.0 / length;
        let expected_e1 = 2.0 / length;
        let expected_e2 = 3.0 / length;

        let expected = Vec3::new(expected_e0, expected_e1, expected_e2);

        let actual = subject.unitvector();

        println!("{:?} {:?}", actual, expected);

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

    #[test]
    fn test_div() {
        let subject = Vec3::new(2.0, 2.0, 2.0);
        let float: f64 = 2.0;

        let expected = Vec3::new(1.0, 1.0, 1.0);

        let actual = subject / float;

        assert!(actual == expected);
    }

    #[test]
    fn test_divassign() {
        let mut actual = Vec3::new(2.0, 2.0, 2.0);
        let float: f64 = 2.0;

        let expected = Vec3::new(1.0, 1.0, 1.0);

        actual /= float;

        assert!(actual == expected);
    }
}
