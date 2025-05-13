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

    pub fn check_compatible_addition(&self, m2 : Matrix) -> bool {
        if self.rows == m2.rows && self.cols == m2.cols {
            return true
        } else {
            return false
        }
    }

    //checks if matrix multiplication from the right (self is the LHS)
    pub fn check_compatible_multiplication_lhs(&self, m2 : Matrix) -> bool {
        if self.cols == m2.rows {
            return true
        } else {
            return false
        }
    }

    //checks if matrix multiplication from the left (self is the RHS)
    pub fn check_compatible_multiplication_rhs(&self, m2 : Matrix) -> bool {
        if self.rows == m2.cols {
            return true
        } else {
            return false
        }
    }
}