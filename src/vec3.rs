use std::ops;
use std::fmt;
use crate::util;

pub type Color3 = Vec3;
pub type Point3 = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    c : [f64; 3]
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            c: [0.0; 3]
        }
    }

    pub fn from_f64(x: f64, y: f64, z: f64) -> Self {
        Self {
            c: [x,y,z]
        }
    }

    pub fn x(&self) -> f64 {
        self.c[0]
    }

    pub fn y(&self) -> f64 {
        self.c[1]
    }

    pub fn z(&self) -> f64 {
        self.c[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.c[0]*self.c[0] + self.c[1]*self.c[1] + self.c[2]*self.c[2]
    }

    pub fn dot(a: &Vec3, b:&Vec3) -> f64 {
        (a.c[0] * b.c[0]) +
        (a.c[1] * b.c[1]) +
        (a.c[2] * b.c[2])
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3::from_f64(a.c[1] * b.c[2] - a.c[2] * b.c[1],
                        a.c[2] * b.c[0] - a.c[0] * b.c[2],
                        a.c[0] * b.c[1] - a.c[1] * b.c[0])
    }

    pub fn unit(a: &Vec3) -> Vec3 {
        *a / a.length()
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3::from_f64(util::random_range(min, max), util::random_range(min, max), util::random_range(min, max))
    }

    pub fn random_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() <= 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit(&Vec3::random_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let v = Vec3::random_unit_sphere();

        if Vec3::dot(normal, &v) < 0.0 {
            return -v;
        }

        v
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.c[0].abs() < s) && (self.c[1].abs() < s) && (self.c[2].abs() < s)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0*Vec3::dot(v, n)*(*n)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Self {
            c: [-self.c[0], -self.c[1], -self.c[2]]
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.c[0] += other.c[0];
        self.c[1] += other.c[1];
        self.c[2] += other.c[2];
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.c[0] *= other.c[0];
        self.c[1] *= other.c[1];
        self.c[2] *= other.c[2];
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.c[0] *= 1.0/other;
        self.c[1] *= 1.0/other;
        self.c[2] *= 1.0/other;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.c[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.c[i]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3[{}, {}, {}]", self.c[0], self.c[1], self.c[2])
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self {
        Self {
            c: [self.c[0] + other.c[0],
                self.c[1] + other.c[1],
                self.c[2] + other.c[2]
                ]
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Self {
            c: [self.c[0] - other.c[0],
                self.c[1] - other.c[1],
                self.c[2] - other.c[2]
                ]
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self {
        Self {
            c: [self.c[0] * other.c[0],
                self.c[1] * other.c[1],
                self.c[2] * other.c[2]
                ]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self {
        Self {
            c: [self.c[0] * other,
                self.c[1] * other,
                self.c[2] * other
                ]
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self {
        self * (1.0/other)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other*self
    }
}
