#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point2d {
    pub x: i64,
    pub y: i64,
}

impl Point2d {
    #[allow(dead_code)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    pub fn area(&self, other: &Self) -> i64 {
        let dx = (self.x - other.x).abs() + 1;
        let dy = (self.y - other.y).abs() + 1;
        dx * dy
    }
}

impl From<&str> for Point2d {
    fn from(value: &str) -> Self {
        let parts = value.split(',').collect::<Vec<&str>>();
        let x = parts[0].trim().parse::<i64>().unwrap();
        let y = parts[1].trim().parse::<i64>().unwrap();
        Self { x, y }
    }
}

impl std::ops::Add for Point2d {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point2d {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<i64> for Point2d {
    type Output = Self;
    
    fn mul(self, scalar: i64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Neg for Point2d {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Point2d {
    #[allow(dead_code)]
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        ((dx * dx) + (dy * dy)).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Line2D {
    pub start: Point2d,
    pub end: Point2d,
}

impl Line2D {
    #[allow(dead_code)]
    pub fn intersects(&self, other: &Self) -> bool {
        fn ccw(a: &Point2d, b: &Point2d, c: &Point2d) -> bool {
            (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
        }

        ccw(&self.start, &other.start, &other.end) != ccw(&self.end, &other.start, &other.end)
            && ccw(&self.start, &self.end, &other.start) != ccw(&self.start, &self.end, &other.end)
    }

    #[allow(dead_code)]
    pub fn contains(&self, point: Point2d) -> bool {
        let cross_product = (point.y - self.start.y) * (self.end.x - self.start.x) - (point.x - self.start.x) * (self.end.y - self.start.y);
        if cross_product != 0 {
            return false;
        }

        let dot_product = (point.x - self.start.x) * (self.end.x - self.start.x) + (point.y - self.start.y) * (self.end.y - self.start.y);
        if dot_product < 0 {
            return false;
        }

        let squared_length = (self.end.x - self.start.x) * (self.end.x - self.start.x) + (self.end.y - self.start.y) * (self.end.y - self.start.y);
        if dot_product > squared_length {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect {
    pub p1: Point2d,
    pub p2: Point2d,
}

impl Rect {
    #[allow(dead_code)]
    pub fn contains(&self, point: Point2d) -> bool {
        point.x >= self.p1.x && point.x <= self.p2.x &&
        point.y >= self.p1.y && point.y <= self.p2.y
    }

    #[allow(dead_code)]
    pub fn intersects_line(&self, line : &Line2D) -> bool {
        let rect_lines = vec![
            Line2D { start: self.p1, end: Point2d { x: self.p2.x, y: self.p1.y } },
            Line2D { start: Point2d { x: self.p2.x, y: self.p1.y }, end: self.p2 },
            Line2D { start: self.p2, end: Point2d { x: self.p1.x, y: self.p2.y } },
            Line2D { start: Point2d { x: self.p1.x, y: self.p2.y }, end: self.p1 },
        ];

        for rect_line in rect_lines {
            if rect_line.intersects(line) {
                return true;
            }
        }

        false
    }
}