use matrix::Matrix;

fn main(){
    let m = Matrix::initialize(2, 2);
    println!("Rows: {}, cols: {}", m.get_rows(), m.get_cols());
    println!("{:?}", m.values);
}