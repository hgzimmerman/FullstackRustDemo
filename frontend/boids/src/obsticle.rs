
use ecs::Entity;
use ecs::Increment;
use ecs::Positioned;
use ecs::Index;
use point::Point;
use stdweb::web::CanvasRenderingContext2d;
use ecs::Drawable;


#[derive(Clone, Copy, Debug, PartialOrd, Ord, Eq, Default)]
pub struct ObsticleId(u16);

impl PartialEq for ObsticleId {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Increment for ObsticleId {
    fn increment(&self) -> Self {
        ObsticleId(self.0.wrapping_add(1))
    }
}
impl Index for ObsticleId {
    fn as_index(&self) -> usize {
        self.0 as usize
    }
    fn from_index(index: usize) -> Self {
        ObsticleId(index as u16)
    }
}

pub struct Obsticle {
    id: ObsticleId,
    position: Point,
}

impl Obsticle {
    fn new(id: ObsticleId, position: Point) -> Obsticle {
        Obsticle {
            id,
            position,
        }
    }
}

impl Entity for Obsticle {
    type Id = ObsticleId;

    fn get_id(&self) -> Self::Id {
        self.id
    }
    fn create(id: Self::Id) -> Obsticle {
        Obsticle::new(id, Point::default())
    }
}

impl Positioned for Obsticle {
    fn create_positioned(id: ObsticleId, position: Point) -> Self {
        Obsticle::new(id, position)
    }
    fn get_position(&self) -> Point {
        self.position
    }
}



impl Drawable for Obsticle {
    fn draw(&self, context: &mut CanvasRenderingContext2d) {
        self.position.draw(context, "maroon")
    }
}