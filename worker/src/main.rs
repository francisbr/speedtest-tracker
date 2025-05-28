use crate::database::bindings::DbConnection;
use crate::database::bindings::EventContext;
use crate::database::bindings::SpeedtestResult;
use crate::database::bindings::SpeedtestTableAccess;
use crate::database::bindings::record_speedtest;
use chrono::Utc;
use chrono_tz::America::Montreal;
use cron::Schedule;
use spacetimedb_sdk::Table;
use spacetimedb_sdk::Timestamp;
use std::fmt::Display;
use std::process::Command;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::LazyLock;
use std::thread;

mod database;

static PROGRAM_START: LazyLock<Timestamp> = LazyLock::new(|| Timestamp::now());

#[tokio::main]
async fn main() {
    let ctx = Arc::new(database::get_connection());
    ctx.db.speedtest().on_insert(on_speedtest_insert);

    println!("Starting job scheduler.");
    let schedule = Schedule::from_str("0/30 * * * * *").unwrap();
    while let Some(t) = schedule.upcoming(Utc).take(1).next() {
        thread::sleep((t - Utc::now()).to_std().unwrap());

        let db_ctx = ctx.clone();
        tokio::spawn(async move {
            worker(&db_ctx).await;
        });
    }
}

async fn worker(db_ctx: &DbConnection) {
    log("Job Started");

    let output = Command::new("speedtest")
        .args(["-f", "json"])
        .output()
        .unwrap();
    let speedtest = serde_json::from_slice::<SpeedtestResult>(&output.stdout).unwrap();

    db_ctx.reducers.record_speedtest(speedtest).unwrap();
}

fn on_speedtest_insert(_ctx: &EventContext, speedtest: &SpeedtestResult) {
    if speedtest.timestamp < *PROGRAM_START {
        return;
    }

    log(format!("Speedtest recorded {}", speedtest.id));
}

fn log(message: impl Display) {
    println!(
        "[{}] {}",
        Utc::now()
            .with_timezone(&Montreal)
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
        message
    )
}
