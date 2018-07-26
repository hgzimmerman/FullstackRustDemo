use boid::BoidId;
use boid::Boid;
// use simulation::Simulation;
use ecs::System;
use ecs::Positioned;
use ecs::Entity;
use ecs::Mobile;
use point::Point;
use vector::Vector;



#[derive(Debug)]
pub struct Flock {
    pub boids: Vec<BoidId>
}

impl Flock {

    pub fn avg_point_mass(&self, boids: &System<Boid>) -> Point {
        let flock_boids = boids.get_with_ids(&self.boids);
        let len = flock_boids.len() as f64;
        let total_position = flock_boids.iter().fold(Point::default(), |sum, current| sum + current.get_position() );
        Point {
            x: total_position.x / len,
            y: total_position.y / len
        }
    }

    pub fn avg_heading(&self, boids: &System<Boid>) -> Vector {
        let flock_boids = boids.get_with_ids(&self.boids);
        let len = flock_boids.len() as f64;
        let total_heading = flock_boids.iter().fold(Vector::default(), |sum, current| sum + current.get_heading() );
        Vector {
            x: total_heading.x / len,
            y: total_heading.y / len
        }
    }

    pub fn create_flocks_from_boids(boids: &System<Boid>, flocking_distance: f64) -> Vec<Flock> {
        let mut flocking_data: Vec<(BoidId, Point)> = boids.values(|boid| (boid.get_id(), boid.get_position()));

        let mut flocks: Vec<Vec<(BoidId, Point)>> = vec!();
        while let Some(data) = flocking_data.pop() {
            let (id, position) = data;

            let mut has_been_added = false;
            
            // Try to add into an existing "flock" first
            for flock in flocks.iter_mut() {
                let mut should_add: bool = false;

                for b in flock.iter() {
                    if position.distance(&b.1) < flocking_distance {
                        should_add = true;
                        break;
                    } else {
                        // warn!("didn't add to flock because distance was too big: {}", position.distance(&b.1));
                    }
                }

                if should_add {
                    flock.push((id, position));
                    has_been_added = true;
                    break;
                }
            }

            // if it hasn't been added to an existing flock, create a flock of one.
            if !has_been_added {
                flocks.push(vec![(id, position)])
            }
        }
        // cull the flocks so that they have a number greater than 1 boid
        let flocks: Vec<Flock> = flocks.into_iter().filter_map(|flock| {
            if flock.len() <= 1 {
                None
            } else {
                Some(Flock {
                    boids: flock.into_iter().map(|x|x.0).collect()
                })
            }
        })
        .collect();
        flocks
    }
}