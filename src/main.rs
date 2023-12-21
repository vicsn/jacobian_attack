use snarkvm_curves::templates::short_weierstrass_jacobian::Projective;
use snarkvm_curves::traits::{AffineCurve, ProjectiveCurve};
use snarkvm_fields::One;
use snarkvm_curves::ModelParameters;
use snarkvm_curves::bls12_377::Bls12_377G1Parameters;
use snarkvm_fields::Field;
use snarkvm_utilities::TestRng;
use snarkvm_utilities::rand::Uniform;

fn main() {
    let one = <Bls12_377G1Parameters as ModelParameters>::BaseField::one();
    let two = one + one;
    let inverse = two.inverse().unwrap();
    println!("multiplicative inverse test: {:?}", inverse);

    let x_1 = two;
    let y_1 = two;
    let z_1 = two;

    let x_2 = two;
    let y_2 = -two;
    let z_2 = two;

    let mut p1: Projective<Bls12_377G1Parameters> = Projective::new(x_1, y_1, z_1);
    let p2 = Projective::new(x_2, y_2, z_2);

    p1 += p2;
    println!("p1: {:?}", p1);

    let rng = &mut TestRng::default();
    let mut p3: Projective<Bls12_377G1Parameters> = Projective::rand(rng);
    let p4 = Projective::new(p3.x, -p3.y, p3.z);
    p3 += p4;
    println!("p3: {:?}", p3);
}
