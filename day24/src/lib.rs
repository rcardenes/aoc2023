use std::{io::BufRead, ops, fmt::Display};

type f64 = fixed::types::I120F8;

#[derive(Debug)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn within_2d_boundaries(&self, min: i64, max: i64) -> bool {
        self.x >= min && self.x <= max &&
            self.y >= min && self.y <= max
    }
}

impl From<&str> for Point3D {
    fn from(value: &str) -> Self {
        let mut it = value.split(",");

        Point3D {
            x: it.next().map(|s| f64::from_str(s.trim()).unwrap()).unwrap(),
            y: it.next().map(|s| f64::from_str(s.trim()).unwrap()).unwrap(),
            z: it.next().map(|s| f64::from_str(s.trim()).unwrap()).unwrap(),
        }
    }
}

impl ops::Add<Point3D> for &Point3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<i64> for &Point3D {
    type Output = Point3D;

    fn mul(self, rhs: i64) -> Self::Output {
        let rhs = f64::from_num(rhs);
        Point3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[derive(Debug)]
pub struct Hailstone {
    position: Point3D,
    velocity: Point3D,
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {} @ {}, {}, {}",
               self.position.x, self.position.y, self.position.z,
               self.velocity.x, self.velocity.y, self.velocity.z
               )
    }
}

impl Hailstone {
    // Position of the hailstone after k nanoseconds have passed
    fn pos_at(&self, k: i64) -> Point3D {
        &self.position + (&self.velocity * k)
    }

    pub fn is_future(&self, point: &Point3D) -> bool {
        let t = (point.x - self.position.x) / self.velocity.x;

        t > 0.0
    }

    pub fn intersection(&self, other: &Hailstone) -> Option<Point3D> {
        // find a second position for each hailstone
        let p2_self = self.pos_at(10);
        let p2_other = other.pos_at(10);

        let (x1, y1) = (self.position.x, self.position.y);
        let (x2, y2) = (p2_self.x, p2_self.y);
        let (x3, y3) = (other.position.x, other.position.y);
        let (x4, y4) = (p2_other.x, p2_other.y);

        let det = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));

        if det.abs() > 0.0000000000000001 {
            Some(Point3D {
                x: ((x1*y2 - y1*x2)*(x3 - x4) - (x1 - x2)*(x3*y4 - y3*x4)) / det,
                y: ((x1*y2 - y1*x2)*(y3 - y4) - (y1 - y2)*(x3*y4 - y3*x4)) / det,
                z: f64::from_num(0),
            })
        } else {
            None
        }
    }
}

pub fn read_problem<R: BufRead>(stream: R) -> Vec<Hailstone> {
    let lines = stream.lines().map(|l| l.unwrap());

    lines.into_iter()
    .map(|line| {
        let (pos_raw, vel_raw) = line.split_once(" @ ").unwrap();

        Hailstone {
            position: Point3D::from(pos_raw),
            velocity: Point3D::from(vel_raw),
        }
    })
    .collect()
}
