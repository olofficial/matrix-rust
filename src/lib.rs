pub struct Matrix{
    cols : usize,
    rows : usize,
    pub values : Vec<f64>,
}

impl Matrix {
    //initializing a zero matrix
    pub fn initialize(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            values : vec![0.0; rows * cols],
        }
    }

    //getter functions for the number of rows and columns in the matrix
    pub fn get_rows(&self) -> usize {
        return self.rows;
    }

    pub fn get_cols(&self) -> usize {
        return self.cols;
    }

    
}