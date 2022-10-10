use super::{HEIGHT, WIDTH};

pub type Direction = f64;

macro_rules! point {
    ($x:expr, $y:expr) => {
        Point::new($x, $y)
    };
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn world_height() -> f64 {
    unsafe { HEIGHT.clone() }
}

pub fn world_width() -> f64 {
    unsafe { WIDTH.clone() }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        let mut point = Point { x, y };
        point.wrap();
        point
    }

    pub fn new_random() -> Point {
        let mut x: f64 = rand::random::<f64>();
        let mut y: f64 = rand::random::<f64>();
        x *= world_width();
        y *= world_height();
        point!(x, y)
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
        self.wrap();
    }

    pub fn translate2(&self, dx: f64, dy: f64) -> Point {
        let mut point = point!(self.x + dx, self.y + dy);
        point.wrap();
        point
    }

    pub fn translate3(&mut self, direction: Direction, distance: f64) {
        self.translate(direction.cos() * distance, direction.sin() * distance);
    }

    pub fn translate4(&self, direction: Direction, distance: f64) -> Point {
        let mut position = self.clone();
        position.translate(direction.cos() * distance, direction.sin() * distance);
        position
    }

    pub fn direction_to(&self, other: &Point) -> Direction {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dy.atan2(dx)
    }

    fn wrap(&mut self) {
        unsafe {
            self.x += WIDTH;
            self.y += HEIGHT;
            self.x %= WIDTH;
            self.y %= HEIGHT;
        }
    }

    pub fn from_polar(r: f64, theta: f64) -> Point {
        point!(r * theta.cos(), r * theta.sin())
    }

    pub fn to_polar(&self) -> (f64, f64) {
        (self.distance(&point!(0.0, 0.0)), self.y.atan2(self.x))
    }
}
