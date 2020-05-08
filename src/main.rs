use std::fs::File;
use std::io::prelude::*;

mod point;
mod shape;
use point::*;
use shape::*;

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const FOV: usize = 90;

pub type Number = u32;
pub type Area = Vec<Vec<Number>>;

fn calculate_fov() -> f32 {
    std::cmp::max(WIDTH, HEIGHT) as f32 / 2.0 / ((FOV as f32) / 2.0).tan()
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

    let rectangle = Rectangle {
        pos: Point3 {
            x: 50.0,
            y: 200.0,
            z: 7.0,
        },
        width: 40.0,
        height: 50.0,
    };

    let distance = calculate_fov();

    let objects: Vec<Box<dyn Intersectable>> = vec![Box::new(rectangle), Box::new(circle)];

    let viewpoint = Point3 {
        x: (WIDTH as f32) / 2f32,
        y: (HEIGHT as f32) / 2f32,
        z: distance,
    };

    //For each pixel in render
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            //Calculate ray
            let ray = Point3 {
                x: (x as f32) - viewpoint.x,
                y: (y as f32) - viewpoint.y,
                z: distance,
            };
            for o in objects.iter() {
                let prev = render_area[x][y];
                render_area[x][y] = o.intersects(&viewpoint, &ray).unwrap_or(prev);
            }
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
