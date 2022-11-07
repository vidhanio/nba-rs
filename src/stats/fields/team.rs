use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum Team {
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
