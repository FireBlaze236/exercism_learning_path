// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

const SECONDS_IN_EARTH_YEAR: f64 = 24.0 * 3600.0 * 365.25;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_Planet {
    ($t:ty, $e:expr) => {
        impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.seconds / (SECONDS_IN_EARTH_YEAR * $e)
            }
        }
    };
}

impl_Planet!(Mercury, 0.2408467);
impl_Planet!(Venus, 0.61519726);
impl_Planet!(Earth, 1.0);
impl_Planet!(Mars, 1.8808158);
impl_Planet!(Jupiter, 11.862615);
impl_Planet!(Saturn, 29.447498);
impl_Planet!(Uranus, 84.016846);
impl_Planet!(Neptune, 164.79132);
