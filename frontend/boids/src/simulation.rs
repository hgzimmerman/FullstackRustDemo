// use boid::BoidId;
use stdweb::web::{
    CanvasRenderingContext2d
};
use ecs::System;
use boid::Boid;
use obsticle::Obsticle;
use ecs::Drawable;
use ecs::Mobile;
use ecs::Positionable;
use ecs::Positioned;
use ecs::Entity;

use point::Point;
use vector::Vector;

use flocks::Flock;

use rand::{Rng, XorShiftRng, SeedableRng};

pub struct Simulation {
    pub config: SimConfig,
    boids: System<Boid>,
    obsticles: System<Obsticle>,
    flocks: Vec<Flock>,
    rng: XorShiftRng,

}

#[derive(Debug)]
pub struct SimConfig {
    pub obsticle_avoidance_range: f64,
    pub obsticle_avoidance_factor: f64,
    /// The standard deviation for calculating noise for boids.
    pub boid_noise_std_dev: Option<f64>,
    /// The rate at which boids will accelerate to adjust to the flock's heading
    pub flocking_affinity: f64,
    /// The distance between a prospective boid and a boid in a flock 
    /// that the prospective boid must be below in order to be added to the flock.
    pub flocking_distance: f64,
    /// The maximum speed the boids can travel.
    pub max_speed: f64,
    /// The rate at which acceleration returns to 0.
    /// The bigger the factor, the faster the acceleration shrinks.
    /// Should always be above 1
    pub acceleration_damping_factor: f64,
    /// The height and width of the canvas
    pub dimensions: WindowDimensions,
    /// Draw the centerpoint of flocks.
    pub draw_flock_centerpoint: bool
}

impl Default for SimConfig {
    fn default() -> Self {
        let dimensions = WindowDimensions {
            width: 1400f64,
            height: 900f64
        };
        SimConfig {
            obsticle_avoidance_range: 75.0, // 50 - 100
            obsticle_avoidance_factor: 7.0, // 0 - 15 
            boid_noise_std_dev: Some(0.3), // .1 - .5
            flocking_affinity: 1.1, // 1 - 10
            flocking_distance: 100.0, // 70-200
            max_speed: 5.0, // 3 - 10
            acceleration_damping_factor: 2.0, // 10 - 5
            dimensions,
            draw_flock_centerpoint: false
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WindowDimensions {
    pub width: f64,
    pub height: f64
}

impl Simulation {

    pub fn new(config: SimConfig) -> Simulation {
        let seed = [1, 2, 3, 4, 5, 44, 7, 8, 9, 232, 22, 2, 1 , 234, 100, 139];
        Simulation {
            config,
            boids: System::new(),
            obsticles: System::new(),
            flocks: Vec::new(),
            rng: XorShiftRng::from_seed(seed)
        }
    }

    fn add_boid(&mut self, position: Point, heading: Vector) {
        self.boids.add_mobile(position, heading);
    }
    pub fn add_random_boid(&mut self) {
        let position = Point {
            x: self.rng.gen_range(0f64, self.config.dimensions.width),
            y: self.rng.gen_range(0f64, self.config.dimensions.height)
        };
        let max_speed = self.config.max_speed;
        let heading = Vector {
            x: self.rng.gen_range(-max_speed, max_speed),
            y: self.rng.gen_range(-max_speed, max_speed)
        };
        self.add_boid(position, heading);
    }
    pub fn add_boid_at_position(&mut self, position: Point) {
        let heading = Vector::generate_noise(&mut self.rng, self.config.max_speed / 4.0);
        self.add_boid(position, heading)
    }

    pub fn add_obsticle(&mut self, position: Point) {
        self.obsticles.add_positioned(position)
    }
    pub fn add_random_obsticle(&mut self) {
        let position = Point {
            x: self.rng.gen_range(0f64, self.config.dimensions.width),
            y: self.rng.gen_range(0f64, self.config.dimensions.height)
        };
        self.add_obsticle(position);
    }

    #[allow(dead_code)]
    pub fn add_obsticle_border(&mut self) {
        let height = self.config.dimensions.height;
        let width = self.config.dimensions.width;
        let mut position = Point {
            x: 0.0,
            y: 0.0
        };

        while position.x <= width {
            self.add_obsticle(position);
            position.x += 20.0;
        }
        position.x = width;
        while position.y <= height {
            self.add_obsticle(position);
            position.y += 20.0;
        }
        position.y = height;
        while position.x >= 0.0 {
            self.add_obsticle(position);
            position.x -= 20.0;
        }
        position.x = 0.0;
        while position.y >= 0.0 {
            self.add_obsticle(position);
            position.y -= 20.0;
        }
    }

    pub fn clear(&mut self) {
        self.boids.clear();
        self.obsticles.clear();
        self.flocks = Vec::new();
    }

    pub fn remove_near_point(&mut self, point: Point) {
        const REMOVE_RADIUS: f64 = 25.0;
        let mut remove_list = Vec::new();
        self.boids.for_each(|b| {
            if b.get_position().distance(&point) < REMOVE_RADIUS {
                remove_list.push(b.get_id())
            }
        });
        for id in remove_list {
            self.boids.remove(&id)
        }

        let mut remove_list = Vec::new();
        self.obsticles.for_each(|o| {
            if o.get_position().distance(&point) < REMOVE_RADIUS {
                remove_list.push(o.get_id())
            }
        });
        for id in remove_list {
            self.obsticles.remove(&id)
        }
   
        // TODO
    }

    pub fn populate_test(&mut self) {
        let position = Point {
            x: 50f64,
            y: 50f64
        };
        let pos_2 = Point {
            x: 180f64,
            y: 50f64
        };
        let pos_3 = Point {
            x: 400f64,
            y: 27f64
        };
        let vector = Vector {
            x: -0.5f64,
            y: 2f64
        };
        
        let vector_2 = Vector {
            x: 2.5f64,
            y: 0.5f64
        };
        self.boids.add_mobile(position, vector);
        self.boids.add_mobile(pos_2, vector);
        self.boids.add_mobile(pos_3, vector);
        self.boids.add_mobile(pos_3, vector_2);

        // self.add_obsticle_border();
    }

    /// One tick of the simulation
    pub fn tick(&mut self) {
        if let Some(noise_std_dev) = self.config.boid_noise_std_dev {
            Self::add_noise_to_boids(&mut self.boids, &mut self.rng, noise_std_dev);
        }
        Self::dampen_boids_acceleration(&mut self.boids, self.config.acceleration_damping_factor);
        Self::flock_boids_together(&mut self.boids, &mut self.flocks, &mut self.rng, &self.config);
        Self::accelerate_boids_to_avoid_obsticles(&mut self.boids, &self.obsticles, self.config.obsticle_avoidance_factor, self.config.obsticle_avoidance_range);
        Self::apply_heading_to_position_boids(&mut self.boids, self.config.dimensions);
        Self::apply_acceleration_to_boids(&mut self.boids);
        Self::clamp_boid_speed(&mut self.boids, self.config.max_speed);
    }

    /// Prevents the speed of the boids from exceeding a specific value.
    fn clamp_boid_speed(boids: &mut System<Boid>, max_speed: f64) {
        boids
            .apply(|boid|{
                let heading = boid.mut_heading();
                let speed = heading.length();
                // The speed can't exceed the max speed.
                if speed > max_speed {
                    heading.set_length(max_speed);
                }
            });
    }

    /// apply acceleration to heading
    fn apply_acceleration_to_boids(boids: &mut System<Boid>) {
        boids
            .apply(|boid| {
                let acceleration = boid.acceleration;
                let heading = boid.mut_heading();
                heading.x += acceleration.x;
                heading.y += acceleration.y;
            });
    }

    /// apply movement
    fn apply_heading_to_position_boids(boids: &mut System<Boid>, dimensions: WindowDimensions) {
        boids
            .apply(|boid| {
                let width = dimensions.width;
                let height = dimensions.height;
                let heading = boid.get_heading();
                let position = boid.mut_position();
                position.x = (position.x + heading.x + width) % width; // modulous, not remainder
                position.y = (position.y + heading.y + height) % height;
            });
    }

    // Avoid obsticles
    fn accelerate_boids_to_avoid_obsticles(boids: &mut System<Boid>, obsticles: &System<Obsticle>, object_avoidance_factor: f64, object_avoidance_range: f64 ) {
        boids
            .apply(|boid| {
                let boid_pos = boid.get_position();
                obsticles.for_each(|obs|{
                    let obs_pos = obs.get_position();

                    let object_avoidance_inner_range = object_avoidance_range / 3.0;

                    if boid_pos.distance(&obs_pos) < object_avoidance_inner_range {
                        let inner_avoidance = object_avoidance_factor * 3.0;
                        // The closer to an obsticle, the stronger this force will be felt.
                        let accel_x = inner_avoidance / (boid_pos.x - obs_pos.x);
                        let accel_y = inner_avoidance / (boid_pos.y - obs_pos.y);

                        boid.acceleration.x += accel_x;
                        boid.acceleration.y += accel_y;
                    } else if boid_pos.distance(&obs_pos) < object_avoidance_range {
                        let outer_avoidance = object_avoidance_factor;
                        // The closer to an obsticle, the stronger this force will be felt.
                        let accel_x = outer_avoidance / (boid_pos.x - obs_pos.x);
                        let accel_y = outer_avoidance / (boid_pos.y - obs_pos.y);

                        boid.acceleration.x += accel_x;
                        boid.acceleration.y += accel_y;
                    }
                });
            });
    }

    // apply flocking mechanics
    fn flock_boids_together(boids: &mut System<Boid>, flocks: &mut Vec<Flock>, rng: &mut XorShiftRng, config: &SimConfig) {
        // const FLOCKING_DISTANCE: f64 = 130f64;
        let flocking_distance: f64 = config.flocking_distance;
        let flocking_affinity = config.flocking_affinity;

        // replace old flock groupings with neww ones.
        flocks.truncate(0);
        let mut  new_flock = Flock::create_flocks_from_boids(&boids, flocking_distance);
        flocks.append(&mut new_flock);

        for flock in flocks.iter() {
            let avg_heading = flock.avg_heading(&boids);
            // Add some noise to where the flock is heading
            let noise_vector = Vector::generate_noise(rng, 0.2f64);

            // let avg_pt = flock.avg_point_mass(&boids);
            boids.apply_to_ids(&flock.boids, |boid| {

                let x_heading = avg_heading.x + boid.get_heading().x;
                let y_heading = avg_heading.y + boid.get_heading().y;

                let mut acceleration_vector = Vector {
                    x: x_heading,
                    y: y_heading
                };
                // The higher the flocking affinity, the more quickly boids will adjust their heading to be with a flock.
                acceleration_vector.set_length(flocking_affinity);

                boid.acceleration.x += acceleration_vector.x;
                boid.acceleration.y += acceleration_vector.y; 

                boid.acceleration += noise_vector
            });
        }
    }

    /// dampen acceleration
    fn dampen_boids_acceleration(boids: &mut System<Boid>, dampening_factor: f64) {
        boids
            .apply(|boid| {
                // The acceleration will be damped to the current acceleration over this number.
                // Lower means that acceleration is more persistent.
                boid.acceleration.x /= dampening_factor; 
                boid.acceleration.y /= dampening_factor;
            });
    }

    /// Adds some noise to the boid's acceleration
    fn add_noise_to_boids(boids: &mut System<Boid>, rng: &mut XorShiftRng, noise_std_dev: f64) {
        boids
            .apply(|boid| {
                boid.acceleration += Vector::generate_noise(rng, noise_std_dev)
            });
    }


    pub fn draw(&self, context: &mut CanvasRenderingContext2d) {
        let width = self.config.dimensions.width;
        let height = self.config.dimensions.height;
        context.clear_rect(0f64, 0f64, width, height); // reset
        // trace!("drawing simulation");
        context.set_fill_style_color("lightgray");
        context.fill_rect(0f64, 0f64, width, height);

        self.obsticles.for_each(|obs| obs.draw(context));
        self.boids.for_each(|boid| boid.draw(context));
        if self.config.draw_flock_centerpoint {
            self.flocks.iter().for_each(|f| f.avg_point_mass(&self.boids).draw(context, "white"));
        }
    }
}



