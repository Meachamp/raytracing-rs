use rand::{thread_rng, Rng, distributions::Uniform};

pub fn random_double() -> f64 {
    thread_rng().gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let between = Uniform::new(min, max);
    thread_rng().sample(between)
}
