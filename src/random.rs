use lazy_static::lazy_static;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::Mutex;

const SEED: u64 = 0;

lazy_static! {
    static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::seed_from_u64(SEED));
}

pub fn gen<T>() -> T
where
    Standard: Distribution<T>,
{
    RNG.lock().unwrap().gen::<T>()
}

pub fn reset(seed: u64) {
    *RNG.lock().unwrap() = StdRng::seed_from_u64(seed);
}
