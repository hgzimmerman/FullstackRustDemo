use vector::Vector;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
use std::ops::AddAssign;
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    /// Rotates the point based on the heading
    pub fn apply_heading(&self, mut heading: Vector) -> Point {
        heading.normalize();
        Point {
            x: self.x * heading.y - self.y * -heading.x,
            y: self.y * heading.y + self.x * -heading.x
        }
    }

    /// Distance to another point
    pub fn distance(&self, other: &Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y).abs()
    }

    /// A vector from self to another point.
    pub fn vector_to_other(&self, other: &Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

#[test]
fn heading_test() {
    let mut pt = Point {
        x: 50f64,
        y: 50f64
    };
    let heading = Vector {
        x: 0f64,
        y: 10f64
    };
    pt.apply_heading(heading);
    assert!(pt.x == 50f64);
    assert!(pt.y != 50f64);
}



use stdweb::web::CanvasRenderingContext2d;
impl Point {
    pub fn draw(&self, context: &mut CanvasRenderingContext2d, color: &str) {
        context.begin_path();

        use std::f64;
        use stdweb::web::FillRule;
        const RADIUS: f64 = 5f64;
        context.arc(self.x, self.y, RADIUS, 0f64, f64::consts::PI * 2f64, false );

        context.set_fill_style_color(color);
        context.fill(FillRule::NonZero);
    }
}

use stdweb::web::event::ClickEvent;
use stdweb::traits::IMouseEvent;
impl From<ClickEvent> for Point {
    fn from(click_event: ClickEvent) -> Self {
        Point {
            x: click_event.offset_x(),
            y: click_event.offset_y()
        }
    }
}
