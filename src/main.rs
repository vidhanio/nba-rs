use nba_api::{NBAHttp, NBAResponse, NBAStatsHttp};

#[tokio::main]
async fn main() {
    let json = NBAStatsHttp
        .send_api_request(
            "playercareerstats",
            &[("PlayerID", "203076"), ("PerMode", "Totals")],
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap()
        .json()
        .unwrap();

    println!("{json:#}");
}
