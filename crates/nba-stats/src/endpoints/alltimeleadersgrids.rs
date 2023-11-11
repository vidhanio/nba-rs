use serde::{Deserialize, Serialize};

use crate::{
    fields::{LeagueId, PerMode, SeasonType},
    serde_utils, Endpoint,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
#[endpoint(path = "alltimeleadersgrids")]
#[endpoint(result_set(field = gp_leaders, ty = "GpLeadersResultSet", name = "GPLeaders"))]
#[endpoint(result_set(field = pts_leaders, ty = "PtsLeadersResultSet", name = "PTSLeaders"))]
#[endpoint(result_set(field = ast_leaders, ty = "AstLeadersResultSet", name = "ASTLeaders"))]
#[endpoint(result_set(field = stl_leaders, ty = "StlLeadersResultSet", name = "STLLeaders"))]
#[endpoint(result_set(field = oreb_leaders, ty = "OrebLeadersResultSet", name = "OREBLeaders"))]
#[endpoint(result_set(field = dreb_leaders, ty = "DrebLeadersResultSet", name = "DREBLeaders"))]
#[endpoint(result_set(field = reb_leaders, ty = "RebLeadersResultSet", name = "REBLeaders"))]
#[endpoint(result_set(field = blk_leaders, ty = "BlkLeadersResultSet", name = "BLKLeaders"))]
#[endpoint(result_set(field = fgm_leaders, ty = "FgmLeadersResultSet", name = "FGMLeaders"))]
#[endpoint(result_set(field = fga_leaders, ty = "FgaLeadersResultSet", name = "FGALeaders"))]
#[endpoint(result_set(field = fg_pct_leaders, ty = "FgPctLeadersResultSet", name = "FG_PCTLeaders"))]
#[endpoint(result_set(field = tov_leaders, ty = "TovLeadersResultSet", name = "TOVLeaders"))]
#[endpoint(result_set(field = fg3m_leaders, ty = "Fg3mLeadersResultSet", name = "FG3MLeaders"))]
#[endpoint(result_set(field = fg3a_leaders, ty = "Fg3aLeadersResultSet", name = "FG3ALeaders"))]
#[endpoint(result_set(field = fg3_pct_leaders, ty = "Fg3PctLeadersResultSet", name = "FG3_PCTLeaders"))]
#[endpoint(result_set(field = pf_leaders, ty = "PfLeadersResultSet", name = "PFLeaders"))]
#[endpoint(result_set(field = ftm_leaders, ty = "FtmLeadersResultSet", name = "FTMLeaders"))]
#[endpoint(result_set(field = fta_leaders, ty = "FtaLeadersResultSet", name = "FTALeaders"))]
#[endpoint(result_set(field = ft_pct_leaders, ty = "FtPctLeadersResultSet", name = "FT_PCTLeaders"))]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AllTimeLeadersGrids {
    #[serde(rename = "LeagueID")]
    league_id: LeagueId,
    per_mode: PerMode,
    season_type: SeasonType,

    #[serde(with = "serde_utils::u32_as_str")]
    top_x: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GpLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub gp: i32,
    pub gp_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PtsLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub pts: f64,
    pub pts_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AstLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub ast: f64,
    pub ast_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StlLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub stl: f64,
    pub stl_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OrebLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub oreb: f64,
    pub oreb_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DrebLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub dreb: f64,
    pub dreb_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RebLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub reb: f64,
    pub reb_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BlkLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub blk: f64,
    pub blk_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FgmLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub fgm: f64,
    pub fgm_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FgaLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub fga: f64,
    pub fga_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FgPctLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub fg_pct: f64,
    pub fg_pct_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TovLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub tov: f64,
    pub tov_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fg3mLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub fg3m: f64,
    pub fg3m_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fg3aLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub fg3a: f64,
    pub fg3a_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fg3PctLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub fg3_pct: f64,
    pub fg3_pct_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PfLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub pf: f64,
    pub pf_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FtmLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub ftm: f64,
    pub ftm_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FtaLeadersResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub fta: f64,
    pub fta_rank: i32,
    pub is_active_flag: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FtPctLeadersResultSet {
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

    #[ignore = "don't want to spam the api"]
    #[tokio::test]
    async fn works() {
        println!(
            "{:#?}",
            assert_ok!(
                AllTimeLeadersGrids::default()
                    .with_per_mode(PerMode::Totals)
                    .with_top_x(15)
                    .send()
                    .await
            )
        );
    }
}
