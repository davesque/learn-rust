mod vector;

use vector::{Vector3D, Scalar, S};

fn main() {
    let u = Vector3D::new(1., 2., 3.);
    let v = Vector3D::new(4., 6., 8.);

    println!("{:?}", u);
    println!("{:?}", v);
    println!("{:?}", u + v - u - u);
    println!("{:?}", Scalar::new(2.) * u);
    println!("{:?}", S::S(2.) * u);
    println!("{:?}", u * Scalar::new(2.));
}
