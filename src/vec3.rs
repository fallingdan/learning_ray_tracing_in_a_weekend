use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e1, e2, e3] }
    }

    pub fn x() -> f64 {
        e[0]
    }

    pub fn y() -> f64 {
        e[1]
    }

    pub fn z() -> f64 {
        e[2]
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self::x() + rhs::x(),
            self::y() + rhs::y(),
            self::z() + rhs::z(),
        )
    }
}
