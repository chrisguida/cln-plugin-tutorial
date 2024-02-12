# Plugin Life Cycle

## 1) Plugin launches
- Plugin is started by CLN in one of two ways: at CLN startup, or dynamically once CLN is already running.
  - At startup:
    - Via a command line option: `lightningd --plugin=/path/to/plugin1 --plugin=path/to/plugin2`
    - Via an option in the config file: `plugin=/path/to/plugin`
  - Dynamically:
    - Via `lightning-cli plugin start /path/to/plugin [options]`

## 2) The `getmanifest` method is called
- CLN calls plugin’s `getmanifest` JSON-RPC method
- Plugin responds with a manifest where everything the plugin needs is registered with CLN, including options, hooks, rpc methods, and notifications.
- During startup, a plugin must respond within 60 seconds or it is killed.

The plugin's response will look something like this:
```json
{
  "options": [
    {
      "name": "greeting",
      "type": "string",
      "default": "World",
      "description": "What name should I call you?",
      "deprecated": false
    }
  ],
  "rpcmethods": [
    {
      "name": "hello",
      "usage": "[name]",
      "description": "Returns a personalized greeting for {greeting} (set via options)."
    },
    {
      "name": "gettime",
      "usage": "",
      "description": "Returns the current time in {timezone}",
      "long_description": "Returns the current time in the timezone that is given as the only parameter.\nThis description may be quite long and is allowed to span multiple lines.",
      "deprecated": false
    }
  ],
  "subscriptions": [
    "connect",
    "disconnect"
  ],
  "hooks": [
    { "name": "openchannel", "before": ["another_plugin"] },
    { "name": "htlc_accepted" }
  ],
  "featurebits": {
    "node": "D0000000",
    "channel": "D0000000",
    "init": "0E000000",
    "invoice": "00AD0000"
  },
  "notifications": [
    {
	  "method": "mycustomnotification"
	}
  ],
  "nonnumericids": true,
  "dynamic": true
}
```

## 3) The `init` method is called
- CLN parses plugin options and passes them back by calling the plugin’s `init` method. This is also the signal that `lightningd`’s JSON-RPC over Unix Socket is now up and ready to receive incoming requests from the plugin.
- The request will look something like this:
```json
{
  "options": {
    "greeting": "World",
	"number": [0]
  },
  "configuration": {
    "lightning-dir": "/home/user/.lightning/testnet",
    "rpc-file": "lightning-rpc",
    "startup": true,
    "network": "testnet",
    "feature_set": {
        "init": "02aaa2",
        "node": "8000000002aaa2",
        "channel": "",
        "invoice": "028200"
    },
    "proxy": {
        "type": "ipv4",
        "address": "127.0.0.1",
        "port": 9050
    },
    "torv3-enabled": true,
    "always_use_proxy": false
  }
}
```
- The plugin sends a success response.
  - The plugin can optionally send a response at this point indicating that it’s disabled.
- This response must occur within a certain timeout:
  - During startup, a plugin must respond within 60 seconds or it is killed.
  - Plugins launched dynamically must respond to both getmanifest and init within 60 seconds or they are killed.

## 4) The plugin is now running normally
Now your plugin can do stuff!

# Exercise: Build a bare minimum plugin that conforms to the above requirements
If you click on [`rust-plugin/src/main.rs`](/home/runner/$REPL_SLUG/rust-plugin/src/main.rs) in the file tree to your left, you will see a basic CLN plugin written in Rust. It doesn't do much, yet. It pretty much just runs. We'll add more to it as we work through this tutorial.

Here it is:
```rust
//! This is a test plugin used to verify that we can compile and run
//! plugins using the Rust API against Core Lightning.
#[macro_use]
extern crate serde_json;
use anyhow::anyhow;
use cln_plugin::{options, Builder, Error, Plugin};
use tokio;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let state = ();

    if let Some(plugin) = Builder::new(tokio::io::stdin(), tokio::io::stdout())
        .dynamic()
        .start(state)
        .await?
    {
        plugin.join().await
    } else {
        Ok(())
    }
}
```

To test, open a shell and enter the following commands:

```
mkdir -p ~/.bitcoin
source startup_regtest.sh
start_ln

# This will launch `bitcoind` in regtest mode and attach two LN nodes to it.

# Enter the above three commands in the default replit directory (~/$REPL_SLUG-Rust-version) whenever reloading replit. This can happen randomly sometimes, or after a timeout, or if you close the repl then return.

l1-cli getinfo

# ...to check that your node is running

# Now build your plugin:

cd rust-plugin
cargo build

# When the build finishes, start your plugin:

l1-cli plugin start $(pwd)/target/debug/rust-plugin

# This will launch your plugin on Node #1. You should see a list of running plugins with your rust plugin as the last entry in the list.

l1-log

# will display Node #1's logfile. If you scroll to the bottom of the pager (shift + G), you should see a message from plugin-manager saying your plugin was started.
```

Press `q` to return to the shell.

- [ ] Launch a Rust plugin with a message on init success, and find success message in the log.
