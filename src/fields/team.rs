//! Team-related fields.

macro_rules! teams {
    {
        ($abbri:ident, $namei:ident) => {
            $($team:ident => ($abbr:literal, $name:literal) => ($($div:ident, $conf:ident)?)),* $(,)?
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

        impl $abbri {
            #[must_use]
            pub const fn abbreviation(self) -> &'static str {
                match self {
                    $(
                        Self::$team => $abbr,
                    )*
                }
            }

            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(
                        Self::$team => $name,
                    )*
                }
            }

            #[must_use]
            pub const fn division(self) -> Option<$crate::fields::Division> {
                match self {
                    $(
                        Self::$team => divconf!(@DIV $($div)?),
                    )*
                }
            }

            #[must_use]
            pub const fn conference(self) -> Option<$crate::fields::Conference> {
                match self {
                    $(
                        Self::$team => divconf!(@CONF $($conf)?),
                    )*
                }
            }
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

        impl $namei {
            #[must_use]
            pub const fn abbreviation(self) -> &'static str {
                match self {
                    $(
                        Self::$team => $abbr,
                    )*
                }
            }

            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(
                        Self::$team => $name,
                    )*
                }
            }

            #[must_use]
            pub const fn division(self) -> Option<$crate::fields::Division> {
                match self {
                    $(
                        Self::$team => divconf!(@DIV $($div)?),
                    )*
                }
            }

            #[must_use]
            pub const fn conference(self) -> Option<$crate::fields::Conference> {
                match self {
                    $(
                        Self::$team => divconf!(@CONF $($conf)?),
                    )*
                }
            }
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

macro_rules! divconf {
    (@DIV $div:ident) => {
        Some($crate::fields::Division::$div)
    };
    (@CONF $conf:ident) => {
        Some($crate::fields::Conference::$conf)
    };
    (@DIV) => {
        None
    };
    (@CONF) => {
        None
    };
}

teams! {
    (NbaTeamAbbreviation, NbaTeamName) => {
        Total => ("TOT", "Total") => (),
        AtlantaHawks => ("ATL", "Atlanta Hawks") => (Southeast, East),
        BostonCeltics => ("BOS", "Boston Celtics") => (Atlantic, East),
        BrooklynNets => ("BRK", "Brooklyn Nets") => (Atlantic, East),
        CharlotteHornets => ("CHO", "Charlotte Hornets") => (Southeast, East),
        ChicagoBulls => ("CHI", "Chicago Bulls") => (Central, East),
        ClevelandCavaliers => ("CLE", "Cleveland Cavaliers") => (Central, East),
        DallasMavericks => ("DAL", "Dallas Mavericks") => (Southwest, West),
        DenverNuggets => ("DEN", "Denver Nuggets") => (Northwest, West),
        DetroitPistons => ("DET", "Detroit Pistons") => (Central, East),
        GoldenStateWarriors => ("GSW", "Golden State Warriors") => (Pacific, West),
        HoustonRockets => ("HOU", "Houston Rockets") => (Southwest, West),
        IndianaPacers => ("IND", "Indiana Pacers") => (Central, East),
        LosAngelesClippers => ("LAC", "Los Angeles Clippers") => (Pacific, West),
        LosAngelesLakers => ("LAL", "Los Angeles Lakers") => (Pacific, West),
        MemphisGrizzlies => ("MEM", "Memphis Grizzlies") => (Southwest, West),
        MiamiHeat => ("MIA", "Miami Heat") => (Southeast, East),
        MilwaukeeBucks => ("MIL", "Milwaukee Bucks") => (Central, East),
        MinnesotaTimberwolves => ("MIN", "Minnesota Timberwolves") => (Northwest, West),
        NewOrleansPelicans => ("NOP", "New Orleans Pelicans") => (Southwest, West),
        NewYorkKnicks => ("NYK", "New York Knicks") => (Atlantic, East),
        OklahomaCityThunder => ("OKC", "Oklahoma City Thunder") => (Northwest, West),
        OrlandoMagic => ("ORL", "Orlando Magic") => (Southeast, East),
        Philadelphia76ers => ("PHI", "Philadelphia 76ers") => (Atlantic, East),
        PhoenixSuns => ("PHX", "Phoenix Suns") => (Pacific, West),
        PortlandTrailBlazers => ("POR", "Portland Trail Blazers") => (Northwest, West),
        SacramentoKings => ("SAC", "Sacramento Kings") => (Pacific, West),
        SanAntonioSpurs => ("SAS", "San Antonio Spurs") => (Southwest, West),
        TorontoRaptors => ("TOR", "Toronto Raptors") => (Atlantic, East),
        UtahJazz => ("UTA", "Utah Jazz") => (Northwest, West),
        WashingtonWizards => ("WAS", "Washington Wizards") => (Southeast, East),
    }
}
