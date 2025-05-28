use crate::tables::speedtest;
use crate::tables::SpeedtestResult;
use spacetimedb::reducer;
use spacetimedb::sys::Result;
use spacetimedb::ReducerContext;
use spacetimedb::Table;

pub mod tables;
pub mod types;

#[reducer]
pub fn record_speedtest(ctx: &ReducerContext, speedtest: SpeedtestResult) -> Result<()> {
    log::info!("Recording speedtest: {}", speedtest.id);

    ctx.db.speedtest().insert(speedtest);

    Ok(())
}

#[reducer]
pub fn delete_speedtest(ctx: &ReducerContext, id: String) -> Result<()> {
    let p = id.clone();

    if let Some(s) = ctx.db.speedtest().id().find(id) {
        ctx.db.speedtest().delete(s);
        log::info!("Speedtest deleted: {p}");
    } else {
        log::info!("Speedtest {p} not found");
    }

    Ok(())
}
