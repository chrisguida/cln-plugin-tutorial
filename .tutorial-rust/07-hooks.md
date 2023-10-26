# Hooks

- Synchronous event handler that blocks CLN from processing until it responds
- Allows plugins to be notified about internal events in lightningd and alter
  its behavior or inject custom behaviors.
- Hooks are considered to be an advanced feature due to the fact that lightningd
  relies on the plugin to tell it what to do next.
- Use them carefully, and make sure your plugins always return a valid response
  to any hook invocation.
- Can execute in either:
  - “single” mode
  - “chain” mode

For a list of all possible hooks, see
[the Hooks section of the docs](https://docs.corelightning.org/docs/hooks)

## Hooks in single mode

- Most hooks are currently set to single-mode.
- Unless otherwise stated all hooks are single-mode.
- In this mode only a single plugin can register the hook, and that plugin will
  get called for each event of that type.
- If a second plugin attempts to register the hook it gets killed and a
  corresponding log entry will be added to the logs.

## Hooks in chain mode

- When hooks are registered, they can optionally specify “before” and “after”
  arrays of plugin names, which control what order they will be called in.
- If a plugin name is unknown, it is ignored, otherwise if the hook calls cannot
  be ordered to satisfy the specifications of all plugin hooks, the plugin
  registration will fail.
- Multiple plugins can register for the hook type and they are called in any
  order they are loaded (i.e. cmdline order first, configuration order file
  second: though note that the order of plugin directories is
  implementation-dependent), overridden only by before and after requirements
  the plugin’s hook registrations specify.
- Each plugin can then handle the event or defer by returning a continue result
  like the following:

```json
{
  "result": "continue"
}
```

- The remainder of the response is ignored and if there are any more plugins
  that have registered the hook the next one gets called.
- If there are no more plugins then the internal handling is resumed as if no
  hook had been called.
- Any other result returned by a plugin is considered an exit from the chain.
- Upon exit no more plugin hooks are called for the current event, and the
  result is executed.

## Differences between event stream notifications and hooks

Hooks and notifications are very similar, however there are a few key
differences:

- **Notifications** are _asynchronous_
  - `lightningd` will send the notifications but not wait for the plugin to
    process them.
- **Hooks**, on the other hand, are _synchronous_
  - `lightningd` cannot finish processing the event until the plugin has
    returned.
- For notifications, any number of plugins can subscribe to a notification topic
  and get notified in parallel
- For hooks, only one plugin may register for single-mode hook types
- And, for both single-mode and chain-mode, only one plugin may return a
  non-continue response
  - This avoids having multiple contradictory responses.

# Exercise: Add a single-mode hook to your plugin

Add this code to your `main.rs` file (hint: insert this before `.dynamic()`):

```rust
.subscribe("connect", connect_handler)
```

This registers the plugin to listen for `htlc_accepted` hooks and respond with a
handler, named `on_htlc_accepted`.

Now add the `on_htlc_accepted` handler function (this should go after your rpc
handler, function, at the bottom of the file):

```rust
async fn on_htlc_accepted(
    _p: Plugin<()>,
    _v: serde_json::Value,
) -> Result<serde_json::Value, Error> {
    log::info!("on_htlc_accepted called");
    thread::sleep(Duration::from_secs(20));
    Ok(json!({"result": "continue"}))
}
```

Also make sure to import these modules at the top of your main.rs file (just
above `use tokio` line is a good spot):

```rust
use std::{thread, time::Duration};
```

Now let's test! Go to the shell and rebuild:

```sh
cargo build
```

Now start your plugin, this time on Node 2:

```sh
l2-cli plugin start $(pwd)/target/debug/rust-plugin
```

Run this command to create a channel between your nodes:

```sh
fund_nodes
```

Create an invoice for Node 1 to pay Node 2 50k sats:

```sh
l2-cli invoice 50000sat test1 "this is the first test"
```

Now copy the resulting bolt11 invoice and paste it into this command:

```sh
l1-cli pay <bolt11>
```

If all went well, the command should hang for 20 seconds while the hook runs.
Check Node 2's logs (`l2-log`) for the message `on_htlc_accepted called` to
verify that your hook was called.

- [ ] Add a hook
- [ ] Test that it works as expected
