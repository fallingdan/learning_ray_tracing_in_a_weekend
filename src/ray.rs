use crate::vec3::{Point3, Vec3};

#[derive(PartialEq)]
struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + (t * self.direction)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let origin: Point3 = Point3::new(1.0, 1.0, 1.0);
        let direction: Vec3 = Vec3::new(1.0, 1.0, 1.0);

        let expected = Ray { origin, direction };

        let actual = Ray::new(origin, direction);

        assert!(actual == expected);
    }

    #[test]
    fn test_at() {
        let origin: Point3 = Point3::new(1.0, 1.0, 1.0);
        let direction: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let subject = Ray { origin, direction };

        let expected = Point3::new(3.0, 3.0, 3.0);

        let actual = subject.at(2.0);

        assert!(actual == expected);
    }
}
