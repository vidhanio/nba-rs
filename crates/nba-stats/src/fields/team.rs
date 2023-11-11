use std::str::FromStr;

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum TeamName {
    #[default]
    AllTeams = 0,

    AtlantaHawks = 1_610_612_737,

    BostonCeltics = 1_610_612_738,

    BrooklynNets = 1_610_612_751,

    CharlotteHornets = 1_610_612_766,

    ChicagoBulls = 1_610_612_741,

    ClevelandCavaliers = 1_610_612_739,

    DallasMavericks = 1_610_612_742,

    DenverNuggets = 1_610_612_743,

    DetroitPistons = 1_610_612_765,

    GoldenStateWarriors = 1_610_612_744,

    HoustonRockets = 1_610_612_745,

    IndianaPacers = 1_610_612_754,

    LaClippers = 1_610_612_746,

    LosAngelesLakers = 1_610_612_747,

    MemphisGrizzlies = 1_610_612_763,

    MiamiHeat = 1_610_612_748,

    MilwaukeeBucks = 1_610_612_749,

    MinnesotaTimberwolves = 1_610_612_750,

    NewOrleansPelicans = 1_610_612_740,

    NewYorkKnicks = 1_610_612_752,

    OklahomaCityThunder = 1_610_612_760,

    OrlandoMagic = 1_610_612_753,

    Philadelphia76ers = 1_610_612_755,

    PhoenixSuns = 1_610_612_756,

    PortlandTrailBlazers = 1_610_612_757,

    SacramentoKings = 1_610_612_758,

    SanAntonioSpurs = 1_610_612_759,

    TorontoRaptors = 1_610_612_761,

    UtahJazz = 1_610_612_762,

    WashingtonWizards = 1_610_612_764,
}

impl TeamName {
    pub const ALL: [Self; 30] = [
        Self::AtlantaHawks,
        Self::BostonCeltics,
        Self::BrooklynNets,
        Self::CharlotteHornets,
        Self::ChicagoBulls,
        Self::ClevelandCavaliers,
        Self::DallasMavericks,
        Self::DenverNuggets,
        Self::DetroitPistons,
        Self::GoldenStateWarriors,
        Self::HoustonRockets,
        Self::IndianaPacers,
        Self::LaClippers,
        Self::LosAngelesLakers,
        Self::MemphisGrizzlies,
        Self::MiamiHeat,
        Self::MilwaukeeBucks,
        Self::MinnesotaTimberwolves,
        Self::NewOrleansPelicans,
        Self::NewYorkKnicks,
        Self::OklahomaCityThunder,
        Self::OrlandoMagic,
        Self::Philadelphia76ers,
        Self::PhoenixSuns,
        Self::PortlandTrailBlazers,
        Self::SacramentoKings,
        Self::SanAntonioSpurs,
        Self::TorontoRaptors,
        Self::UtahJazz,
        Self::WashingtonWizards,
    ];

    #[must_use]
    pub const fn abbreviation(self) -> &'static str {
        match self {
            Self::AllTeams => "ALL",
            Self::AtlantaHawks => "ATL",
            Self::BostonCeltics => "BOS",
            Self::BrooklynNets => "BKN",
            Self::CharlotteHornets => "CHA",
            Self::ChicagoBulls => "CHI",
            Self::ClevelandCavaliers => "CLE",
            Self::DallasMavericks => "DAL",
            Self::DenverNuggets => "DEN",
            Self::DetroitPistons => "DET",
            Self::GoldenStateWarriors => "GSW",
            Self::HoustonRockets => "HOU",
            Self::IndianaPacers => "IND",
            Self::LaClippers => "LAC",
            Self::LosAngelesLakers => "LAL",
            Self::MemphisGrizzlies => "MEM",
            Self::MiamiHeat => "MIA",
            Self::MilwaukeeBucks => "MIL",
            Self::MinnesotaTimberwolves => "MIN",
            Self::NewOrleansPelicans => "NOP",
            Self::NewYorkKnicks => "NYK",
            Self::OklahomaCityThunder => "OKC",
            Self::OrlandoMagic => "ORL",
            Self::Philadelphia76ers => "PHI",
            Self::PhoenixSuns => "PHX",
            Self::PortlandTrailBlazers => "POR",
            Self::SacramentoKings => "SAC",
            Self::SanAntonioSpurs => "SAS",
            Self::TorontoRaptors => "TOR",
            Self::UtahJazz => "UTA",
            Self::WashingtonWizards => "WAS",
        }
    }
}

impl FromStr for TeamName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ATL" => Ok(Self::AtlantaHawks),
            "BOS" => Ok(Self::BostonCeltics),
            "BKN" => Ok(Self::BrooklynNets),
            "CHA" => Ok(Self::CharlotteHornets),
            "CHI" => Ok(Self::ChicagoBulls),
            "CLE" => Ok(Self::ClevelandCavaliers),
            "DAL" => Ok(Self::DallasMavericks),
            "DEN" => Ok(Self::DenverNuggets),
            "DET" => Ok(Self::DetroitPistons),
            "GSW" => Ok(Self::GoldenStateWarriors),
            "HOU" => Ok(Self::HoustonRockets),
            "IND" => Ok(Self::IndianaPacers),
            "LAC" => Ok(Self::LaClippers),
            "LAL" => Ok(Self::LosAngelesLakers),
            "MEM" => Ok(Self::MemphisGrizzlies),
            "MIA" => Ok(Self::MiamiHeat),
            "MIL" => Ok(Self::MilwaukeeBucks),
            "MIN" => Ok(Self::MinnesotaTimberwolves),
            "NOP" => Ok(Self::NewOrleansPelicans),
            "NYK" => Ok(Self::NewYorkKnicks),
            "OKC" => Ok(Self::OklahomaCityThunder),
            "ORL" => Ok(Self::OrlandoMagic),
            "PHI" => Ok(Self::Philadelphia76ers),
            "PHX" => Ok(Self::PhoenixSuns),
            "POR" => Ok(Self::PortlandTrailBlazers),
            "SAC" => Ok(Self::SacramentoKings),
            "SAS" => Ok(Self::SanAntonioSpurs),
            "TOR" => Ok(Self::TorontoRaptors),
            "UTA" => Ok(Self::UtahJazz),
            "WAS" => Ok(Self::WashingtonWizards),
            _ => Err(()),
        }
    }
}
