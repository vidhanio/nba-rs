//! All possible fields which may be used in the request or response of a
//! [`crate::stats::Endpoint`].

macro_rules! reexport {
    {
        $($name:ident);* $(;)?
    } => {
        $(
            mod $name;
            pub use self::$name::*;
        )*
    };
}

reexport! {
    active_flag;
    ahead_or_behind;
    clutch_time;
    college;
    conference;
    country;
    direction;
    division;
    draft_pick;
    draft_year;
    experience;
    game_id;
    half;
    height;
    last_n_games;
    league_id;
    location;
    measure_type;
    month;
    outcome;
    per_mode;
    play_type;
    player_or_team;
    playoff_round;
    point_diff;
    position;
    quarter;
    scope;
    season;
    season_segment;
    season_type;
    shot_clock_range;
    sorter;
    starter_or_bench;
    stat_category;
    team;
    type_grouping;
    weight;
    yes_or_no;
}

macro_rules! trait_param {
    {$Trait:ident {
        $($Ty:ty { $ResultSet:ident };)*
    }} => {
        #[::sealed::sealed]
        pub trait $Trait:
            ::serde::Serialize
            + ::serde::de::DeserializeOwned
            + ::std::fmt::Debug
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::cmp::Eq
            + ::std::marker::Sync
        {
            type ResultSet: ::serde::Serialize
                + ::serde::de::DeserializeOwned
                + ::std::fmt::Debug
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::marker::Sync;
        }

        $(
            #[::sealed::sealed]
            impl $Trait for $Ty {
                type ResultSet = $ResultSet;
            }
        )*
    };
}
pub(crate) use trait_param;
