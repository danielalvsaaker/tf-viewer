pub mod backend;
mod sport;
pub use sport::{Sport, SPORTS};

#[derive(Deserialize, Serialize)]
pub struct Activity {
    pub id: String,
    pub gear_id: Option<String>,
    pub session: backend::Session,
    pub record: backend::Record,
    pub lap: Vec<backend::Lap>,
}

use serde::{Deserialize, Serialize};
use uom::si::Unit as _;

#[derive(Serialize, Deserialize)]
pub struct Timestamp(chrono::DateTime<chrono::offset::Local>);

#[derive(Clone, Serialize, Deserialize)]
pub enum ActivityType {
    Cycling,
    Running,
    Other(String),
}

impl Default for ActivityType {
    fn default() -> Self {
        Self::Cycling
    }
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ActivityType::Cycling => "Cycling",
            ActivityType::Running => "Running",
            ActivityType::Other(ref inner) => inner,
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for ActivityType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "cycling" => Self::Cycling,
            "running" => Self::Running,
            _ => Self::Other(s.into()),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Value<'a, T> {
    value: T,
    unit: std::borrow::Cow<'a, str>,
}

impl<'a, T: std::fmt::Display> std::fmt::Display for Value<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

impl<'a, T> Value<'a, T> {
    pub fn new(value: T, unit: &'a str) -> Self {
        Self {
            value,
            unit: std::borrow::Cow::Borrowed(unit),
        }
    }
}

impl<'a> Value<'a, u16> {
    pub fn from_power(power: uom::si::u16::Power) -> Value<'a, u16> {
        Self {
            value: power.get::<uom::si::power::watt>(),
            unit: std::borrow::Cow::Borrowed(uom::si::power::watt::abbreviation()),
        }
    }
}

impl<'a> Value<'a, Vec<Option<u16>>> {
    pub fn from_power_iter(i: impl IntoIterator<Item = Option<uom::si::u16::Power>>) -> Self {
        Self {
            value: i
                .into_iter()
                .map(|x| x.map(|y| y.get::<uom::si::power::watt>()))
                .collect(),
            unit: std::borrow::Cow::Borrowed(uom::si::power::watt::abbreviation()),
        }
    }
}

impl<'a, T: std::ops::AddAssign> std::ops::AddAssign for Value<'a, T> {
    fn add_assign(&mut self, other: Self) {
        assert!(self.unit == other.unit);
        self.value += other.value;
    }
}

#[derive(Deserialize)]
#[serde(tag = "unit", rename_all = "lowercase")]
pub enum Unit {
    Metric,
    Imperial,
}

impl Default for &Unit {
    fn default() -> Self {
        &Unit::Metric
    }
}

macro_rules! convert_fn {
    (
        input_type: $input_type:path;
        quantity: $quantity:ident;
        storage_type: $storage_type:ident;
        convert_type: $convert_type:path;
    ) => {
        ::paste::paste! {
            impl<'a> Value<'a, $storage_type> {
                pub fn [<from_ $quantity>]<L, R>(i: ::uom::si::$storage_type::$input_type, unit: &Unit) -> Self
                where
                    L: ::uom::Conversion<$storage_type, T = $convert_type> + ::uom::si::$quantity::Unit,
                    R: ::uom::Conversion<$storage_type, T = $convert_type> + ::uom::si::$quantity::Unit,
                {
                    match unit {
                        Unit::Metric => Value::new(i.get::<L>(), L::abbreviation()),
                        Unit::Imperial => Value::new(i.get::<R>(), R::abbreviation()),
                    }
                }
            }

            impl<'a> Value<'a, Vec<Option<$storage_type>>> {
                pub fn [<from_ $quantity _iter>]<L, R>(i: Vec<Option<::uom::si::$storage_type::$input_type>>, unit: &Unit) -> Self
                where
                    L: ::uom::Conversion<$storage_type, T = $convert_type> + ::uom::si::$quantity::Unit,
                    R: ::uom::Conversion<$storage_type, T = $convert_type> + ::uom::si::$quantity::Unit,
                {
                    let (value, unit): (Vec<Option<$storage_type>>, _) = match unit {
                        Unit::Metric => (i.iter().map(|x| x.map(|y| y.get::<L>())).collect(), L::abbreviation()),
                        Unit::Imperial => (i.iter().map(|x| x.map(|y| y.get::<R>())).collect(), R::abbreviation()),
                    };

                    Value::new(value, unit)
                }
            }
        }
    }
}

convert_fn! {
    input_type: Length;
    quantity: length;
    storage_type: f64;
    convert_type: f64;
}

convert_fn! {
    input_type: Length;
    quantity: length;
    storage_type: u16;
    convert_type: uom::num::rational::Ratio<u16>;
}

convert_fn! {
    input_type: Length;
    quantity: length;
    storage_type: u32;
    convert_type: uom::num::rational::Ratio<u32>;
}

convert_fn! {
    input_type: Velocity;
    quantity: velocity;
    storage_type: f64;
    convert_type: f64;
}
