use serde::Deserialize;

use crate::stats::endpoints::ResultSet;

#[derive(Deserialize)]
pub struct SeasonTotalsRegularSeason {
    #[serde(rename = "rowSet")]
    row_set: Vec<SeasonTotalsRegularSeasonRow>,
}

impl ResultSet for SeasonTotalsRegularSeason {
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

    fn row_set(&self) -> &[Self::Row] {
        &self.row_set
    }
}

#[derive(Deserialize)]
pub struct SeasonTotalsRegularSeasonRow {
    #[serde(rename = "PLAYER_ID")]
    pub player_id: i32,
    #[serde(rename = "SEASON_ID")]
    pub season_id: String,
    #[serde(rename = "LEAGUE_ID")]
    pub league_id: String,
    #[serde(rename = "TEAM_ID")]
    pub team_id: i32,
    #[serde(rename = "TEAM_ABBREVIATION")]
    pub team_abbreviation: String,
    #[serde(rename = "PLAYER_AGE")]
    pub player_age: i32,
    #[serde(rename = "GP")]
    pub gp: i32,
    #[serde(rename = "GS")]
    pub gs: i32,
    #[serde(rename = "MIN")]
    pub min: i32,
    #[serde(rename = "FGM")]
    pub fgm: i32,
    #[serde(rename = "FGA")]
    pub fga: i32,
    #[serde(rename = "FG_PCT")]
    pub fg_pct: f32,
    #[serde(rename = "FG3M")]
    pub fg3m: i32,
    #[serde(rename = "FG3A")]
    pub fg3a: i32,
    #[serde(rename = "FG3_PCT")]
    pub fg3_pct: f32,
    #[serde(rename = "FTM")]
    pub ftm: i32,
    #[serde(rename = "FTA")]
    pub fta: i32,
    #[serde(rename = "FT_PCT")]
    pub ft_pct: f32,
    #[serde(rename = "OREB")]
    pub oreb: i32,
    #[serde(rename = "DREB")]
    pub dreb: i32,
    #[serde(rename = "REB")]
    pub reb: i32,
    #[serde(rename = "AST")]
    pub ast: i32,
    #[serde(rename = "STL")]
    pub stl: i32,
    #[serde(rename = "BLK")]
    pub blk: i32,
    #[serde(rename = "TOV")]
    pub tov: i32,
    #[serde(rename = "PF")]
    pub pf: i32,
    #[serde(rename = "PTS")]
    pub pts: i32,
}
