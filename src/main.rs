use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    //DO RENDER
    //SAVE AS PPM

    return render_to_file(vec![vec![1, 0, 0], vec![0, 1, 1], vec![1, 1, 0]], "conway");
}

fn render_to_file(matrix: Vec<Vec<i32>>, name: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.ppm", name))?;
    file.write_all(b"P1\n")?;
    let num_columns = matrix.len();
    let num_rows = matrix[0].len();
    file.write_all(format!("{} {} \n", num_columns, num_rows).as_bytes())?;

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
