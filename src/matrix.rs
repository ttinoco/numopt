use std::fmt::{self, Debug};

pub struct CooMat {
    shape: (usize, usize),
    row_inds: Vec<usize>,
    col_inds: Vec<usize>,
    data: Vec<f64>,
}

pub struct CooMatIter<'a> {
    k: usize,
    mat: &'a CooMat,
}

pub struct CsrMat {
    shape: (usize, usize),
    indptr: Vec<usize>,
    indices: Vec<usize>,
    data: Vec<f64>,
}

impl CooMat {

    pub fn new(shape: (usize, usize), 
               row_inds: Vec<usize>,
               col_inds: Vec<usize>,
               data: Vec<f64>) -> Self {
        assert_eq!(row_inds.len(), col_inds.len());
        assert_eq!(row_inds.len(), data.len());
        Self {
            shape: shape,
            row_inds: row_inds,
            col_inds: col_inds,
            data: data,
        }
    }

    pub fn from_pattern(shape: (usize, usize), 
                        row_inds: Vec<usize>,
                        col_inds: Vec<usize>) -> Self {
        assert_eq!(row_inds.len(), col_inds.len());
        let data = vec![0.;row_inds.len()];
        Self {
            shape: shape,
            row_inds: row_inds,
            col_inds: col_inds,
            data: data,
        }
    }

    pub fn from_nnz(shape: (usize, usize), nnz: usize) -> Self {
        Self {
            shape: shape,
            row_inds: vec![0;nnz],
            col_inds: vec![0;nnz],
            data: vec![0.;nnz],
        }
    }

    pub fn rows(&self) -> usize { self.shape.0 }
    pub fn cols(&self) -> usize { self.shape.1 }
    pub fn nnz(&self) -> usize { self.row_inds.len() }
    pub fn row_inds(&self) -> &[usize] { &self.row_inds }
    pub fn col_inds(&self) -> &[usize] { &self.col_inds }
    pub fn data(&self) -> &[f64] { &self.data }
    pub fn set_row_ind(&mut self, k: usize, row: usize) -> () { self.row_inds[k] = row }
    pub fn set_col_ind(&mut self, k: usize, row: usize) -> () { self.col_inds[k] = row }
    pub fn set_data(&mut self, k:usize, d: f64) -> () { self.data[k] = d }
    pub fn iter(&self) -> CooMatIter { CooMatIter::new(&self) }

    pub fn to_csr(&self) -> CsrMat {

        let mut indptr: Vec<usize> = vec![0; self.rows()+1];
        let mut indices: Vec<usize> = vec![0; self.nnz()];
        let mut data: Vec<f64> = vec![0.; self.nnz()];

        let mut counter: Vec<usize> = vec![0; self.rows()];

        // Count elements per row
        for row in self.row_inds.iter() {
            counter[*row] += 1;
        }
        
        // Set indptr
        indptr[0] = 0;
        let mut offset: usize = 0;
        for (i, c) in counter.iter().enumerate() {
            indptr[i+1] = offset + c;
            offset += c;
        }
        assert_eq!(indptr[self.rows()], self.nnz());

        // Set indices and data
        let mut k: usize; 
        counter.copy_from_slice(&vec![0; self.rows()]);
        for (row, col, val) in self.iter() {
            k = indptr[row] + counter[row]; 
            indices[k] = col;
            data[k] = val;
            counter[row] += 1;
        }
        
        // Return
        CsrMat {
            shape: self.shape,
            indptr: indptr,
            indices: indices,
            data: data,
        }
    }
}

impl Debug for CooMat {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CooMat")
         .field("shape", &self.shape)
         .field("row_inds", &self.row_inds)
         .field("col_inds", &self.col_inds)
         .field("data", &self.data)
         .finish()
    }
}

impl<'a> CooMatIter<'a> {
    fn new(mat: &'a CooMat) -> Self {
        Self {
            k: 0,
            mat: mat,
        }
    }
}

impl<'a> Iterator for CooMatIter<'a> {
    type Item = (usize, usize, f64);
    fn next(&mut self) -> Option<Self::Item> {
        if self.k < self.mat.nnz() {
            let item = (self.mat.row_inds[self.k],
                    self.mat.col_inds[self.k],
                    self.mat.data[self.k]);
            self.k += 1;
            return Some(item);
        }
        else {
            self.k = 0;
            return None;
        }
    }
}

impl CsrMat {

    pub fn new(shape: (usize, usize), 
               indptr: Vec<usize>,
               indices: Vec<usize>,
               data: Vec<f64>) -> Self {
        assert_eq!(indptr.len(), shape.0+1);
        assert_eq!(indices.len(), data.len());
        assert_eq!(*indptr.last().unwrap(), data.len());
        Self {
            shape: shape,
            indptr: indptr,
            indices: indices,
            data: data,
        }
    }

    pub fn rows(&self) -> usize { self.shape.0 }
    pub fn cols(&self) -> usize { self.shape.1 }
    pub fn nnz(&self) -> usize { self.indices.len() }
    pub fn indptr(&self) -> &[usize] { &self.indptr }
    pub fn indices(&self) -> &[usize] { &self.indices }
    pub fn data(&self) -> &[f64] { &self.data }

    pub fn sum_duplicates(&mut self) -> () {

        let mut colseen: Vec<bool> = vec![false; self.cols()];
        let mut colrow: Vec<usize> = vec![0; self.cols()];
        let mut colnewk: Vec<usize> = vec![0; self.cols()];

        let mut d: f64;
        let mut col: usize;
        let mut new_k: usize = 0;
        let mut new_counter: Vec<usize> = vec![0; self.rows()];
        let mut new_indices: Vec<usize> = Vec::new();
        let mut new_data: Vec<f64> = Vec::new();
        for row in 0..self.rows() {
            for k in self.indptr[row]..self.indptr[row+1] {
                
                col = self.indices[k];
                d = self.data[k];

                // New column in row
                if !colseen[col] || colrow[col] != row {        
                    colnewk[col] = new_k;
                    new_counter[row] += 1;
                    new_indices.push(col);
                    new_data.push(d);
                    new_k += 1;
                }
                
                // Duplicate column in row
                else { 
                    new_data[colnewk[col]] += d;
                }

                // Update
                colseen[col] = true;
                colrow[col] = row;
            }

        }

        let mut offset: usize = 0;
        let mut new_indptr: Vec<usize> = vec![0; self.rows()+1];
        for (row, c) in new_counter.iter().enumerate() {
            new_indptr[row+1] = offset + c;
            offset += c;
        }

        self.indptr = new_indptr;
        self.indices = new_indices;
        self.data = new_data;

        assert_eq!(self.indptr.len(), self.rows()+1);
        assert_eq!(self.indices.len(), self.indptr[self.rows()]);
        assert_eq!(self.indices.len(), self.data.len());
    }
}

impl Debug for CsrMat {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CsrMat")
         .field("shape", &self.shape)
         .field("indptr", &self.indptr)
         .field("indices", &self.indices)
         .field("data", &self.data)
         .finish()
    }
}

