use serde::{Deserialize, Serialize};

use crate::{
    fields::{LeagueId, PerMode, SeasonType},
    Endpoint,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
#[endpoint(path = "alltimeleadersgrids")]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
#[endpoint(row(field = gp_leaders, ty = "GpLeadersRow", row = "GPLeaders"))]
#[endpoint(row(field = pts_leaders, ty = "PtsLeadersRow", row = "PTSLeaders"))]
#[endpoint(row(field = ast_leaders, ty = "AstLeadersRow", row = "ASTLeaders"))]
#[endpoint(row(field = stl_leaders, ty = "StlLeadersRow", row = "STLLeaders"))]
#[endpoint(row(field = oreb_leaders, ty = "OrebLeadersRow", row = "OREBLeaders"))]
#[endpoint(row(field = dreb_leaders, ty = "DrebLeadersRow", row = "DREBLeaders"))]
#[endpoint(row(field = reb_leaders, ty = "RebLeadersRow", row = "REBLeaders"))]
#[endpoint(row(field = blk_leaders, ty = "BlkLeadersRow", row = "BLKLeaders"))]
#[endpoint(row(field = fgm_leaders, ty = "FgmLeadersRow", row = "FGMLeaders"))]
#[endpoint(row(field = fga_leaders, ty = "FgaLeadersRow", row = "FGALeaders"))]
#[endpoint(row(field = fg_pct_leaders, ty = "FgPctLeadersRow", row = "FG_PCTLeaders"))]
#[endpoint(row(field = tov_leaders, ty = "TovLeadersRow", row = "TOVLeaders"))]
#[endpoint(row(field = fg3m_leaders, ty = "Fg3mLeadersRow", row = "FG3MLeaders"))]
#[endpoint(row(field = fg3a_leaders, ty = "Fg3aLeadersRow", row = "FG3ALeaders"))]
#[endpoint(row(field = fg3_pct_leaders, ty = "Fg3PctLeadersRow", row = "FG3_PCTLeaders"))]
#[endpoint(row(field = pf_leaders, ty = "PfLeadersRow", row = "PFLeaders"))]
#[endpoint(row(field = ftm_leaders, ty = "FtmLeadersRow", row = "FTMLeaders"))]
#[endpoint(row(field = fta_leaders, ty = "FtaLeadersRow", row = "FTALeaders"))]
#[endpoint(row(field = ft_pct_leaders, ty = "FtPctLeadersRow", row = "FT_PCTLeaders"))]
pub struct AllTimeLeadersGrids {
    #[serde(rename = "LeagueID")]
    league_id: LeagueId,
    per_mode: PerMode,
    season_type: SeasonType,

    #[serde(with = "crate::serde::string_u32")]
    top_x: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GpLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub gp: i32,
    pub gp_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PtsLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub pts: f64,
    pub pts_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AstLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub ast: f64,
    pub ast_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StlLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub stl: f64,
    pub stl_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OrebLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub oreb: f64,
    pub oreb_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DrebLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub dreb: f64,
    pub dreb_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RebLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub reb: f64,
    pub reb_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BlkLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub blk: f64,
    pub blk_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FgmLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub fgm: f64,
    pub fgm_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FgaLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub fga: f64,
    pub fga_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FgPctLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub fg_pct: f64,
    pub fg_pct_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TovLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub tov: f64,
    pub tov_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fg3mLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub fg3m: f64,
    pub fg3m_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fg3aLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub fg3a: f64,
    pub fg3a_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fg3PctLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub fg3_pct: f64,
    pub fg3_pct_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PfLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub pf: f64,
    pub pf_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FtmLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub ftm: f64,
    pub ftm_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FtaLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub fta: f64,
    pub fta_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FtPctLeadersRow {
    pub player_id: i32,
    pub player_name: String,
    pub ft_pct: f64,
    pub ft_pct_rank: i32,
    pub is_active_flag: String,
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;

    use super::*;

    #[tokio::test]
    async fn works() {
        assert_ok!(
            AllTimeLeadersGrids {
                per_mode: PerMode::Totals,
                top_x: 5,
                ..Default::default()
            }
            .send()
            .await
        );
    }
}
