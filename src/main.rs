use ndarray::Array2;
use sph::systems::particle::ParticleSystem;
use sph::state::{BasicState, State};
// use nalgebra::{Point2, Vector2};
// use sph::interpolate::density::{self, density};
// use sph::kernels::cubic_spline::CubicSpline;
// use sph::particle::{self, Particle};
// use sph::traits::kernel::Kernel;

// use rand::Rng;

// pub fn main() {
//     let x1 = Point::<f32, 2>::new(0., 0.);
//     let x2 = Point::<f32, 2>::new(-1.0, 0.0);
//     let kernel = CubicSpline::<f32, 2>::new(1.0);
//     println!("Norm of (x1 - x2): {}", kernel.apply(x1, x2));
//
//     let x1 = Point::<f64, 2>::new(0., 0.);
//     let x2 = Point::<f64, 2>::new(-1.0, 0.0);
//     let kernel = CubicSpline::<f64, 2>::new(2.0);
//     println!("Norm of (x1 - x2): {}", kernel.apply(x1, x2));
//
//     let x1 = Point::<f32, 3>::new(0., 0., 0.0);
//     let x2 = Point::<f32, 3>::new(-1.0, 0.0, 0.);
//     let kernel = CubicSpline::<f32, 3>::new(1.0);
//     println!("Norm of (x1 - x2): {}", kernel.apply(x1, x2));
//
//     let x1 = Point::<f64, 3>::new(0., 0., 0.);
//     let x2 = Point::<f64, 3>::new(-1.0, 0.0, 0.);
//     let kernel = CubicSpline::<f64, 3>::new(2.0);
//     println!("Norm of (x1 - x2): {}", kernel.apply(x1, x2));
// }

pub fn main() {
    fn calc(i: usize, j: usize) -> f32 {
        let (i, j): (f32, f32) = (i as f32, j as f32);
        i + j / 10.0
    }

    let data = Array2::<f32>::from_shape_fn((8, 6), |(i, j)| calc(i, j));

    let system = ParticleSystem::<BasicState<f32>>::from_array(3, data);

    let positions = system.state.positions();
    let (m, n) = positions.dim();
    println!("Positions:\t{m} x {n}");
    println!("{positions}");

    let velocities = system.state.velocities();
    let (m, n) = velocities.dim();
    println!("Velocities:\t{m} x {n}");
    println!("{velocities}");

    let masses = system.state.masses();
    let m = masses.dim();
    println!("Masses:\t\t{m}");
    println!("{masses}");

    let densities = system.state.densities();
    let m = densities.dim();
    println!("Densities:\t{m}");
    println!("{densities}");

    let position = system.state.position(2);
    let m = position.dim();
    println!("Position:\t{m}");
    println!("{position}");

    let velocity = system.state.velocity(2);
    let m = velocity.dim();
    println!("Velocity:\t{m}");
    println!("{velocity}");

    let mass = system.state.mass(2);
    println!("Mass:\t\t{mass}");

    let density = system.state.density(2);
    println!("Density:\t{density}");
}
