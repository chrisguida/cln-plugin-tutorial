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
        .rpcmethod("testmethod_argument", "This is a test", testmethod_argument)
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

async fn testoptions(p: Plugin<()>, _v: serde_json::Value) -> Result<serde_json::Value, Error> {
    Ok(json!({
        "opt-option": format!("{:?}", p.option("opt-option").unwrap())
    }))
}

async fn testmethod(_p: Plugin<()>, _v: serde_json::Value) -> Result<serde_json::Value, Error> {
    Ok(json!("Hello"))
}

async fn testmethod_argument(p: Plugin<()>, v: serde_json::Value) -> Result<serde_json::Value, Error> {
  Ok(json!(format!("Hello, {}", v)))
}

async fn testmethod_option(p: Plugin<()>, _v: serde_json::Value) -> Result<serde_json::Value, Error> {
    Ok(json!(format!("Hello, {:?}", p.option("name").unwrap())))
}

async fn connect_handler(_p: Plugin<()>, v: serde_json::Value) -> Result<(), Error> {
    log::info!("Got a connect notification: {}", v);
    Ok(())
}

async fn peer_connected_handler(
    _p: Plugin<()>,
    v: serde_json::Value,
) -> Result<serde_json::Value, Error> {
    log::info!("Got a connect hook call: {}", v);
    Ok(json!({"result": "continue"}))
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

async fn on_htlc_accepted(
    _p: Plugin<()>,
    _v: serde_json::Value,
) -> Result<serde_json::Value, Error> {
    log::info!("on_htlc_accepted called");
    thread::sleep(Duration::from_secs(20));
    Ok(json!({"result": "continue"}))
}
