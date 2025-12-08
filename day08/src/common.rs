#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Default> Pos<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Pos<T> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Pos<T> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Pos<T> {
    type Output = Self;
    
    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Pos<T> {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl <T> Pos<T> {
    pub fn distance(&self, other: &Self) -> f64
    where T: std::ops::Sub<Output = T> + Into<f64> + Copy
    {
        let dx: f64 = (self.x - other.x).into();
        let dy: f64 = (self.y - other.y).into();
        let dz: f64 = (self.z - other.z).into();
        ((dx * dx) + (dy * dy) + (dz * dz)).sqrt()
    }
}