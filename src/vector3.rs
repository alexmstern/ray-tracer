use std::ops;

#[derive(Clone, Debug, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + Copy
        + Into<f64>
        + From<f64>
        + PartialEq
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }

    pub fn magnitude(&self) -> T {
        let x: f64 = self.x.into();
        let y: f64 = self.y.into();
        let z: f64 = self.z.into();
        let mag = (x * x + y * y + z * z).sqrt();
        T::from(mag)
    }

    pub fn unit_vector(&self) -> Vector3<T> {
        let mag = self.magnitude();
        if mag == T::from(0.0) {
            // Return zero vector if the magnitude is zero
            Vector3::new(T::from(0.0), T::from(0.0), T::from(0.0))
        } else {
            let scale = T::from(1.0) / mag;
            Vector3 {
                x: self.x * scale,
                y: self.y * scale,
                z: self.z * scale,
            }
        }
    }
}

impl<T> ops::Add for Vector3<T>
where
    T: ops::Add<Output = T> + Copy,
{
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> ops::Sub for Vector3<T>
where
    T: ops::Sub<Output = T> + Copy,
{
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> ops::Mul<T> for Vector3<T>
where
    T: ops::Mul<Output = T> + Copy,
{
    type Output = Vector3<T>;

    fn mul(self, scalar: T) -> Vector3<T> {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T> ops::Div<T> for Vector3<T>
where
    T: ops::Div<Output = T> + Copy,
{
    type Output = Vector3<T>;

    fn div(self, scalar: T) -> Vector3<T> {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T> PartialEq for Vector3<T>
where
    T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

pub fn dot<T>(v1: Vector3<T>, v2: Vector3<T>) -> T
where
    T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy,
{
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}