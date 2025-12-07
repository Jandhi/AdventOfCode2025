#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T: Default> Pos<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(x : T) -> Self {
        Self { x : x, y: T::default() }
    }

    pub fn y(y : T) -> Self {
        Self { x : T::default(), y: y }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Pos<T> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Pos<T> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Pos<T> {
    type Output = Self;
    
    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Pos<T> {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}


pub trait PosIndexable<T> {
    fn at(&self, pos: Pos<usize>) -> &T;
    fn at_mut(&mut self, pos: Pos<usize>) -> &mut T;
}

impl<T> PosIndexable<T> for Vec<Vec<T>> {
    fn at(&self, pos: Pos<usize>) -> &T {
        &self[pos.y][pos.x]
    }

    fn at_mut(&mut self, pos: Pos<usize>) -> &mut T {
        &mut self[pos.y][pos.x]
    }
}