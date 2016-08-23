
extern crate ndarray;
extern crate num;

use ndarray::prelude::*;

use num::Zero;
use std::clone::Clone;

pub fn diag<A: Clone + Zero>(d: &Array<A, Ix>) -> Array<A, (Ix, Ix)> {
    diag_k(d, 0)
}

pub fn diag_k<A: Clone + Zero>(d: &Array<A, Ix>, k: usize) -> Array<A, (Ix, Ix)> {
    let l = d.shape()[0];
    let s = l + k;
    let mut a = Array::zeros((s, s));
    for i in 0..l {
        a[(i, i + k)] = d[i].clone();
    }
    a
}
