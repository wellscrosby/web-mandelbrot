use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};
use web_mandelbrot::run;

fn main() {
    pollster::block_on(run());

    let mut rng = rand::thread_rng();
    //let normal = Normal::new(2.0, 3.0).unwrap();

    let mut samples = vec![];
    for _ in 0..100 {
        samples.push((rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5)));
    }

    let mut wgpuified = "(".to_owned();
    for sample in &samples {
        wgpuified.push_str(&format!("vec2<f32>({}, {}), ", sample.0, sample.1));
    }
    println!("{:?}", wgpuified);
    println!("{:?}", &samples.len());
}
