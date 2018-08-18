
use ecs::Entity;
use ecs::Increment;
use ecs::Positioned;
use ecs::Positionable;
use ecs::Mobile;
use point::Point;
use vector::Vector;


use ecs::Drawable;
use ecs::Index;

use stdweb::web::CanvasRenderingContext2d;
use stdweb::web::FillRule;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, Eq, Default)]
pub struct BoidId(u16);

impl PartialEq for BoidId {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Increment for BoidId {
    fn increment(&self) -> Self {
        BoidId(self.0.wrapping_add(1))
    }
}
impl Index for BoidId {
    fn as_index(&self) -> usize {
        self.0 as usize
    }
    fn from_index(index: usize) -> Self {
        BoidId(index as u16)
    }
}


pub struct Boid {
    id: BoidId,
    position: Point,
    heading: Vector,
    pub acceleration: Vector
}

impl Boid {
    fn new(id: BoidId, position: Point, heading: Vector) -> Boid {
        Boid {
            id,
            position,
            heading,
            acceleration: Vector::default()
        }
    }


}

impl Entity for Boid {
    type Id = BoidId;

    fn get_id(&self) -> BoidId {
        self.id
    }
    fn create(id: BoidId) -> Boid {
        Boid::new(id, Point::default(), Vector::default())
    }
}

impl Positioned for Boid {
    fn create_positioned(id: BoidId, position: Point) -> Self {
        Boid::new(id, position, Vector::default())
    }
    fn get_position(&self) -> Point {
        self.position
    }
}

impl Positionable for Boid {
    fn mut_position(&mut self) -> &mut Point {
        &mut self.position
    }
}

impl Drawable for Boid {
    fn draw(&self, context: &mut CanvasRenderingContext2d) {
        let position: Point = self.get_position();
        let heading: Vector = self.get_heading();
        context.begin_path();

        let pt1 = Point {x:   0f64, y: 20f64};
        let pt2 = Point {x: -5f64, y: -10f64};
        let pt3 = Point {x:  5f64, y: -10f64};

        let pt1 = pt1.apply_heading(heading);
        let pt2 = pt2.apply_heading(heading);
        let pt3 = pt3.apply_heading(heading);


        context.move_to(position.x + pt1.x, position.y + pt1.y);
        context.line_to(position.x + pt2.x, position.y + pt2.y);
        context.line_to(position.x + pt3.x, position.y + pt3.y);

        context.close_path();
        context.set_fill_style_color("teal");
        context.fill(FillRule::NonZero);


        // boid center point
        const RADIUS: f64 = 2f64;
        use std::f64;
        context.begin_path();
        context.arc(position.x, position.y, RADIUS, 0f64, f64::consts::PI * 2f64, false );
        context.set_fill_style_color("yellow");
        context.fill(FillRule::NonZero);
        context.close_path()
    }
}

impl Mobile for Boid {
    fn create_mobile(id: BoidId, position: Point, heading: Vector) -> Self {
        Boid::new(id, position, heading)
    }
    fn get_heading(&self) -> Vector {
        self.heading
    }
    fn mut_heading(&mut self) -> &mut Vector {
        &mut self.heading
    }
}


