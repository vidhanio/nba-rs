macro_rules! params {
    {
        $(
            class $name:ident:
                $($variant:ident = $s:literal)*
                [$default:expr]
        )*
    } => {
        ::paste::paste! {
            $(
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub enum $name {
                    $([<$variant:camel>]),*
                }

                impl ::std::default::Default for $name {
                    fn default() -> Self {
                        Self::[<$default:camel>]
                    }
                }

                impl ::std::fmt::Display for $name {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match self {
                            $(Self::[<$variant:camel>] => write!(f, $s)),*
                        }
                    }
                }
            )*
        }
    };
}

params! {
    class AheadBehind:
        ahead_or_behind = "Ahead or Behind"
        behind_or_tied = "Behind or Tied"
        ahead_or_tied = "Ahead or Tied"
        [ahead_or_behind]

    class ClutchTime:
        last_5_minutes = "Last 5 Minutes"
        last_4_minutes = "Last 4 Minutes"
        last_3_minutes = "Last 3 Minutes"
        last_2_minutes = "Last 2 Minutes"
        last_1_minute = "Last 1 Minute"
        last_30_seconds = "Last 30 Seconds"
        last_10_seconds = "Last 10 Seconds"
        [last_5_minutes]

    class Conference:
        east = "East"
        west = "West"
        none = ""
        [none]

    class ContextMeasureSimple:
        pts = "PTS"
        fgm = "FGM"
        fga = "FGA"
        fg_pct = "FG_PCT"
        fg3m = "FG3M"
        fg3a = "FG3A"
        fg3_pct = "FG3_PCT"
        pts_fb = "PTS_FB"
        pts_off_tov = "PTS_OFF_TOV"
        pts_2nd_chance = "PTS_2ND_CHANCE"
        pf = "PF"
        efg_pct = "EFG_PCT"
        ts_pct = "TS_PCT"
        [pts]

    class ContextMeasureDetailed:
        pts = "PTS"
        fgm = "FGM"
        fga = "FGA"
        fg_pct = "FG_PCT"
        fg3m = "FG3M"
        fg3a = "FG3A"
        fg3_pct = "FG3_PCT"
        pts_fb = "PTS_FB"
        pts_off_tov = "PTS_OFF_TOV"
        pts_2nd_chance = "PTS_2ND_CHANCE"
        ftm = "FTM"
        fta = "FTA"
        oreb = "OREB"
        dreb = "DREB"
        ast = "AST"
        fgm_ast = "FGM_AST"
        fg3_ast = "FG3_AST"
        stl = "STL"
        blk = "BLK"
        blka = "BLKA"
        tov = "TOV"
        poss_end_ft = "POSS_END_FT"
        pts_paint = "PTS_PAINT"
        reb = "REB"
        tm_fgm = "TM_FGM"
        tm_fga = "TM_FGA"
        tm_fg3m = "TM_FG3M"
        tm_fg3a = "TM_FG3A"
        tm_ftm = "TM_FTM"
        tm_fta = "TM_FTA"
        tm_oreb = "TM_OREB"
        tm_dreb = "TM_DREB"
        tm_reb = "TM_REB"
        tm_team_reb = "TM_TEAM_REB"
        tm_ast = "TM_AST"
        tm_stl = "TM_STL"
        tm_blk = "TM_BLK"
        tm_blka = "TM_BLKA"
        tm_tov = "TM_TOV"
        tm_team_tov = "TM_TEAM_TOV"
        tm_pf = "TM_PF"
        tm_pfd = "TM_PFD"
        tm_pts = "TM_PTS"
        tm_pts_paint = "TM_PTS_PAINT"
        tm_pts_fb = "TM_PTS_FB"
        tm_pts_off_tov = "TM_PTS_OFF_TOV"
        tm_pts_2nd_chance = "TM_PTS_2ND_CHANCE"
        tm_fgm_ast = "TM_FGM_AST"
        tm_fg3_ast = "TM_FG3_AST"
        tm_poss_end_ft = "TM_POSS_END_FT"
        opp_ftm = "OPP_FTM"
        opp_fta = "OPP_FTA"
        opp_oreb = "OPP_OREB"
        opp_dreb = "OPP_DREB"
        opp_reb = "OPP_REB"
        opp_team_reb = "OPP_TEAM_REB"
        opp_ast = "OPP_AST"
        opp_stl = "OPP_STL"
        opp_blk = "OPP_BLK"
        opp_blka = "OPP_BLKA"
        opp_tov = "OPP_TOV"
        opp_team_tov = "OPP_TEAM_TOV"
        opp_pf = "OPP_PF"
        opp_pfd = "OPP_PFD"
        opp_pts = "OPP_PTS"
        opp_pts_paint = "OPP_PTS_PAINT"
        opp_pts_fb = "OPP_PTS_FB"
        opp_pts_off_tov = "OPP_PTS_OFF_TOV"
        opp_pts_2nd_chance = "OPP_PTS_2ND_CHANCE"
        opp_fgm_ast = "OPP_FGM_AST"
        opp_fg3_ast = "OPP_FG3_AST"
        opp_poss_end_ft = "OPP_POSS_END_FT"
        [pts]

    class DefenseCategory:
        overall = "Overall"
        threes = "3 Pointers"
        twos = "2 Pointers"
        less_than_6ft = "Less Than 6Ft"
        less_than_10ft = "Less Than 10Ft"
        greater_than_15ft = "Greater Than 15Ft"
        [overall]

    class Direction:
        asc = "ASC"
        desc = "DESC"
        [asc]

    class DistanceRange:
        range_5ft = "5ft Range"
        range_8ft = "8ft Range"
        by_zone = "By Zone"
        [by_zone]

    class DivisionSimple:
        atlantic = "Atlantic"
        central = "Central"
        northwest = "Northwest"
        pacific = "Pacific"
        southeast = "Southeast"
        southwest = "Southwest"
        [atlantic]

    class Division:
        east = "East"
        west = "West"
        [east]

    class GameScopeSimple:
        last_10 = "Last 10"
        yesterday = "Yesterday"
        [last_10]

    class GameScopeDetailed:
        last_10 = "Last 10"
        yesterday = "Yesterday"
        season = "Season"
        finals = "Finals"
        [season]

    class GameSegment:
        first_half = "First Half"
        overtime = "Overtime"
        second_half = "Second Half"
        [first_half]

    class LeagueID:
        nba = "00"
        aba = "01"
        wnba = "10"
        g_league = "20"
        [nba]

    class Location:
        home = "Home"
        road = "Road"
        [home]

    class MeasureTypeBase:
        base = "Base"
        [base]

    class MeasureTypeSimple:
        base = "Base"
        opponent = "Opponent"
        [base]

    class MeasureTypePlayerGameLogs:
        base = "Base"
        advanced = "Advanced"
        misc = "Misc"
        scoring = "Scoring"
        usage = "Usage"
        [base]

    class MeasureTypeDetailed:
        base = "Base"
        opponent = "Opponent"
        advanced = "Advanced"
        misc = "Misc"
        scoring = "Scoring"
        usage = "Usage"
        four_factors = "Four Factors"
        [base]

    class MeasureTypeDetailedDefense:
    base = "Base"
        opponent = "Opponent"
        advanced = "Advanced"
        misc = "Misc"
        scoring = "Scoring"
        usage = "Usage"
        four_factors = "Four Factors"
        defense = "Defense"
        [base]

    class Outcome:
        win = "W"
        loss = "L"
        [win]

    class PaceAdjust:
        no = "N"
        yes = "Y"
        [no]

    class PaceAdjustNo:
        no = "N"
        [no]

    class PlusMinus:
        no = "N"
        yes = "Y"
        [no]

    class PlusMinusNo:
        no = "N"
        [no]

    class Period:
        all = "0"
        first = "1"
        second = "2"
        third = "3"
        fourth = "4"
        [all]

    class StartPeriod:
        all = "0"
        first = "1"
        second = "2"
        third = "3"
        fourth = "4"
        [all]

    class EndPeriod:
        all = "0"
        first = "1"
        second = "2"
        third = "3"
        fourth = "4"
        [all]

    class PerModeSimple:
        totals = "Totals"
        per_game = "PerGame"
        [totals]

    class PerMode36:
        totals = "Totals"
        per_game = "PerGame"
        per_36 = "Per36"
        [totals]

    class PerMode48:
        totals = "Totals"
        per_game = "PerGame"
        per_48 = "Per48"
        [totals]

    class PerModeTime:
        totals = "Totals"
        per_game = "PerGame"
        per_36 = "Per36"
        per_48 = "Per48"
        minutes_per = "MinutesPer"
        per_40 = "Per40"
        [totals]

    class PerModeDetailed:
        totals = "Totals"
        per_game = "PerGame"
        per_36 = "Per36"
        per_48 = "Per48"
        minutes_per = "MinutesPer"
        per_40 = "Per40"
        per_minute = "PerMinute"
        per_possession = "PerPossession"
        per_play = "PerPlay"
        per_100_possessions = "Per100Possessions"
        per_100_plays = "Per100Plays"
        [totals]

    class PlayerExperience:
        rookie = "Rookie"
        sophomore = "Sophomore"
        veteran = "Veteran"
        [rookie]

    class PlayerOrTeam:
        player = "Player"
        team = "Team"
        [team]

    class PlayerOrTeamAbbreviation:
        player = "P"
        team = "T"
        [team]

    class PlayerPosition:
        forward = "Forward"
        center = "Center"
        guard = "Guard"
        [forward]

    class PlayerPositionAbbreviation:
        forward = "F"
        center = "C"
        guard = "G"
        center_forward = "C-F"
        forward_center = "F-C"
        forward_guard = "F-G"
        guard_forward = "G-F"
        [forward]

    class PlayerScope:
        all_players = "All Players"
        rookies = "Rookies"
        [all_players]

    class TodaysPlayers:
        no = "N"
        yes = "Y"
        [no]

    class ActivePlayers:
        no = "N"
        yes = "Y"
        [no]

    class PlayType:
        transition = "Transition"
        isolation = "Isolation"
        pr_ball_handler = "PRBallHandler"
        pr_roll_man = "PRRollman"
        post_up = "Postup"
        spot_up = "Spotup"
        handoff = "Handoff"
        cut = "Cut"
        off_screen = "OffScreen"
        putbacks = "OffRebound"
        misc = "Misc"
        [transition]

    class PtMeasureType:
        speed_distance = "SpeedDistance"
        rebounding = "Rebounding"
        possessions = "Possessions"
        catch_shoot = "CatchShoot"
        pull_up_shot = "PullUpShot"
        defense = "Defense"
        drives = "Drives"
        passing = "Passing"
        elbowTouch = "ElbowTouch"
        postTouch = "PostTouch"
        paintTouch = "PaintTouch"
        efficiency = "Efficiency"
        [speed_distance]

    class Rank:
        no = "N"
        yes = "Y"
        [no]

    class RankNo:
        no = "N"
        [no]

    class Scope:
        rs = "RS"
        s = "S"
        rookies = "Rookies"
        [s]

    class SeasonType:
        regular = "Regular Season"
        preseason = "Pre Season"
        [regular]

    class SeasonTypePlayoffs:
        regular = "Regular Season"
        preseason = "Pre Season"
        playoffs = "Playoffs"
        [regular]

    class SeasonTypeAllStar:
        regular = "Regular Season"
        preseason = "Pre Season"
        playoffs = "Playoffs"
        all_star = "All Star"
        [regular]

    class SeasonSegment:
        post_all_star = "Post All-Star"
        pre_all_star = "Pre All-Star"
        [post_all_star]

    class ShotClockRange:
        range_22_24 = "24-22"
        range_18_22 = "22-18 Very Early"
        range_15_18 = "18-15 Early"
        range_7_15 = "15-7 Average"
        range_4_7 = "7-4 Late"
        range_0_4 = "4-0 Very Late"
        shot_clock_off = "ShotClock Off"
        [shot_clock_off]

    class Sorter:
        fgm = "FGM"
        fga = "FGA"
        fg_pct = "FG_PCT"
        fg3m = "FG3M"
        fg3a = "FG3A"
        fg3_pct = "FG3_PCT"
        ftm = "FTM"
        fta = "FTA"
        ft_pct = "FT_PCT"
        oreb = "OREB"
        dreb = "DREB"
        ast = "AST"
        stl = "STL"
        blk = "BLK"
        tov = "TOV"
        reb = "REB"
        pts = "PTS"
        date = "DATE"
        [date]

    class StarterBench:
        starters = "Starters"
        bench = "Bench"
        [starters]

    class Stat:
        points = "PTS"
        rebounds = "REB"
        assists = "AST"
        field_goal_percent = "FG_PCT"
        free_throw_percent = "FT_PCT"
        threes_percent = "FG3_PCT"
        steals = "STL"
        blocks = "BLK"
        [points]

    class StatCategory:
        points = "Points"
        rebounds = "Rebounds"
        assists = "Assists"
        defense = "Defense"
        clutch = "Clutch"
        playmaking = "Playmaking"
        efficiency = "Efficiency"
        fast_break = "Fast Break"
        scoring_breakdown = "Scoring Breakdown"
        [points]

    class StatCategoryAbbreviation:
        pts = "PTS"
        fgm = "FGM"
        fga = "FGA"
        fg_pct = "FG_PCT"
        fg3m = "FG3M"
        fg3a = "FG3A"
        fg3_pct = "FG3_PCT"
        ftm = "FTM"
        fta = "FTA"
        oreb = "OREB"
        dreb = "DREB"
        ast = "AST"
        stl = "STL"
        blk = "BLK"
        tov = "TOV"
        reb = "REB"
        [pts]

    class StatType:
        traditional = "Traditional"
        advanced = "Advanced"
        tracking = "Tracking"
        [traditional]

    class TypeGrouping:
        offensive = "offensive"
        defensive = "defensive"
        [offensive]
}
