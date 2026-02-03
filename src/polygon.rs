use rand::Rng;
use std::fmt;


/// 2D point with integer coordinates.
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

/// Polygon represented by an ordered list of vertices.
#[derive(Clone, Debug, Default)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    /// Perimeter length of the polygon.
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

    /// Area of the polygon using the shoelace formula.
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

    /// Compute the convex hull (monotonic chain).
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

    /// Build a simple polygon by angular sorting.
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

/// Generate random points within given bounds.
pub fn random_points(num: usize, x_range: (i64, i64), y_range: (i64, i64)) -> Vec<Point> {
    let mut rng = rand::rng();
    let mut points = Vec::with_capacity(num);
    for _ in 0..num {
        let x = rng.gen_range(x_range.0..=x_range.1);
        let y = rng.gen_range(y_range.0..=y_range.1);
        points.push(Point::new(x, y));
    }
    points
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, p) in self.points.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", p)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_and_perimeter() {
        let poly = Polygon::new(vec![
            Point::new(0, 0),
            Point::new(4, 0),
            Point::new(0, 3),
        ]);
        assert!((poly.area() - 6.0).abs() < 1e-6);
        assert!((poly.perimeter() - 12.0).abs() < 1e-6);
    }

    #[test]
    fn test_convex_hull() {
        let points = vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(0, 0),
        ];
        let hull = Polygon::convex_hull(&points);
        assert!(hull.points.len() >= 3);
    }

    #[test]
    fn test_simple_polygon() {
        let points = vec![
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(2, 2),
            Point::new(0, 2),
        ];
        let poly = Polygon::simple_polygon(&points);
        assert_eq!(poly.points.len(), 4);
    }

    #[test]
    fn test_random_points_bounds() {
        let pts = random_points(5, (0, 2), (-1, 1));
        assert_eq!(pts.len(), 5);
        for p in pts {
            assert!(p.x >= 0 && p.x <= 2);
            assert!(p.y >= -1 && p.y <= 1);
        }
    }

    #[test]
    fn test_display_formats() {
        let p = Point::new(1, 2);
        assert_eq!(p.to_string(), "1 2");
        let poly = Polygon::new(vec![Point::new(0, 0), Point::new(1, 1)]);
        assert!(poly.to_string().contains("0 0"));
    }
}
