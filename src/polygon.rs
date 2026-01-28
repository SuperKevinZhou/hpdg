#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    pub fn perimeter(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }
        let mut sum = 0.0;
        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = self.points[(i + 1) % self.points.len()];
            let dx = (a.x - b.x) as f64;
            let dy = (a.y - b.y) as f64;
            sum += (dx * dx + dy * dy).sqrt();
        }
        sum
    }

    pub fn area(&self) -> f64 {
        if self.points.len() < 3 {
            return 0.0;
        }
        let mut sum: i128 = 0;
        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = self.points[(i + 1) % self.points.len()];
            sum += (a.x as i128) * (b.y as i128) - (a.y as i128) * (b.x as i128);
        }
        (sum.abs() as f64) * 0.5
    }
}
