use rand::Rng;

pub fn rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
