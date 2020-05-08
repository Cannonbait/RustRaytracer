use std::fs::File;
use std::io::prelude::*;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const DISTANCE: usize = 1;

type Number = u32;
type Area = Vec<Vec<Number>>;

struct Circle {
    pub pos: Point3,
    pub radius: f32,
}
trait Intersectable {
    fn intersects(&self, origin: &Point3, direction: &Point3) -> Option<Number>;
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

#[derive(Debug)]
struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    fn distance(&self, other: &Point3) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

fn main() -> std::io::Result<()> {
    //Create matrix with default background

    let mut render_area = setup_render_matrix();
    //For each pixel, calculate intersection and set colour

    let circle = Circle {
        pos: Point3 {
            x: 3.0,
            y: 5.0,
            z: 6.0,
        },
        radius: 50.0,
    };

    let viewpoint = Point3 {
        x: (WIDTH as f32) / 2f32,
        y: (HEIGHT as f32) / 2f32,
        z: (DISTANCE as f32),
    };

    //For each pixel in render
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            //Calculate ray
            let ray = Point3 {
                x: (x as f32) - viewpoint.x,
                y: (y as f32) - viewpoint.y,
                z: (DISTANCE as f32),
            };
            render_area[x][y] = circle.intersects(&viewpoint, &ray).unwrap_or(0);
        }
    }

    return render_to_file(render_area, "conway");
}

fn setup_render_matrix() -> Area {
    return vec![vec![0; WIDTH]; HEIGHT];
}

fn render_to_file(matrix: Area, name: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.ppm", name))?;
    file.write_all(b"P2\n")?;
    let num_columns = matrix.len();
    let num_rows = matrix[0].len();
    file.write_all(format!("{} {} \n", num_columns, num_rows).as_bytes())?;
    file.write_all(b"15\n")?;

    for row in matrix.iter() {
        let string_row = row
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        file.write_all(format!("{}\n", string_row).as_bytes())?;
    }

    Ok(())
}
