
// 1. import ndarray package
#[macro_use(s)]
extern crate ndarray;

use ndarray::Array;

fn main() {
    // 3. Create a null vector of size 10
    let a = Array::<i32, _>::zeros(10);
    println!("3: {:?}", &a);

    // 5. Create a null vector of size 10 but the fiffth value which is 1
    let mut a = Array::<i32, _>::zeros(10);
    a[4] = 1;
    println!("5: {:?}", &a);

    // 6. Create a vector with values ranging from 10 to 49
    // XXX: restricted to float
    let a = Array::range(10., 50., 1.);
    println!("6: {:?}", &a);

    // 7. Reverse a vector
    let a = Array::range(0., 50., 1.);
    let b = a.slice(s![..;-1]);
    println!("7: {:?}", &b);
}
