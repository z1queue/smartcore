pub mod naive;
pub mod qr;
pub mod svd;
pub mod evd;
pub mod ndarray_bindings;

use std::ops::Range;
use std::fmt::Debug;
use svd::SVDDecomposableMatrix;
use evd::EVDDecomposableMatrix;
use qr::QRDecomposableMatrix;

pub trait BaseMatrix: Clone + Debug {  

    type RowVector: Clone + Debug;    

    fn from_row_vector(vec: Self::RowVector) -> Self;

    fn to_row_vector(self) -> Self::RowVector;

    fn get(&self, row: usize, col: usize) -> f64; 

    fn get_row_as_vec(&self, row: usize) -> Vec<f64>;

    fn get_col_as_vec(&self, col: usize) -> Vec<f64>;    

    fn set(&mut self, row: usize, col: usize, x: f64);         

    fn eye(size: usize) -> Self;

    fn zeros(nrows: usize, ncols: usize) -> Self;

    fn ones(nrows: usize, ncols: usize) -> Self;

    fn to_raw_vector(&self) -> Vec<f64>;

    fn fill(nrows: usize, ncols: usize, value: f64) -> Self;

    fn shape(&self) -> (usize, usize);    

    fn v_stack(&self, other: &Self) -> Self;

    fn h_stack(&self, other: &Self) -> Self;

    fn dot(&self, other: &Self) -> Self;    

    fn vector_dot(&self, other: &Self) -> f64;     

    fn slice(&self, rows: Range<usize>, cols: Range<usize>) -> Self;    

    fn approximate_eq(&self, other: &Self, error: f64) -> bool;

    fn add_mut(&mut self, other: &Self) -> &Self;

    fn sub_mut(&mut self, other: &Self) -> &Self;

    fn mul_mut(&mut self, other: &Self) -> &Self;

    fn div_mut(&mut self, other: &Self) -> &Self;

    fn div_element_mut(&mut self, row: usize, col: usize, x: f64);

    fn mul_element_mut(&mut self, row: usize, col: usize, x: f64);

    fn add_element_mut(&mut self, row: usize, col: usize, x: f64);

    fn sub_element_mut(&mut self, row: usize, col: usize, x: f64);

    fn add(&self, other: &Self) -> Self {
        let mut r = self.clone();
        r.add_mut(other);
        r
    }

    fn sub(&self, other: &Self) -> Self {
        let mut r = self.clone();
        r.sub_mut(other);
        r
    }

    fn mul(&self, other: &Self) -> Self {
        let mut r = self.clone();
        r.mul_mut(other);
        r
    }

    fn div(&self, other: &Self) -> Self {
        let mut r = self.clone();
        r.div_mut(other);
        r
    }

    fn add_scalar_mut(&mut self, scalar: f64) -> &Self;

    fn sub_scalar_mut(&mut self, scalar: f64) -> &Self;

    fn mul_scalar_mut(&mut self, scalar: f64) -> &Self;

    fn div_scalar_mut(&mut self, scalar: f64) -> &Self;

    fn add_scalar(&self, scalar: f64) -> Self{
        let mut r = self.clone();
        r.add_scalar_mut(scalar);
        r
    }

    fn sub_scalar(&self, scalar: f64) -> Self{
        let mut r = self.clone();
        r.sub_scalar_mut(scalar);
        r
    }

    fn mul_scalar(&self, scalar: f64) -> Self{
        let mut r = self.clone();
        r.mul_scalar_mut(scalar);
        r
    }

    fn div_scalar(&self, scalar: f64) -> Self{
        let mut r = self.clone();
        r.div_scalar_mut(scalar);
        r
    }

    fn transpose(&self) -> Self;    

    fn rand(nrows: usize, ncols: usize) -> Self;

    fn norm2(&self) -> f64;

    fn norm(&self, p:f64) -> f64;

    fn column_mean(&self) -> Vec<f64>;

    fn negative_mut(&mut self);

    fn negative(&self) -> Self {
        let mut result = self.clone();
        result.negative_mut();
        result
    }    

    fn reshape(&self, nrows: usize, ncols: usize) -> Self;

    fn copy_from(&mut self, other: &Self);

    fn abs_mut(&mut self) -> &Self;

    fn abs(&self) -> Self {
        let mut result = self.clone();
        result.abs_mut();
        result
    }

    fn sum(&self) -> f64;

    fn max_diff(&self, other: &Self) -> f64;   
    
    fn softmax_mut(&mut self); 

    fn pow_mut(&mut self, p: f64) -> &Self;

    fn pow(&mut self, p: f64) -> Self {
        let mut result = self.clone();
        result.pow_mut(p);
        result
    }

    fn argmax(&self) -> Vec<usize>; 
    
    fn unique(&self) -> Vec<f64>;    

}

pub trait Matrix: BaseMatrix + SVDDecomposableMatrix + EVDDecomposableMatrix + QRDecomposableMatrix {}

pub fn row_iter<M: Matrix>(m: &M) -> RowIter<M> {
    RowIter{
        m: m,
        pos: 0,
        max_pos: m.shape().0
    }
}

pub struct RowIter<'a, M: Matrix> {
    m: &'a M,
    pos: usize,
    max_pos: usize
}

impl<'a, M: Matrix> Iterator for RowIter<'a, M> {

    type Item = Vec<f64>;

    fn next(&mut self) -> Option<Vec<f64>> {
        let res;
        if self.pos < self.max_pos {
            res = Some(self.m.get_row_as_vec(self.pos))
        } else {
            res = None
        }
        self.pos += 1;
        res
    }

}