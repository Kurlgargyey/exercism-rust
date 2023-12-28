#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        seconds_to_year(d.seconds) / Self::ORBITAL_PERIOD
    }
}

macro_rules! impl_planet {
    (
        $(($p:ident, $op:literal)),+
    ) => {
        $(
            pub struct $p;
            impl Planet for $p {
            const ORBITAL_PERIOD: f64 = $op;
        })+
    };
}

impl_planet![
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
];

fn seconds_to_year(seconds: u64) -> f64 {
    seconds as f64 / 31557600.0
}
