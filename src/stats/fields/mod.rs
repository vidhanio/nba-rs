//! All possible fields which may be used in the request or response of a
//! [`crate::stats::Endpoint`].

#![allow(missing_docs)]

macro_rules! reexport {
    {
        $($name:ident);* $(;)?
    } => {
        $(
            mod $name;
            pub use $name::*;
        )*
    };
}

reexport! {
    ahead_or_behind;
    clutch_time;
    college;
    conference;
    country;
    division;
    draft_pick;
    draft_year;
    experience;
    half;
    height;
    last_n_games;
    league_id;
    location;
    measure_type;
    month;
    outcome;
    per_mode;
    player_or_team;
    playoff_round;
    play_type;
    point_diff;
    position;
    quarter;
    scope;
    season;
    season_segment;
    season_type;
    shot_clock_range;
    starter_or_bench;
    stat_category;
    team;
    type_grouping;
    weight;
    yes_or_no;
}

macro_rules! convert {
    {
        $subset:ident => $superset:ident {
            $($variant:ident,)*
        }
    } => {
        impl From<$subset> for $superset {
            fn from(value: $subset) -> Self {
                match value {
                    $(
                        $subset::$variant => Self::$variant,
                    )*
                }
            }
        }

        impl TryFrom<$superset> for $subset {
            type Error = ();

            fn try_from(value: $superset) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $superset::$variant => Ok(Self::$variant),
                    )*
                    _ => Err(()),
                }
            }
        }
    };
}
use convert;
