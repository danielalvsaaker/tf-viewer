use serde::{Deserialize, Serialize};

pub const SPORTS: [Sport; 52] = [
    Sport::Generic,
    Sport::Running,
    Sport::Cycling,
    Sport::Transition,
    Sport::FitnessEquipment,
    Sport::Swimming,
    Sport::Basketball,
    Sport::Soccer,
    Sport::Tennis,
    Sport::AmericanFootball,
    Sport::Training,
    Sport::Walking,
    Sport::CrossCountrySkiing,
    Sport::AlpineSkiing,
    Sport::Snowboarding,
    Sport::Rowing,
    Sport::Mountaineering,
    Sport::Hiking,
    Sport::Multisport,
    Sport::Paddling,
    Sport::Flying,
    Sport::EBiking,
    Sport::Motorcycling,
    Sport::Boating,
    Sport::Driving,
    Sport::Golf,
    Sport::HangGliding,
    Sport::HorsebackRiding,
    Sport::Hunting,
    Sport::Fishing,
    Sport::InlineSkating,
    Sport::RockClimbing,
    Sport::Sailing,
    Sport::IceSkating,
    Sport::SkyDiving,
    Sport::Snowshoeing,
    Sport::Snowmobiling,
    Sport::StandUpPaddleboarding,
    Sport::Surfing,
    Sport::Wakeboarding,
    Sport::WaterSkiing,
    Sport::Kayaking,
    Sport::Rafting,
    Sport::Windsurfing,
    Sport::Kitesurfing,
    Sport::Tactical,
    Sport::Jumpmaster,
    Sport::Boxing,
    Sport::FloorClimbing,
    Sport::Diving,
    Sport::All,
    Sport::Unknown,
];

#[derive(Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
pub enum Sport {
    Generic,
    Running,
    Cycling,
    /// Multisport transition
    Transition,
    FitnessEquipment,
    Swimming,
    Basketball,
    Soccer,
    Tennis,
    AmericanFootball,
    Training,
    Walking,
    CrossCountrySkiing,
    AlpineSkiing,
    Snowboarding,
    Rowing,
    Mountaineering,
    Hiking,
    Multisport,
    Paddling,
    Flying,
    EBiking,
    Motorcycling,
    Boating,
    Driving,
    Golf,
    HangGliding,
    HorsebackRiding,
    Hunting,
    Fishing,
    InlineSkating,
    RockClimbing,
    Sailing,
    IceSkating,
    SkyDiving,
    Snowshoeing,
    Snowmobiling,
    StandUpPaddleboarding,
    Surfing,
    Wakeboarding,
    WaterSkiing,
    Kayaking,
    Rafting,
    Windsurfing,
    Kitesurfing,
    Tactical,
    Jumpmaster,
    Boxing,
    FloorClimbing,
    Diving,
    /// All is for goals only to include all sports.
    All,
    Unknown,
}

impl std::str::FromStr for Sport {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "generic" => Sport::Generic,
            "running" => Sport::Running,
            "cycling" => Sport::Cycling,
            "transition" => Sport::Transition,
            "fitness_equipment" => Sport::FitnessEquipment,
            "swimming" => Sport::Swimming,
            "basketball" => Sport::Basketball,
            "soccer" => Sport::Soccer,
            "tennis" => Sport::Tennis,
            "american_football" => Sport::AmericanFootball,
            "training" => Sport::Training,
            "walking" => Sport::Walking,
            "cross_country_skiing" => Sport::CrossCountrySkiing,
            "alpine_skiing" => Sport::AlpineSkiing,
            "snowboarding" => Sport::Snowboarding,
            "rowing" => Sport::Rowing,
            "mountaineering" => Sport::Mountaineering,
            "hiking" => Sport::Hiking,
            "multisport" => Sport::Multisport,
            "paddling" => Sport::Paddling,
            "flying" => Sport::Flying,
            "e_biking" => Sport::EBiking,
            "motorcycling" => Sport::Motorcycling,
            "boating" => Sport::Boating,
            "driving" => Sport::Driving,
            "golf" => Sport::Golf,
            "hang_gliding" => Sport::HangGliding,
            "horseback_riding" => Sport::HorsebackRiding,
            "hunting" => Sport::Hunting,
            "fishing" => Sport::Fishing,
            "inline_skating" => Sport::InlineSkating,
            "rock_climbing" => Sport::RockClimbing,
            "sailing" => Sport::Sailing,
            "ice_skating" => Sport::IceSkating,
            "sky_diving" => Sport::SkyDiving,
            "snowshoeing" => Sport::Snowshoeing,
            "snowmobiling" => Sport::Snowmobiling,
            "stand_up_paddleboarding" => Sport::StandUpPaddleboarding,
            "surfing" => Sport::Surfing,
            "wakeboarding" => Sport::Wakeboarding,
            "water_skiing" => Sport::WaterSkiing,
            "kayaking" => Sport::Kayaking,
            "rafting" => Sport::Rafting,
            "windsurfing" => Sport::Windsurfing,
            "kitesurfing" => Sport::Kitesurfing,
            "tactical" => Sport::Tactical,
            "jumpmaster" => Sport::Jumpmaster,
            "boxing" => Sport::Boxing,
            "floor_climbing" => Sport::FloorClimbing,
            "diving" => Sport::Diving,
            "all" => Sport::All,
            _ => Sport::Unknown,
        })
    }
}

impl std::fmt::Display for Sport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Sport::Generic => write!(f, "Generic"),
            Sport::Running => write!(f, "Running"),
            Sport::Cycling => write!(f, "Cycling"),
            Sport::Transition => write!(f, "Transition"),
            Sport::FitnessEquipment => write!(f, "Fitness equipment"),
            Sport::Swimming => write!(f, "Swimming"),
            Sport::Basketball => write!(f, "Basketball"),
            Sport::Soccer => write!(f, "Soccer"),
            Sport::Tennis => write!(f, "Tennis"),
            Sport::AmericanFootball => write!(f, "American football"),
            Sport::Training => write!(f, "Training"),
            Sport::Walking => write!(f, "Walking"),
            Sport::CrossCountrySkiing => write!(f, "Cross country skiing"),
            Sport::AlpineSkiing => write!(f, "Alpine skiing"),
            Sport::Snowboarding => write!(f, "Snowboarding"),
            Sport::Rowing => write!(f, "Rowing"),
            Sport::Mountaineering => write!(f, "Mountaineering"),
            Sport::Hiking => write!(f, "Hiking"),
            Sport::Multisport => write!(f, "Multisport"),
            Sport::Paddling => write!(f, "Paddling"),
            Sport::Flying => write!(f, "Flying"),
            Sport::EBiking => write!(f, "E-biking"),
            Sport::Motorcycling => write!(f, "Motorcycling"),
            Sport::Boating => write!(f, "Boating"),
            Sport::Driving => write!(f, "Driving"),
            Sport::Golf => write!(f, "Golf"),
            Sport::HangGliding => write!(f, "Hang gliding"),
            Sport::HorsebackRiding => write!(f, "Horseback riding"),
            Sport::Hunting => write!(f, "Hunting"),
            Sport::Fishing => write!(f, "Fishing"),
            Sport::InlineSkating => write!(f, "Inline skating"),
            Sport::RockClimbing => write!(f, "Rock climbing"),
            Sport::Sailing => write!(f, "Sailing"),
            Sport::IceSkating => write!(f, "Ice skating"),
            Sport::SkyDiving => write!(f, "Sky diving"),
            Sport::Snowshoeing => write!(f, "Snowshoeing"),
            Sport::Snowmobiling => write!(f, "Snowmobiling"),
            Sport::StandUpPaddleboarding => write!(f, "Stand-up paddleboarding"),
            Sport::Surfing => write!(f, "Surfing"),
            Sport::Wakeboarding => write!(f, "Wakeboarding"),
            Sport::WaterSkiing => write!(f, "Water skiing"),
            Sport::Kayaking => write!(f, "Kayaking"),
            Sport::Rafting => write!(f, "Rafting"),
            Sport::Windsurfing => write!(f, "Windsurfing"),
            Sport::Kitesurfing => write!(f, "Kitesurfing"),
            Sport::Tactical => write!(f, "Tactical"),
            Sport::Jumpmaster => write!(f, "Jumpmaster"),
            Sport::Boxing => write!(f, "Boxing"),
            Sport::FloorClimbing => write!(f, "Floor climbing"),
            Sport::Diving => write!(f, "Diving"),
            Sport::All => write!(f, "All"),
            Sport::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Default for Sport {
    fn default() -> Self {
        Self::Cycling
    }
}
