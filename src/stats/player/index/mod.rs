mod parameters;

pub use self::parameters::PlayerIndexParameters;

crate::endpoint! { "playerindex"

    PlayerIndex(PlayerIndexParameters) {
        player_index["PlayerIndex"]: PlayerIndexRow {
            person_id: u32,
            player_last_name: String,
            player_first_name: String,
            player_slug: String,
            team_id: u32,
            team_slug: Option<String>,
            is_defunct: u32,
            team_city: Option<String>,
            team_name: Option<String>,
            team_abbreviation: Option<String>,
            jersey_number: Option<String>,
            position: String,
            height: String,
            weight: String,
            college: String,
            country: String,
            draft_year: Option<u32>,
            draft_round: Option<u32>,
            draft_number: Option<u32>,
            roster_status: Option<f64>,
            pts: Option<f64>,
            reb: Option<f64>,
            ast: Option<f64>,
            stats_timeframe: String,
            from_year: String,
            to_year: String,
        },
    }
}
