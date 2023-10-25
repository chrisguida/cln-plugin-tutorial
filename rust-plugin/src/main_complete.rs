//! This is a test plugin used to verify that we can compile and run
//! plugins using the Rust API against Core Lightning.
#[macro_use]
extern crate serde_json;
use anyhow::anyhow;
use cln_plugin::{options, Builder, Error, Plugin};
use std::{thread, time::Duration};
use tokio;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let state = ();

    if let Some(plugin) = Builder::new(tokio::io::stdin(), tokio::io::stdout())
        .rpcmethod("testmethod", "This is a test", testmethod)
        .option(options::ConfigOption::new(
            "name",
            options::Value::String("World".to_string()),
            "Recipient of the greeting. Defaults to 'World'",
        ))
        .subscribe("connect", connect_handler)
        .hook("htlc_accepted", on_htlc_accepted)
        .dynamic()
        .start(state)
        .await?
    {
        plugin.join().await
    } else {
        Ok(())
    }
}

async fn testmethod(p: Plugin<()>, _v: serde_json::Value) -> Result<serde_json::Value, Error> {
    let name_value = p
        .option("name")
        .unwrap_or_else(|| options::Value::String("Unknown".to_string()));
    if let options::Value::String(name_str) = name_value {
        Ok(json!(format!("Hello, {}", name_str)))
    } else {
        Err(anyhow!("Expected a string for 'name' option".to_string())) // Adjust the error type and message as needed
    }
}

async fn connect_handler(_p: Plugin<()>, v: serde_json::Value) -> Result<(), Error> {
    log::info!("Got a connect notification: {}", v);
    Ok(())
}

async fn on_htlc_accepted(
    _p: Plugin<()>,
    _v: serde_json::Value,
) -> Result<serde_json::Value, Error> {
    log::info!("on_htlc_accepted called");
    thread::sleep(Duration::from_secs(20));
    Ok(json!({"result": "continue"}))
}
