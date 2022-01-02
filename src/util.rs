use rand::{thread_rng, Rng};
use rand::distributions::OpenClosed01;

pub fn random_double() -> f64 {
    let val: f64 = thread_rng().sample(OpenClosed01);

    1.0 - val
}
