//! Team-related fields.

macro_rules! teams {
    {
        ($abbri:ident, $namei:ident) => {
            $($team:ident => ($abbr:literal, $name:literal)),* $(,)?
        }
    } => {
        #[derive(Clone, Copy, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub enum $abbri {
            #[default]
            $(
                #[serde(rename = $abbr)]
                $team,
            )*
        }

        impl ::std::fmt::Display for $abbri {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        Self::$team => write!(f, $abbr),
                    )*
                }
            }
        }

        #[derive(Clone, Copy, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub enum $namei {
            #[default]
            $(
                #[serde(rename = $name)]
                $team,
            )*
        }

        impl ::std::fmt::Display for $namei {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        Self::$team => write!(f, $name),
                    )*
                }
            }
        }

        impl From<$abbri> for $namei {
            fn from(team: $abbri) -> Self {
                match team {
                    $(
                        $abbri::$team => Self::$team,
                    )*
                }
            }
        }

        impl From<$namei> for $abbri {
            fn from(team: $namei) -> Self {
                match team {
                    $(
                        $namei::$team => Self::$team,
                    )*
                }
            }
        }
    };
}

teams! {
    (NbaTeamAbbreviation, NbaTeamName) => {
        Total => ("TOT", "Total"),
        AtlantaHawks => ("ATL", "Atlanta Hawks"),
        BostonCeltics => ("BOS", "Boston Celtics"),
        BrooklynNets => ("BRK", "Brooklyn Nets"),
        CharlotteHornets => ("CHO", "Charlotte Hornets"),
        ChicagoBulls => ("CHI", "Chicago Bulls"),
        ClevelandCavaliers => ("CLE", "Cleveland Cavaliers"),
        DallasMavericks => ("DAL", "Dallas Mavericks"),
        DenverNuggets => ("DEN", "Denver Nuggets"),
        DetroitPistons => ("DET", "Detroit Pistons"),
        GoldenStateWarriors => ("GSW", "Golden State Warriors"),
        HoustonRockets => ("HOU", "Houston Rockets"),
        IndianaPacers => ("IND", "Indiana Pacers"),
        LosAngelesClippers => ("LAC", "Los Angeles Clippers"),
        LosAngelesLakers => ("LAL", "Los Angeles Lakers"),
        MemphisGrizzlies => ("MEM", "Memphis Grizzlies"),
        MiamiHeat => ("MIA", "Miami Heat"),
        MilwaukeeBucks => ("MIL", "Milwaukee Bucks"),
        MinnesotaTimberwolves => ("MIN", "Minnesota Timberwolves"),
        NewOrleansPelicans => ("NOP", "New Orleans Pelicans"),
        NewYorkKnicks => ("NYK", "New York Knicks"),
        OklahomaCityThunder => ("OKC", "Oklahoma City Thunder"),
        OrlandoMagic => ("ORL", "Orlando Magic"),
        Philadelphia76ers => ("PHI", "Philadelphia 76ers"),
        PhoenixSuns => ("PHX", "Phoenix Suns"),
        PortlandTrailBlazers => ("POR", "Portland Trail Blazers"),
        SacramentoKings => ("SAC", "Sacramento Kings"),
        SanAntonioSpurs => ("SAS", "San Antonio Spurs"),
        TorontoRaptors => ("TOR", "Toronto Raptors"),
        UtahJazz => ("UTA", "Utah Jazz"),
        WashingtonWizards => ("WAS", "Washington Wizards"),
    }
}

teams! {
    (WnbaTeamAbbreviation, WnbaTeamName) => {
        Total => ("TOT", "Total"),
        AtlantaDream => ("ATL", "Atlanta Dream"),
        ChicagoSky => ("CHI", "Chicago Sky"),
        ConnecticutSun => ("CONN", "Connecticut Sun"),
        DallasWings => ("DAL", "Dallas Wings"),
        IndianaFever => ("IND", "Indiana Fever"),
        LasVegasAces => ("LV", "Las Vegas Aces"),
        LosAngelesSparks => ("LA", "Los Angeles Sparks"),
        MinnesotaLynx => ("MIN", "Minnesota Lynx"),
        NewYorkLiberty => ("NY", "New York Liberty"),
        PhoenixMercury => ("PHX", "Phoenix Mercury"),
        SeattleStorm => ("SEA", "Seattle Storm"),
        WashingtonMystics => ("WSH", "Washington Mystics"),
    }
}
