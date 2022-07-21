#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::use_self)]

mod analyzed;
mod download;
mod filter_invalid;
mod missing_params;

use anyhow::Result;
use nba_api::NBAClient;
use once_cell::sync::Lazy;
use reqwest::Response;

#[macro_export]
macro_rules! regex {
    ($($re:expr),+ $(,)?) => {($({
        static RE: ::once_cell::sync::OnceCell<::regex::Regex> = ::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| ::regex::Regex::new($re).unwrap())
    }),+)};
}

macro_rules! dirs {
    {$(
        $vis:vis static $name:ident = $($path:tt) / *;
    )+} => {$(
        $vis static $name: ::once_cell::sync::Lazy<::std::path::PathBuf> = ::once_cell::sync::Lazy::new(|| {
            let path = ::std::path::PathBuf::new()
                $(.join(&*$path))*;

            assert!(path.exists());

            path
        });
    )+};
}

const ENDPOINTS: &[&str] = &[
    "2AssistTracker",
    "AllPlayers",
    "AllStarBallotPredictor",
    "AllTimeLeadersGrids",
    "AssistLeaders",
    "AssistTracker",
    "AssistTrackerStats",
    "BoxScoreAdvanced",
    "BoxScoreAdvancedV2",
    "BoxScoreDefensive",
    "BoxScoreFourFactors",
    "BoxScoreFourFactorsV2",
    "BoxScoreMatchups",
    "BoxScoreMisc",
    "BoxScoreMiscV2",
    "BoxScorePlayerTracking",
    "BoxScorePlayerTrackV2",
    "BoxScoreScoring",
    "BoxScoreScoringV2",
    "BoxScoreSimilarityScore",
    "BoxScoreSummary",
    "BoxScoreSummaryV2",
    "BoxScoreTraditional",
    "BoxScoreTraditionalV2",
    "BoxScoreUsage",
    "BoxScoreUsageV2",
    "CommonAllPlayers",
    "CommonPlayerInfo",
    "CommonPlayoffSeries",
    "CommonTeamRoster",
    "CommonTeamYears",
    "CumeStatsPlayer",
    "CumeStatsPlayerGames",
    "CumeStatsTeam",
    "CumeStatsTeamGames",
    "DefenseHub",
    "DLeaguePredictor",
    "DraftBoard",
    "DraftCombineDrillResults",
    "DraftCombineNonStationaryShooting",
    "DraftCombinePlayerAnthro",
    "DraftCombinePlayerMeasurements",
    "DraftCombineSpotShooting",
    "DraftCombineStats",
    "DraftHistory",
    "FantasyWidget",
    "FranchiseHistory",
    "FranchiseLeaders",
    "FranchisePlayers",
    "GameRotation",
    "GLAlumBoxScoreSimilarityScore",
    "GLeaguePredictor",
    "HomePage",
    "HomePageLeaders",
    "HomePageV2",
    "HustleStatsBoxScore",
    "InfographicFanDuelPlayer",
    "LeadersTiles",
    "LeagueDashLineups",
    "LeagueDashOppPtShot",
    "LeagueDashPlayerBioStats",
    "LeagueDashPlayerClutch",
    "LeagueDashPlayerPtShot",
    "LeagueDashPlayerShotLocations",
    "LeagueDashPlayerStats",
    "LeagueDashPtDefend",
    "LeagueDashPtStats",
    "LeagueDashPtTeamDefend",
    "LeagueDashTeamClutch",
    "LeagueDashTeamPtShot",
    "LeagueDashTeamShotLocations",
    "LeagueDashTeamStats",
    "LeagueGameFinder",
    "LeagueGameLog",
    "LeagueHustleStatsPlayer",
    "LeagueHustleStatsPlayerLeaders",
    "LeagueHustleStatsTeam",
    "LeagueHustleStatsTeamLeaders",
    "LeagueLeaders",
    "LeagueLineupViz",
    "LeaguePlayerOnDetails",
    "LeagueSeasonMatchups",
    "LeagueStandings",
    "LeagueStandingsV3",
    "LineupStats",
    "MatchupsRollup",
    "PlayByPlay",
    "PlayByPlayMini",
    "PlayByPlayV2",
    "PlayerAwards",
    "PlayerBioStats",
    "PlayerCareerByCollege",
    "PlayerCareerByCollegeRollup",
    "PlayerCareerStats",
    "PlayerClutchStats",
    "PlayerCompare",
    "PlayerCompareStats",
    "PlayerDashboardByClutch",
    "PlayerDashboardByGameSplits",
    "PlayerDashboardByGeneralSplits",
    "PlayerDashboardByLastNGames",
    "PlayerDashboardByOpponent",
    "PlayerDashboardByShootingSplits",
    "PlayerDashboardByTeamPerformance",
    "PlayerDashboardByYearOverYear",
    "PlayerDashPtPass",
    "PlayerDashPtReb",
    "PlayerDashPtReboundLogs",
    "PlayerDashPtShotDefend",
    "PlayerDashPtShotlog",
    "PlayerDashPtShots",
    "PlayerDefenseStats",
    "PlayerEstimatedMetrics",
    "PlayerFantasyProfile",
    "PlayerFantasyProfileBarGraph",
    "PlayerGameLog",
    "PlayerGameLogs",
    "PlayerGameLogsStats",
    "PlayerGameSplitsStats",
    "PlayerGameStreakFinder",
    "PlayerGeneralSplitsStats",
    "PlayerInfo",
    "PlayerLastNGamesStats",
    "PlayerNextNGames",
    "PlayerOnDetails",
    "PlayerOpponentStats",
    "PlayerPassesStats",
    "PlayerProfile",
    "PlayerProfileV2",
    "PlayerReboundsStats",
    "PlayersCareerStats",
    "PlayersClutchStats",
    "PlayersDefenseStats",
    "PlayersGeneralStats",
    "PlayerShotChartDetail",
    "PlayerShotsStats",
    "PlayersHustleLeaders",
    "PlayersHustleStats",
    "PlayersShotLocationStats",
    "PlayersShotStats",
    "PlayersTrackingStats",
    "PlayersVsPlayers",
    "PlayerTeamPerformanceStats",
    "PlayerTrackBucketSimilarityScore",
    "PlayerTrackRankSimilarityComp",
    "PlayerTrackSimilarityScore",
    "PlayerTrackSimilarityUniqueness",
    "PlayerVsPlayer",
    "PlayerYearOverYearStats",
    "PlayoffPicture",
    "PlayoffSeries",
    "Scoreboard",
    "ScoreboardMini",
    "ScoreboardV2",
    "ShotChartDetail",
    "ShotChartLeagueWide",
    "ShotChartLineupDetail",
    "SynergyBucketSimilarityScore",
    "SynergyPlayTypes",
    "SynergySimilarityScore",
    "TeamAndPlayerVsPlayers",
    "TeamAndPlayersVsPlayers",
    "TeamClutchStats",
    "TeamDashboardByClutch",
    "TeamDashboardByGameSplits",
    "TeamDashboardByGeneralSplits",
    "TeamDashboardByLastNGames",
    "TeamDashboardByOpponent",
    "TeamDashboardByShootingSplits",
    "TeamDashboardByTeamPerformance",
    "TeamDashboardByYearOverYear",
    "TeamDashLineups",
    "TeamDashPtPass",
    "TeamDashPtReb",
    "TeamDashPtShots",
    "TeamDetails",
    "TeamEstimatedMetrics",
    "TeamFranchiseLeaders",
    "TeamFranchiseLeadersRank",
    "TeamGameLog",
    "TeamGameLogs",
    "TeamGameSplitsStats",
    "TeamGameStreakFinder",
    "TeamGeneralSplitsStats",
    "TeamHistoricalLeaders",
    "TeamInfo",
    "TeamInfoCommon",
    "TeamLastNGamesStats",
    "TeamLineupStats",
    "TeamOpponentStats",
    "TeamPassesStats",
    "TeamPerformanceStats",
    "TeamPlayerDashboard",
    "TeamPlayerOnOffDetails",
    "TeamPlayerOnOffSummary",
    "TeamPlayerStats",
    "TeamReboundsStats",
    "TeamRoster",
    "TeamsClutchStats",
    "TeamsDefenseStats",
    "TeamsGeneralStats",
    "TeamShootingSplitsStats",
    "TeamShotChartLineupDetail",
    "TeamShotsStats",
    "TeamsHustleLeaders",
    "TeamsHustleStats",
    "TeamsShotLocationStats",
    "TeamsShotStats",
    "TeamsYearByYearStats",
    "TeamVsPlayer",
    "TeamYearByYearStats",
    "TeamYearOverYearStats",
    "TeamYears",
    "VideoDetails",
    "VideoEvents",
    "VideoStatus",
    "WinProbabilityPlayByPlay",
    "WinProbabilityPBP",
];

dirs! {
    static ANALYSIS_DIR = "analysis";

    static RESPONSES_DIR = ANALYSIS_DIR / "0_responses";

    static INVALID_DIR = ANALYSIS_DIR / "1_invalid";
    static VALID_DIR = ANALYSIS_DIR / "1_valid";

    static PARAMS_NOT_FOUND_DIR = ANALYSIS_DIR / "2_params_not_found";

    static JSON_DIR = ANALYSIS_DIR / "3_json";
}

static CLIENT: Lazy<NBAClient> = Lazy::new(NBAClient::new);

async fn nba_request(endpoint: &str, params: &[(&str, &str)]) -> Result<Response> {
    CLIENT
        .send_request(endpoint, params)
        .await
        .map_err(Into::into)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    download::download(ENDPOINTS).await?;
    filter_invalid::filter_invalid().await?;
    missing_params::missing_params().await?;

    Ok(())
}
