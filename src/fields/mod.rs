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
    sorter;
    starter_or_bench;
    stat_category;
    team;
    type_grouping;
    weight;
    yes_or_no;
}
