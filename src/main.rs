// Simulating Mobius Strip
// Using parametric equations

use clap::Parser;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::io::{BufWriter, Write};
use std::{f64::consts::PI, fs::File};

#[derive(Parser)]
#[command(about = "Generates 3D coordinates of points on a Mobius strip", author, version)]
struct Args {
    // Step size for u and theta's range (default: 0.05)
    #[arg(
        short, 
        long, 
        default_value_t = 0.05,
        help = "Controls the density of points generated, smaller values create more detailed output"
    )]
    steps: f64,

    // Saved file
    #[arg(
        short, 
        long, 
        default_value_t = String::from("data/mobius_points.csv"),
        help = "Path to save the generated points as CSV"
    )]
    path: String,
}

// 3D Coordinates
struct MobiusStrip {
    x: f64,
    y: f64,
    z: f64,
}

impl MobiusStrip {
    fn new(step: f64) -> Vec<MobiusStrip> {
        // u belongs in [-1,1]
        let u = Self::range_parallel(-1.0, 1.0, step);
        // theta belongs in [0, 2pi]
        let theta = Self::range_parallel(0.0, 2.0 * PI, step);

        // Process coordinates on u x theta in parallel
        let values: Vec<_> = u
            .into_par_iter()
            .flat_map(|ui| {
                theta.par_iter().map(move |ti| Self {
                    x: Self::cmp_x(&ui, ti),
                    y: Self::cmp_y(&ui, ti),
                    z: Self::cmp_z(&ui, ti),
                })
            })
            .collect();

        return values;
    }

    // fn range(start: f64, stop: f64, step: f64) -> Vec<f64> {
    //     let mut values = Vec::new();

    //     let mut x = start;

    //     while x <= stop {
    //         values.push(x);
    //         x += step
    //     }

    //     return values;
    // }

    fn range_parallel(start: f64, stop: f64, step: f64) -> Vec<f64> {
        // Calculate total elements
        let count = ((stop - start) / step).floor() as usize + 1;

        // Helpful for small step size
        (0..count)
            .into_par_iter()
            .map(|i| start + (i as f64) * step)
            .collect()
    }

    // x coordinate using u and theta
    fn cmp_x(u: &f64, theta: &f64) -> f64 {
        return (1.0 + (u / 2.0) * (theta / 2.0).cos()) * theta.cos();
    }

    // y coordinate using u and theta
    fn cmp_y(u: &f64, theta: &f64) -> f64 {
        return (1.0 + (u / 2.0) * (theta / 2.0).cos()) * theta.sin();
    }

    // z coordinate using u and theta
    fn cmp_z(u: &f64, theta: &f64) -> f64 {
        return (u / 2.0) * (theta / 2.0).sin();
    }
}

fn main() {
    let args = Args::parse();
    let steps = args.steps;
    let path = args.path;

    let start = std::time::Instant::now();

    let points = MobiusStrip::new(steps);
    let total_points = points.len();

    let file = File::create(&path).expect("failed to create file");

    // Use buffer for fast write ops
    let mut writer = BufWriter::new(file);
    writeln!(writer, "x,y,z").expect("failed to write header");

    for point in &points {
        writeln!(writer, "{:.6},{:.6},{:.6}", point.x, point.y, point.z)
            .expect("failed to write point");
    }

    println!("\nMOBIUS STRIP SIMULATION");
    println!("Step size     : {}", steps);
    println!("Points        : {}", total_points);
    println!("Output file   : {}", path);
    println!("Time taken    : {:.2?}\n", start.elapsed());
}
