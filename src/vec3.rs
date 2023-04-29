use std::ops;

#[derive(Copy,Clone)]
pub struct Vec3 {
    pub slice: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            slice: [e0, e1, e2],
        }
    }

    pub fn x(&self) -> f32 {
        self.slice[0]
    }

    pub fn y(&self) -> f32 {
        self.slice[1]
    }

    pub fn z(&self) -> f32 {
        self.slice[2]
    }

    pub fn lenght_squared(&self) -> f32 {
        self.slice[0] * self.slice[0]
            + self.slice[1] * self.slice[1]
            + self.slice[2] * self.slice[2]
    }
    pub fn lenght(&self) -> f32 {
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

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.slice[0] *= rhs;
        self.slice[1] *= rhs;
        self.slice[2] *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.slice[0] /= rhs;
        self.slice[1] /= rhs;
        self.slice[2] /= rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

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

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
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

impl ops::Mul<Vec3> for f32 {
    type Output= Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs*self
    }
}

impl ops::Mul<(f32, Vec3)> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: (f32, Vec3)) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] * rhs.0 * rhs.1.slice[0],
                self.slice[1] * rhs.0 * rhs.1.slice[1],
                self.slice[2] * rhs.0 * rhs.1.slice[2],
            ],
        }
    }
}

impl ops::Mul<(Vec3, f32)> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: (Vec3, f32)) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] * rhs.1 * rhs.0.slice[0],
                self.slice[1] * rhs.1 * rhs.0.slice[1],
                self.slice[2] * rhs.1 * rhs.0.slice[2],
            ],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            slice: [
                self.slice[0] / rhs,
                self.slice[1] / rhs,
                self.slice[2] / rhs,
            ],
        }
    }
}
pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
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
