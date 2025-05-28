use crate::database::bindings::DbConnection;
use spacetimedb_sdk::DbContext;
use spacetimedb_sdk::credentials::File;
use std::process::exit;

pub mod bindings;
mod serde;

pub fn get_connection() -> DbConnection {
    println!("Acquiring Database connection.");

    let ctx = DbConnection::builder()
        .with_uri("http://192.168.50.19:3333")
        .with_module_name("speedtest")
        .with_token(File::new("speedtest").load().unwrap())
        .build()
        .unwrap();

    ctx.subscription_builder()
        .on_error(|_ctx, error| {
            println!("{error:#?}");
            exit(1);
        })
        .subscribe_to_all_tables();

    ctx.run_threaded();
    ctx
}
