// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    s: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.s as f64 / Self::ORBITAL_PERIOD
    }
}

macro_rules! define_planet {
    ($name: ident, $earth_orbital_ratio: literal) => {
        pub struct $name;

        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = 31557600.0 * $earth_orbital_ratio as f64;
        }
    };
}

define_planet!(Mercury, 0.2408467);
define_planet!(Venus, 0.61519726);
define_planet!(Earth, 1.0);
define_planet!(Mars, 1.8808158);
define_planet!(Jupiter, 11.862615);
define_planet!(Saturn, 29.447498);
define_planet!(Uranus, 84.0168466);
define_planet!(Neptune, 164.79132);
