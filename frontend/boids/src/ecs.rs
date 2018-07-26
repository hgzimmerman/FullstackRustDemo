//! The ECS isn't really needed, nor does it do a good job really, I should probably switch to using Vecs.
use stdweb::web::CanvasRenderingContext2d;
use vector::Vector;
use point::Point;
// pub trait Id: Ord + PartialOrd + Increment + Default + Copy {}
// impl <T> Id for T where T: Ord + PartialOrd + Increment + Default + Copy {}

pub trait Id: PartialEq + Increment + Index + Copy + Default {}
impl <T> Id for T where T: PartialEq + Increment + Index + Default + Copy {}


pub trait Increment {
    fn increment(&self) -> Self;
}

pub trait Index {
    fn as_index(&self) -> usize;
    fn from_index(usize) -> Self;
}

pub trait Entity {
    type Id: Id;

    fn get_id(&self) -> Self::Id;
    fn create(id: Self::Id) -> Self;
}

// impl <TOrd for T




pub trait Positioned: Entity {
    fn create_positioned(id: Self::Id, position: Point) -> Self;
    fn get_position(&self) -> Point;
}

pub trait Positionable: Positioned {
    fn mut_position(&mut self) -> &mut Point;
}

pub trait Mobile: Positionable {
    fn create_mobile(id: Self::Id, position: Point, vector: Vector) -> Self;
    fn get_heading(&self) -> Vector;
    fn mut_heading(&mut self) -> &mut Vector;
}


pub trait Drawable: Positioned {
    fn draw(&self, context: &mut CanvasRenderingContext2d);
}


pub struct System<T: Entity> {
    group: Vec<Option<T>>
}

fn find_lowest_available_id<T: Entity>(group: &Vec<Option<T>>) -> Option<T::Id> {
    let len = group.len();
    if len != 0 {
        for i in 0..len - 1 {
            if let None = group[i] {
                return Some(T::Id::from_index(i))
            }
        }
        return None
    } else {
        None
    }
    
}

impl <T: Entity> System<T> {

    pub fn new() -> Self {
        let mut group = Vec::new();
        for _ in 0..1000 {
            group.push(None)
        }
        System {
            group
        }
    }

    pub fn add(&mut self) {
        if let Some(lowest_available_id) = find_lowest_available_id(&self.group) {

            let entity = T::create(lowest_available_id);
            self.group[lowest_available_id.as_index()] = Some(entity);
        }
        
    }

    pub fn clear(&mut self) {
        self.group.iter_mut().for_each(|e| {e.take();});
    }

    pub fn get(&self, id: &T::Id) -> Option<&T> {
        self.group[id.as_index()].as_ref()
    }

    // pub fn get_mut(&mut self, id: &T::Id) -> &mut T {
    //     use std::ops::IndexMut;
    //     self.group.index_mut(id.as_index())
    // }
    
    // pub fn get_mut(&mut self, id: &T::Id) -> Option<&mut T> {
    //     self.group[id.as_index()]
    // }

    pub fn remove(&mut self, id: &T::Id) {
        self.group[id.as_index()] = None;
    }

    pub fn values<F,U>(&self, f:F) -> Vec<U> where F: Fn(&T) -> U {
        self.group
            .iter()
            .filter_map(|x: &Option<T>| {
                match x {
                    Some(y) => Some(y),
                    None => None
                }
            })
            .map(f)
            .collect()
    }

    pub fn for_each<F>(&self, f:F) where F: FnMut(&T) {
        self.group
            .iter()
            .filter_map(|x|{
                match x {
                    Some(y) => Some(y),
                    None => None
                }
            })
            .for_each(f)
    }

    pub fn get_with_ids(&self, ids: &Vec<T::Id>) -> Vec<&T> {
        self.group
            .iter()
            .filter_map(|x| {
                match x {
                    Some(y) => {
                        if ids.contains(&y.get_id()) {
                            Some(y)
                        } else {
                            None
                        }
                    },
                    None => None
                }
            })
            .collect()
    }

    pub fn apply<F>(&mut self, mut f: F) where F: FnMut(&mut T) {
        self.group
        .iter_mut()
        .for_each(|x| {
            match x {
                Some(ref mut y) => f(y),
                None => {}
            }
        } )
    }
    pub fn apply_to_ids<F>(&mut self, ids: &Vec<T::Id>, mut f: F) where F: FnMut(&mut T) {
        self.group
        .iter_mut()
        .filter_map(|x| {
            match x {
                Some(y) => {
                    if ids.contains(&y.get_id()) {
                        Some(y)
                    } else {
                        None
                    }
                },
                None => None
            }
        })
        .for_each(|x| f(x))
    }
}

impl <T: Mobile> System<T> {
    pub fn add_mobile(&mut self, position: Point, vector: Vector) {
        if let Some(lowest_available_id) = find_lowest_available_id(&self.group) {
            let entity = T::create_mobile(lowest_available_id.clone(), position, vector);
            self.group[lowest_available_id.as_index()] = Some(entity);
        }
    }
}

impl <T: Positioned> System<T> {
    pub fn add_positioned(&mut self, position: Point) {
        if let Some(lowest_available_id) = find_lowest_available_id(&self.group) {
            let entity = T::create_positioned(lowest_available_id, position);
            self.group[lowest_available_id.as_index()] = Some(entity);
        }
    }
}