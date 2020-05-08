use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    //DO RENDER
    //SAVE AS PPM

    return render(vec![
        vec![1, 0, 0],
        vec![0, 1, 1],
        vec![1, 1, 0]
    ], "conway");
}


fn render(matrix: Vec<Vec<i32>>, name: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.ppm", name))?;
    file.write_all(b"Hello, world!")?;
    Ok(())
 }

//  P1
// # This is an example bitmap of the letter "J"
// 6 10
// 0 0 0 0 1 0
// 0 0 0 0 1 0
// 0 0 0 0 1 0
// 0 0 0 0 1 0
// 0 0 0 0 1 0
// 0 0 0 0 1 0
// 1 0 0 0 1 0
// 0 1 1 1 0 0
// 0 0 0 0 0 0
// 0 0 0 0 0 0