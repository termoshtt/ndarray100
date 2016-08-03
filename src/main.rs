
// 1. import ndarray package
#[macro_use(s)]
extern crate ndarray;

extern crate rand;
extern crate ndarray_rand;

use ndarray::Array;
use rand::distributions::*;
use ndarray_rand::RandomExt;

fn main() {
    // 3. Create a null vector of size 10
    let a = Array::<f64, _>::zeros(10);
    println!("3: {:?}", &a);

    // 5. Create a null vector of size 10 but the fiffth value which is 1
    let mut a = Array::<f64, _>::zeros(10);
    a[4] = 1.0;
    println!("5: {:?}", &a);

    // 6. Create a vector with values ranging from 10 to 49
    let a = Array::range(10., 50., 1.);
    println!("6: {:?}", &a);

    // 7. Reverse a vector
    let a = Array::range(0., 50., 1.);
    let b = a.slice(s![..;-1]);
    println!("7: {:?}", &b);

    // 8. Create a 3x3 matrix with values ranging from 0 to 8
    let a = Array::range(0., 9., 1.).into_shape((3, 3)).unwrap();
    println!("8: {:?}", &a);

    // 9. Find indices of non-zero elements
    // TODO

    // 10. Create a 3x3 identity matrix
    let a = Array::<f64, _>::eye(3);
    println!("10: {:?}", &a);

    // 11. Create a 3x3x3 array with random values
    let r_dist = Range::new(0., 1.);
    let a = Array::<f64, _>::random((3, 3, 3), r_dist);
    println!("11: {:?}", &a);

    // 12. Create a 10x10 array with random values and find min/max
    // See http://qiita.com/l1048576/items/343ca40a03c3b86b67cb (Japanese)
    let r_dist = Range::new(0., 1.);
    let a = Array::<f64, _>::random((10, 10), r_dist);
    let a_max = a.iter().fold(0.0 / 0.0, |m, v| v.max(m));
    let a_min = a.iter().fold(0.0 / 0.0, |m, v| v.min(m));
    println!("12: max={:?}, min={:?}", a_max, a_min);
}
