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

    fn multiply(&mut self, num: i64){
        for item in self.value.iter_mut(){
            for value in item.iter_mut(){
                *value *= num;
            }
        }
    }

    fn add(&mut self, num: i64){
        for item in self.value.iter_mut(){
            for value in item.iter_mut(){
                *value += num;
            }
        }
    }
}

fn main() {
    let mut m: Matrix = Matrix::new(4,5,-1);
    m.multiply(5);
    m.add(6);
    m.add(23);
    m.multiply(11);
    println!("{:?}",m);
}
