use crate::ray::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }

    pub fn normal(&self) -> Vec3 {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        ab.cross(&ac).normalized()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let normal = self.normal();
        let denom = normal.dot(&ray.direction);
        if denom.abs() < 0.0001 {
            return None;
        }
        let d = normal.dot(&self.a);
        let t = (d - normal.dot(&ray.origin)) / denom;
        if t < 0.0 {
            return None;
        }
        let p = ray.origin + ray.direction * t;
        let ab = self.b - self.a;
        let bc = self.c - self.b;
        let ca = self.a - self.c;
        let ap = p - self.a;
        let bp = p - self.b;
        let cp = p - self.c;
        let c1 = ab.cross(&ap);
        let c2 = bc.cross(&bp);
        let c3 = ca.cross(&cp);
        if c1.dot(&c2) >= 0.0 && c2.dot(&c3) >= 0.0 {
            Some(t)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    data: std::simd::f32x4,
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.data.to_array()[0]
    }

    pub fn y(&self) -> f32 {
        self.data.to_array()[1]
    }

    pub fn z(&self) -> f32 {
        self.data.to_array()[2]
    }

    pub fn zero() -> Self {
        Self {
            data: std::simd::f32x4::from_array([0.0, 0.0, 0.0, 0.0]),
        }
    }

    pub fn one() -> Self {
        Self {
            data: std::simd::f32x4::from_array([1.0, 1.0, 1.0, 0.0]),
        }
    }

    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: std::simd::f32x4::from_array([x, y, z, 0.0]),
        }
    }

    pub fn length(&self) -> f32 {
        (self * self).data.to_array().sum().sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        self.from(self.x() / length, self.y() / length, self.z() / length);
    }

    pub fn normalized(&self) -> Self {
        let mut normalized = self.clone();
        normalized.normalize();
        normalized
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3 {
            x: rand::random::<f32>() * 2.0 - 1.0,
            y: rand::random::<f32>() * 2.0 - 1.0,
            z: rand::random::<f32>() * 2.0 - 1.0,
        }
        .normalized()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// impl std::ops::Sub<Vec3> for Vec3 {
//     type Output = Self;
//
//     fn sub(self, other: Self) -> Self {
//         let a_simd = std::simd::f32x4::from_array([self.x, self.y, self.z, 0.0]);
//         let b_simd = std::simd::f32x4::from_array([other.x, other.y, other.z, 0.0]);
//         let result = (a_simd - b_simd).to_array();
//         return Self {
//             x: result[0],
//             y: result[1],
//             z: result[2],
//         };
//     }
// }

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

/// Element-wise multiplication
impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

/// Element-wise multiplication
impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl std::ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        };
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
