mod vector;

use vector::{Vector3D, Scalar};

fn main() {
    let u = Vector3D::new(1., 2., 3.);
    let v = Vector3D::new(1., 1., 1.);

    println!("{:?}", u);
    println!("{:?}", v);

    println!("{:?}", u + v - u - u);

    println!("{:?}", u * Scalar::new(2.));
    println!("{:?}", Scalar::new(2.) * u);

    println!("{:?}", u * 2.);
    println!("{:?}", 2. * u);
}
