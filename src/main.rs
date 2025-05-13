use matrix::Matrix;

fn main(){
    let m1 = Matrix::initialize(2, 2);
    let m2 = Matrix::initialize(2, 2);

    let booly = m1.check_compatible_addition(m2);
    println!("Rows: {}, cols: {}", m1.get_rows(), m1.get_cols());
    println!("{:?}", m1.values);
    println!("{}", booly);
}