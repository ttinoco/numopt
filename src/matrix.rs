use std::fmt::{self, Debug};
use num_traits::{Float, NumCast};

pub struct CooMat<N: Float> {
    pub shape: (usize, usize),
    pub row_inds: Vec<usize>,
    pub col_inds: Vec<usize>,
    pub data: Vec<N>,
}

pub struct CooMatIter<'a, N: Float> {
    k: usize,
    mat: &'a CooMat<N>,
}

pub struct CsrMat<N: Float> {
    pub shape: (usize, usize),
    pub indptr: Vec<usize>,
    pub indices: Vec<usize>,
    pub data: Vec<N>,
}

impl<N: Float> CooMat<N> {

    pub fn new(shape: (usize, usize), 
               row_inds: Vec<usize>,
               col_inds: Vec<usize>,
               data: Vec<N>) -> Self {
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
        let data = vec![NumCast::from(0.).unwrap();row_inds.len()];
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
            data: vec![NumCast::from(0.).unwrap();nnz],
        }
    }

    pub fn rows(&self) -> usize { self.shape.0 }
    pub fn cols(&self) -> usize { self.shape.1 }
    pub fn nnz(&self) -> usize { self.row_inds.len() }
    pub fn iter(&self) -> CooMatIter<N> { CooMatIter::new(&self) }

    pub fn to_csr(&self) -> CsrMat<N> {

        let mut indptr: Vec<usize> = vec![0; self.rows()+1];
        let mut indices: Vec<usize> = vec![0; self.nnz()];
        let mut data: Vec<N> = vec![NumCast::from(0.).unwrap(); self.nnz()];

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

impl<N: Float + Debug> Debug for CooMat<N> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CooMat")
         .field("shape", &self.shape)
         .field("row_inds", &self.row_inds)
         .field("col_inds", &self.col_inds)
         .field("data", &self.data)
         .finish()
    }
}

impl<'a, N: Float> CooMatIter<'a, N> {
    fn new(mat: &'a CooMat<N>) -> Self {
        Self {
            k: 0,
            mat: mat,
        }
    }
}

impl<'a, N: Float> Iterator for CooMatIter<'a, N> {
    type Item = (usize, usize, N);
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

impl<N: Float> CsrMat<N> {

    pub fn new(shape: (usize, usize), 
               indptr: Vec<usize>,
               indices: Vec<usize>,
               data: Vec<N>) -> Self {
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
}

