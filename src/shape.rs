use super::point::*;
use super::*;
pub struct Rectangle {
    pub pos: Point3,
    pub width: f32,
    pub height: f32,
}
pub struct Circle {
    pub pos: Point3,
    pub radius: f32,
}
pub trait Intersectable {
    fn intersects(&self, origin: &Point3, direction: &Point3) -> Option<Number>;
}

impl Intersectable for Rectangle {
    fn intersects(&self, origin: &Point3, direction: &Point3) -> Option<Number> {
        let distance = self.pos.z - origin.z;
        let multiple = distance / direction.z;

        let hit_x = direction.x * multiple;
        let hit_y = direction.y * multiple;

        let hit = self.pos.x < hit_x
            && self.pos.x + self.width > hit_x
            && self.pos.y < hit_y
            && self.pos.y + self.height > hit_y;

        if hit {
            Some(15)
        } else {
            None
        }
    }
}

impl Intersectable for Circle {
    fn intersects(&self, origin: &Point3, direction: &Point3) -> Option<Number> {
        let distance = self.pos.z - origin.z;
        let multiple = distance / direction.z;

        let hit_x = direction.x * multiple;
        let hit_y = direction.y * multiple;

        let hit = self.pos.distance(&Point3 {
            x: hit_x,
            y: hit_y,
            z: self.pos.z,
        }) < self.radius;

        if hit {
            Some(15)
        } else {
            None
        }
    }
}
