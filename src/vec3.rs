use std::ops;

use crate::Float;



#[derive(Copy,Clone)]
pub struct Vec3 {
    pub slice: [Float; 3],
}

impl Vec3 {
    pub fn new(e0: Float, e1: Float, e2: Float) -> Vec3 {
        Vec3 {
            slice: [e0, e1, e2],
        }
    }

    pub fn x(&self) -> Float {
        self.slice[0]
    }

    pub fn y(&self) -> Float {
        self.slice[1]
    }

    pub fn z(&self) -> Float {
        self.slice[2]
    }

    pub fn lenght_squared(&self) -> Float {
        self.slice[0] * self.slice[0]
            + self.slice[1] * self.slice[1]
            + self.slice[2] * self.slice[2]
    }
    pub fn lenght(&self) -> Float {
        self.lenght_squared().sqrt()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            slice: [-self.slice[0], -self.slice[1], -self.slice[2]],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.slice[0] += rhs.slice[0];
        self.slice[1] += rhs.slice[1];
        self.slice[2] += rhs.slice[2];
    }
}

impl ops::MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, rhs: Float) {
        self.slice[0] *= rhs;
        self.slice[1] *= rhs;
        self.slice[2] *= rhs;
    }
}

impl ops::DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, rhs: Float) {
        self.slice[0] /= rhs;
        self.slice[1] /= rhs;
        self.slice[2] /= rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        &self.slice[index]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] + rhs.slice[0],
                self.slice[1] + rhs.slice[1],
                self.slice[2] + rhs.slice[2],
            ],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] - rhs.slice[0],
                self.slice[1] - rhs.slice[1],
                self.slice[2] - rhs.slice[2],
            ],
        }
    }
}

impl ops::Mul<Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Float) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] * rhs,
                self.slice[1] * rhs,
                self.slice[2] * rhs,
            ],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] * rhs.slice[0],
                self.slice[1] * rhs.slice[1],
                self.slice[2] * rhs.slice[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for Float {
    type Output= Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs*self
    }
}

impl ops::Mul<(Float, Vec3)> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: (Float, Vec3)) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] * rhs.0 * rhs.1.slice[0],
                self.slice[1] * rhs.0 * rhs.1.slice[1],
                self.slice[2] * rhs.0 * rhs.1.slice[2],
            ],
        }
    }
}

impl ops::Mul<(Vec3, Float)> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: (Vec3, Float)) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] * rhs.1 * rhs.0.slice[0],
                self.slice[1] * rhs.1 * rhs.0.slice[1],
                self.slice[2] * rhs.1 * rhs.0.slice[2],
            ],
        }
    }
}

impl ops::Div<Float> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Float) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] / rhs,
                self.slice[1] / rhs,
                self.slice[2] / rhs,
            ],
        }
    }
}
pub fn dot(u: &Vec3, v: &Vec3) -> Float {
    u.slice[0] * v.slice[0] + u.slice[1] * v.slice[1] + u.slice[2] * v.slice[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        slice: [
            u.slice[1] * v.slice[2] - u.slice[2] * v.slice[1],
            u.slice[2] * v.slice[0] - u.slice[0] * v.slice[2],
            u.slice[0] * v.slice[1] - u.slice[1] * v.slice[0],
        ],
    }
}

pub fn unit_vector(v:&Vec3)->Vec3{
   let lenght = v.lenght(); 
    *v / lenght
}
