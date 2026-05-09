#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Triple {
    pub row: usize,
    pub col: usize,
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseMatrix {
    rows: usize,
    cols: usize,
    triples: Vec<Triple>,
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            triples: Vec::new(),
        }
    }

    pub fn from_array(array: Vec<Vec<i32>>) -> Self {
        let rows = array.len();
        let mut cols = 0;
        if rows > 0 {
            cols = array[0].len();
        }

        let mut matrix = Self::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                if array[i][j] != 0 {
                    matrix.triples.push(Triple {
                        row: i,
                        col: j,
                        value: array[i][j],
                    });
                }
            }
        }

        matrix
    }

    pub fn to_array(&self) -> Vec<Vec<i32>> {
        let mut array = vec![vec![0; self.cols]; self.rows];

        for item in &self.triples {
            array[item.row][item.col] = item.value;
        }

        array
    }

    pub fn triples(&self) -> Vec<Triple> {
        self.triples.clone()
    }

    pub fn add(&self, other: &SparseMatrix) -> Option<SparseMatrix> {
        if self.rows != other.rows || self.cols != other.cols {
            return None;
        }

        let a = self.to_array();
        let b = other.to_array();
        let mut result = vec![vec![0; self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i][j] = a[i][j] + b[i][j];
            }
        }

        Some(SparseMatrix::from_array(result))
    }

    pub fn sub(&self, other: &SparseMatrix) -> Option<SparseMatrix> {
        if self.rows != other.rows || self.cols != other.cols {
            return None;
        }

        let a = self.to_array();
        let b = other.to_array();
        let mut result = vec![vec![0; self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i][j] = a[i][j] - b[i][j];
            }
        }

        Some(SparseMatrix::from_array(result))
    }

    pub fn mul(&self, other: &SparseMatrix) -> Option<SparseMatrix> {
        if self.cols != other.rows {
            return None;
        }

        let a = self.to_array();
        let b = other.to_array();
        let mut result = vec![vec![0; other.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0;
                for k in 0..self.cols {
                    sum += a[i][k] * b[k][j];
                }
                result[i][j] = sum;
            }
        }

        Some(SparseMatrix::from_array(result))
    }
}
