use serde::{Deserialize, Serialize};

use super::ResultSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeasonTotalsRegularSeason {
    #[serde(rename = "rowSet")]
    row_set: Vec<SeasonTotalsRegularSeasonRow>,
}

impl ResultSet for SeasonTotalsRegularSeason {
    const ENDPOINT: &'static str = "playercareerstats";
    const HEADERS: &'static [&'static str] = &[
        "PLAYER_ID",
        "SEASON_ID",
        "LEAGUE_ID",
        "TEAM_ID",
        "TEAM_ABBREVIATION",
        "PLAYER_AGE",
        "GP",
        "GS",
        "MIN",
        "FGM",
        "FGA",
        "FG_PCT",
        "FG3M",
        "FG3A",
        "FG3_PCT",
        "FTM",
        "FTA",
        "FT_PCT",
        "OREB",
        "DREB",
        "REB",
        "AST",
        "STL",
        "BLK",
        "TOV",
        "PF",
        "PTS",
    ];

    type Row = SeasonTotalsRegularSeasonRow;

    fn rows(&self) -> &[Self::Row] {
        &self.row_set
    }

    fn set_rows(&mut self, rows: &[Self::Row]) {
        self.row_set = rows.to_owned();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SeasonTotalsRegularSeasonRow {
    pub player_id: i32,
    pub season_id: String,
    pub league_id: String,
    pub team_id: i32,
    pub team_abbreviation: String,
    pub player_age: i32,
    pub gp: i32,
    pub gs: i32,
    pub min: i32,
    pub fgm: i32,
    pub fga: i32,
    pub fg_pct: f32,
    pub fg3m: i32,
    pub fg3a: i32,
    pub fg3_pct: f32,
    pub ftm: i32,
    pub fta: i32,
    pub ft_pct: f32,
    pub oreb: i32,
    pub dreb: i32,
    pub reb: i32,
    pub ast: i32,
    pub stl: i32,
    pub blk: i32,
    pub tov: i32,
    pub pf: i32,
    pub pts: i32,
}
