#[derive(Debug)]
pub struct Matrix {
    rows: i64,
    cols: i64,
    value: Vec<Vec<i64>>
}

impl Matrix {
    fn new(rows: i64, cols: i64, init: i64) -> Self{
        Self{
            rows,
            cols,
            value: vec![vec![init; cols as usize]; rows as usize]
        }
    }
}
