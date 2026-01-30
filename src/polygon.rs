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

    pub fn convex_hull(points: &[Point]) -> Self {
        if points.len() <= 1 {
            return Self::new(points.to_vec());
        }
        let mut pts = points.to_vec();
        pts.sort_by_key(|p| (p.x, p.y));

        fn cross(o: Point, a: Point, b: Point) -> i128 {
            let ox = o.x as i128;
            let oy = o.y as i128;
            let ax = a.x as i128;
            let ay = a.y as i128;
            let bx = b.x as i128;
            let by = b.y as i128;
            (ax - ox) * (by - oy) - (ay - oy) * (bx - ox)
        }

        let mut lower: Vec<Point> = Vec::new();
        for p in &pts {
            while lower.len() >= 2 {
                let l = lower.len();
                if cross(lower[l - 2], lower[l - 1], *p) <= 0 {
                    lower.pop();
                } else {
                    break;
                }
            }
            lower.push(*p);
        }

        let mut upper: Vec<Point> = Vec::new();
        for p in pts.iter().rev() {
            while upper.len() >= 2 {
                let l = upper.len();
                if cross(upper[l - 2], upper[l - 1], *p) <= 0 {
                    upper.pop();
                } else {
                    break;
                }
            }
            upper.push(*p);
        }

        lower.pop();
        upper.pop();
        lower.extend(upper);
        Self::new(lower)
    }

    pub fn simple_polygon(points: &[Point]) -> Self {
        if points.len() <= 2 {
            return Self::new(points.to_vec());
        }
        let (mut sum_x, mut sum_y) = (0i128, 0i128);
        for p in points {
            sum_x += p.x as i128;
            sum_y += p.y as i128;
        }
        let cx = sum_x as f64 / points.len() as f64;
        let cy = sum_y as f64 / points.len() as f64;
        let mut pts = points.to_vec();
        pts.sort_by(|a, b| {
            let ang_a = (a.y as f64 - cy).atan2(a.x as f64 - cx);
            let ang_b = (b.y as f64 - cy).atan2(b.x as f64 - cx);
            ang_a.partial_cmp(&ang_b).unwrap_or(std::cmp::Ordering::Equal)
        });
        Self::new(pts)
    }
}
